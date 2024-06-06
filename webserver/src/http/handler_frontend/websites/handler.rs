//! The handler for managing websites

use axum::extract::Path;
use axum::Json;
use futures_util::TryStreamExt;
use rorm::and;
use rorm::fields;
use rorm::insert;
use rorm::prelude::ForeignModelByField;
use rorm::query;
use rorm::update;
use rorm::FieldAccess;
use rorm::Model;
use swaggapi::delete;
use swaggapi::get;
use swaggapi::post;
use swaggapi::put;
use tracing::instrument;
use uuid::Uuid;

use crate::global::GLOBAL;
use crate::http::common::errors::ApiError;
use crate::http::common::errors::ApiResult;
use crate::http::common::errors::FormResult;
use crate::http::common::schemas::FormError;
use crate::http::common::schemas::UuidSchema;
use crate::http::extractors::session_user::SessionUser;
use crate::http::handler_frontend::websites::schema::AddDomainToWebsiteForm;
use crate::http::handler_frontend::websites::schema::AddDomainToWebsiteRequest;
use crate::http::handler_frontend::websites::schema::CreateWebsiteRequest;
use crate::http::handler_frontend::websites::schema::DeployState;
use crate::http::handler_frontend::websites::schema::DomainField;
use crate::http::handler_frontend::websites::schema::FullWebsite;
use crate::http::handler_frontend::websites::schema::FullWebsiteDomain;
use crate::http::handler_frontend::websites::schema::ListWebsites;
use crate::http::handler_frontend::websites::schema::RemoveDomainPath;
use crate::http::handler_frontend::websites::schema::SimpleWebsite;
use crate::http::handler_frontend::websites::schema::UpdateWebsiteRequest;
use crate::models::website::Website;
use crate::models::website::WebsiteDomain;
use crate::utils::schemars::SchemaDateTime;

/// Create a new website
#[post("/websites")]
#[instrument(ret, err)]
pub async fn create_website(
    SessionUser(user): SessionUser,
    Json(CreateWebsiteRequest { name }): Json<CreateWebsiteRequest>,
) -> ApiResult<Json<UuidSchema>> {
    let mut tx = GLOBAL.db.start_transaction().await?;

    let uuid = Website::create_website(name, user.uuid, &mut tx).await?;

    tx.commit().await?;

    Ok(Json(UuidSchema { uuid }))
}

/// Retrieve a single website
#[get("/websites/:uuid")]
#[instrument(ret, err)]
pub async fn get_website(
    SessionUser(user): SessionUser,
    Path(UuidSchema { uuid }): Path<UuidSchema>,
) -> ApiResult<Json<FullWebsite>> {
    let mut tx = GLOBAL.db.start_transaction().await?;

    let mut website = query!(&mut tx, Website)
        .condition(Website::F.uuid.equals(uuid))
        .optional()
        .await?
        .ok_or(ApiError::ResourceNotFound)?;

    if *website.owner.key() != user.uuid {
        return Err(ApiError::MissingPrivileges);
    }

    Website::F.domains.populate(&mut tx, &mut website).await?;

    tx.commit().await?;

    // Unwrap is okay as the populate call above fills the cached field
    #[allow(clippy::unwrap_used)]
    Ok(Json(FullWebsite {
        uuid,
        name: website.name,
        domains: website
            .domains
            .cached
            .unwrap()
            .into_iter()
            .map(|x| FullWebsiteDomain {
                uuid: x.uuid,
                domain: x.domain,
            })
            .collect(),
        created_at: SchemaDateTime(website.created_at),
        deploy_state: website.deploy_state.into_inner(),
        last_deployment: website.last_deployment.map(SchemaDateTime),
    }))
}

/// Retrieve all websites owned by this user
#[get("/websites")]
#[instrument(ret, err)]
pub async fn get_all_websites(SessionUser(user): SessionUser) -> ApiResult<Json<ListWebsites>> {
    let mut tx = GLOBAL.db.start_transaction().await?;

    let websites = query!(&mut tx, Website)
        .condition(Website::F.owner.equals(user.uuid))
        .stream()
        .map_ok(|x| SimpleWebsite {
            uuid: x.uuid,
            name: x.name,
            created_at: SchemaDateTime(x.created_at),
            deploy_state: x.deploy_state.into_inner(),
            last_deployment: x.last_deployment.map(SchemaDateTime),
        })
        .try_collect()
        .await?;

    tx.commit().await?;

    Ok(Json(ListWebsites { websites }))
}

/// Update a website
#[put("/websites/:uuid")]
#[instrument(ret, err)]
pub async fn update_website(
    SessionUser(user): SessionUser,
    Path(UuidSchema { uuid }): Path<UuidSchema>,
    Json(UpdateWebsiteRequest { name }): Json<UpdateWebsiteRequest>,
) -> ApiResult<()> {
    let mut tx = GLOBAL.db.start_transaction().await?;

    let website = query!(&mut tx, Website)
        .condition(Website::F.uuid.equals(uuid))
        .optional()
        .await?
        .ok_or(ApiError::ResourceNotFound)?;

    if user.uuid == *website.owner.key() {
        return Err(ApiError::MissingPrivileges);
    }

    update!(&mut tx, Website)
        .condition(Website::F.uuid.equals(uuid))
        .set(Website::F.name, name)
        .exec()
        .await?;

    tx.commit().await?;

    Ok(())
}

/// Add a domain to a website
#[post("/websites/:uuid/domains")]
#[instrument(ret, err)]
pub async fn add_domain_to_website(
    SessionUser(user): SessionUser,
    Path(UuidSchema { uuid: website_uuid }): Path<UuidSchema>,
    Json(AddDomainToWebsiteRequest { domain }): Json<AddDomainToWebsiteRequest>,
) -> ApiResult<FormResult<Json<UuidSchema>, AddDomainToWebsiteForm>> {
    let mut tx = GLOBAL.db.start_transaction().await?;

    let website = query!(&mut tx, Website)
        .condition(Website::F.uuid.equals(website_uuid))
        .optional()
        .await?
        .ok_or(ApiError::ResourceNotFound)?;

    if user.uuid != *website.owner.key() {
        return Err(ApiError::MissingPrivileges);
    }

    let exists = query!(&mut tx, WebsiteDomain)
        .condition(WebsiteDomain::F.domain.equals(&domain))
        .optional()
        .await?
        .is_some();

    if exists {
        return Ok(Err(FormError::single(AddDomainToWebsiteForm::Domain(
            DomainField::AlreadyRegistered,
        ))));
    }

    let uuid = insert!(&mut tx, WebsiteDomain)
        .return_primary_key()
        .single(&WebsiteDomain {
            uuid: Uuid::new_v4(),
            domain,
            website: ForeignModelByField::Key(website_uuid),
        })
        .await?;

    update!(&mut tx, Website)
        .condition(Website::F.uuid.equals(website_uuid))
        .set(
            Website::F.deploy_state,
            fields::types::Json(DeployState::PendingChanges),
        )
        .exec()
        .await?;

    tx.commit().await?;

    Ok(Ok(Json(UuidSchema { uuid })))
}

/// Remove a domain from a website
#[delete("/websites/:website_uuid/domains/:domain_uuid")]
#[instrument(ret, err)]
pub async fn remove_domain_from_website(
    SessionUser(user): SessionUser,
    Path(RemoveDomainPath {
        website_uuid,
        domain_uuid,
    }): Path<RemoveDomainPath>,
) -> ApiResult<()> {
    let mut tx = GLOBAL.db.start_transaction().await?;

    let website = query!(&mut tx, Website)
        .condition(Website::F.uuid.equals(website_uuid))
        .optional()
        .await?
        .ok_or(ApiError::ResourceNotFound)?;

    if user.uuid != *website.owner.key() {
        return Err(ApiError::MissingPrivileges);
    }

    let exists = query!(&mut tx, WebsiteDomain)
        .condition(and!(
            WebsiteDomain::F.uuid.equals(domain_uuid),
            WebsiteDomain::F.website.equals(website_uuid)
        ))
        .optional()
        .await?
        .is_some();

    if !exists {
        return Err(ApiError::ResourceNotFound);
    }

    rorm::delete!(&mut tx, WebsiteDomain)
        .condition(WebsiteDomain::F.uuid.equals(domain_uuid))
        .await?;

    update!(&mut tx, Website)
        .condition(Website::F.uuid.equals(website_uuid))
        .set(
            Website::F.deploy_state,
            fields::types::Json(DeployState::PendingChanges),
        )
        .exec()
        .await?;

    tx.commit().await?;

    Ok(())
}

/// Delete a website
#[delete("/websites/:uuid")]
#[instrument(ret, err)]
pub async fn delete_website(
    SessionUser(user): SessionUser,
    Path(UuidSchema { uuid }): Path<UuidSchema>,
) -> ApiResult<()> {
    let mut tx = GLOBAL.db.start_transaction().await?;

    let website = query!(&mut tx, Website)
        .condition(Website::F.uuid.equals(uuid))
        .optional()
        .await?
        .ok_or(ApiError::ResourceNotFound)?;

    if user.uuid == *website.owner.key() {
        return Err(ApiError::MissingPrivileges);
    }

    rorm::delete!(&mut tx, Website)
        .condition(Website::F.uuid.equals(uuid))
        .await?;

    tx.commit().await?;

    // TODO: Issue removal of website

    Ok(())
}

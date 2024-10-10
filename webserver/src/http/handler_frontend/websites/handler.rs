//! The handler for managing websites

use std::time::Duration;

use axum::extract::Path;
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
use tower_sessions_rorm_store::tower_sessions::Session;
use tracing::debug;
use tracing::error;
use tracing::info;
use tracing::info_span;
use tracing::instrument;
use tracing::Instrument;
use uuid::Uuid;

use crate::global::webconf_updater::WebconfChanges;
use crate::global::webconf_updater::WebconfUpdateResult;
use crate::global::GLOBAL;
use crate::http::common::errors::ApiError;
use crate::http::common::errors::ApiResult;
use crate::http::common::schemas::FormResult;
use crate::http::common::schemas::SingleUuid;
use crate::http::extractors::api_json::ApiJson;
use crate::http::extractors::session_user::SessionUser;
use crate::http::handler_frontend::user_invites::schema::UserRoleWithClub;
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
use crate::http::handler_frontend::ws::schema::DnsQueryResult;
use crate::http::handler_frontend::ws::schema::WsServerMsg;
use crate::models::website::Website;
use crate::models::website::WebsiteDomain;
use crate::models::Club;
use crate::models::User;
use crate::utils::schemars::SchemaDateTime;

/// Create a new website
#[post("/")]
#[instrument(ret, err)]
pub async fn create_website(
    SessionUser { user, .. }: SessionUser,
    ApiJson(CreateWebsiteRequest { name }): ApiJson<CreateWebsiteRequest>,
) -> ApiResult<ApiJson<SingleUuid>> {
    let user = user.0;
    let mut tx = GLOBAL.db.start_transaction().await?;

    let uuid = Website::create_website(name, user.uuid, &mut tx).await?;

    update!(&mut tx, User)
        .condition(User::F.uuid.equals(user.uuid))
        .set(User::F.website_count, user.website_count + 1)
        .exec()
        .await?;

    tx.commit().await?;

    Ok(ApiJson(SingleUuid { uuid }))
}

/// Retrieve a single website
#[get("/:uuid")]
#[instrument(ret, err)]
pub async fn get_website(
    SessionUser { user, .. }: SessionUser,
    Path(SingleUuid { uuid }): Path<SingleUuid>,
) -> ApiResult<ApiJson<FullWebsite>> {
    let user = user.0;
    let mut tx = GLOBAL.db.start_transaction().await?;

    let mut website = query!(&mut tx, Website)
        .condition(Website::F.uuid.equals(uuid))
        .optional()
        .await?
        .ok_or(ApiError::BadRequest)?;

    if *website.owner.key() != user.uuid {
        return Err(ApiError::MissingPrivileges);
    }

    Website::F.domains.populate(&mut tx, &mut website).await?;

    tx.commit().await?;

    // Unwrap is okay as the populate call above fills the cached field
    #[allow(clippy::unwrap_used)]
    Ok(ApiJson(FullWebsite {
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
#[get("/")]
#[instrument(ret, err)]
pub async fn get_all_websites(
    SessionUser { user, .. }: SessionUser,
) -> ApiResult<ApiJson<ListWebsites>> {
    let user = user.0;
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

    Ok(ApiJson(ListWebsites { websites }))
}

/// Update a website
#[put("/:uuid")]
#[instrument(ret, err)]
pub async fn update_website(
    SessionUser { user, .. }: SessionUser,
    Path(SingleUuid { uuid }): Path<SingleUuid>,
    ApiJson(UpdateWebsiteRequest { name }): ApiJson<UpdateWebsiteRequest>,
) -> ApiResult<()> {
    let user = user.0;
    let mut tx = GLOBAL.db.start_transaction().await?;

    let website = query!(&mut tx, Website)
        .condition(Website::F.uuid.equals(uuid))
        .optional()
        .await?
        .ok_or(ApiError::BadRequest)?;

    if user.uuid != *website.owner.key() {
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
#[post("/:uuid/domains")]
#[instrument(ret, err)]
pub async fn add_domain_to_website(
    SessionUser { user, .. }: SessionUser,
    Path(SingleUuid { uuid: website_uuid }): Path<SingleUuid>,
    ApiJson(AddDomainToWebsiteRequest { domain }): ApiJson<AddDomainToWebsiteRequest>,
) -> ApiResult<ApiJson<FormResult<SingleUuid, AddDomainToWebsiteForm>>> {
    let user = user.0;
    let mut tx = GLOBAL.db.start_transaction().await?;

    let website = query!(&mut tx, Website)
        .condition(Website::F.uuid.equals(website_uuid))
        .optional()
        .await?
        .ok_or(ApiError::BadRequest)?;

    if user.uuid != *website.owner.key() {
        return Err(ApiError::MissingPrivileges);
    }

    let exists = query!(&mut tx, WebsiteDomain)
        .condition(WebsiteDomain::F.domain.equals(&domain))
        .optional()
        .await?
        .is_some();

    if exists {
        return Ok(ApiJson(FormResult::err(AddDomainToWebsiteForm::Domain(
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

    Ok(ApiJson(FormResult::ok(SingleUuid { uuid })))
}

/// Remove a domain from a website
#[delete("/:website_uuid/domains/:domain_uuid")]
#[instrument(ret, err)]
pub async fn remove_domain_from_website(
    SessionUser { user, role }: SessionUser,
    Path(RemoveDomainPath {
        website_uuid,
        domain_uuid,
    }): Path<RemoveDomainPath>,
) -> ApiResult<()> {
    let user = user.0;
    let club = if let UserRoleWithClub::User { club } = role {
        club
    } else {
        return Err(ApiError::new_internal_server_error("received invalid role"));
    };
    let mut tx = GLOBAL.db.start_transaction().await?;

    let website = query!(&mut tx, Website)
        .condition(Website::F.uuid.equals(website_uuid))
        .optional()
        .await?
        .ok_or(ApiError::BadRequest)?;

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
        return Err(ApiError::BadRequest);
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

    let website_count = query!(&mut tx, (Club::F.website_count,))
        .condition(Club::F.uuid.equals(club))
        .one()
        .await?
        .0;

    update!(&mut tx, Club)
        .condition(Club::F.uuid.equals(club))
        .set(Club::F.website_count, website_count + 1)
        .await?;

    tx.commit().await?;

    Ok(())
}

/// Delete a website
#[delete("/:uuid")]
#[instrument(ret, err)]
pub async fn delete_website(
    SessionUser { user, role }: SessionUser,
    Path(SingleUuid { uuid }): Path<SingleUuid>,
) -> ApiResult<()> {
    let user = user.0;
    let club = if let UserRoleWithClub::User { club } = role {
        club
    } else {
        return Err(ApiError::new_internal_server_error("received invalid role"));
    };
    let mut tx = GLOBAL.db.start_transaction().await?;

    let website = query!(&mut tx, Website)
        .condition(Website::F.uuid.equals(uuid))
        .optional()
        .await?
        .ok_or(ApiError::BadRequest)?;

    if user.uuid != *website.owner.key() {
        return Err(ApiError::MissingPrivileges);
    }

    rorm::delete!(&mut tx, Website)
        .condition(Website::F.uuid.equals(uuid))
        .await?;

    update!(&mut tx, User)
        .condition(User::F.uuid.equals(user.uuid))
        .set(User::F.website_count, user.website_count - 1)
        .exec()
        .await?;

    let website_count = query!(&mut tx, (Club::F.website_count,))
        .condition(Club::F.uuid.equals(club))
        .one()
        .await?
        .0;

    update!(&mut tx, Club)
        .condition(Club::F.uuid.equals(club))
        .set(Club::F.website_count, website_count - 1)
        .await?;

    tx.commit().await?;

    // TODO: Issue removal of website

    Ok(())
}

/// Deploy the configuration to the webserver.
///
/// This will configure the webspace and request certificates for all added domains
///
/// Returns an uuid that will be used to send a notification via websocket when the deployment
/// process has finished
#[post("/:uuid/deploy")]
#[instrument(err)]
pub async fn deploy_website(
    SessionUser { user, .. }: SessionUser,
    Path(SingleUuid { uuid }): Path<SingleUuid>,
) -> ApiResult<ApiJson<SingleUuid>> {
    let user = user.0;
    let mut tx = GLOBAL.db.start_transaction().await?;

    let mut website = query!(&mut tx, Website)
        .condition(Website::F.uuid.equals(uuid))
        .optional()
        .await?
        .ok_or(ApiError::BadRequest)?;

    if user.uuid != *website.owner.key() {
        return Err(ApiError::MissingPrivileges);
    }

    Website::F.domains.populate(&mut tx, &mut website).await?;

    // Unwrap okay as we populated the domains above
    #[allow(clippy::unwrap_used)]
    let domains = website
        .domains
        .cached
        .unwrap()
        .into_iter()
        .map(|x| x.domain)
        .collect();

    tx.commit().await?;

    let uuid = Uuid::new_v4();

    tokio::spawn(
        async move {
            let res = GLOBAL
                .webconf_updater
                .apply_changes(WebconfChanges {
                    user: user.uuid,
                    website: website.uuid,
                    domains,
                })
                .await;

            let success = match res {
                Ok(state) => match state {
                    WebconfUpdateResult::Success => true,
                    WebconfUpdateResult::Fail => false,
                },
                Err(err) => {
                    error!("webconf update failed: {err}");
                    false
                }
            };

            let deploy_state = if success {
                DeployState::Deployed
            } else {
                DeployState::DeploymentFailed
            };
            if let Err(err) = update!(&GLOBAL.db, Website)
                .condition(Website::F.uuid.equals(website.uuid))
                .set(Website::F.deploy_state, fields::types::Json(deploy_state))
                .exec()
                .await
            {
                error!("Database error: {err}");
                return;
            }

            // Send update to client
            GLOBAL
                .ws
                .send_to_user(
                    user.uuid,
                    WsServerMsg::DeployUpdate {
                        task: uuid,
                        state: if success {
                            WebconfUpdateResult::Success
                        } else {
                            WebconfUpdateResult::Fail
                        },
                    },
                )
                .await;
        }
        .instrument(info_span!("webconf-update")),
    );

    Ok(ApiJson(SingleUuid { uuid }))
}

#[post("/:uuid/check-dns")]
#[instrument]
pub async fn check_dns(
    Path(uuid): Path<Uuid>,
    SessionUser { user, .. }: SessionUser,
    session: Session,
) -> ApiResult<ApiJson<SingleUuid>> {
    let user = user.0;
    let mut tx = GLOBAL.db.start_transaction().await?;

    let mut website = query!(&mut tx, Website)
        .condition(Website::F.uuid.equals(uuid))
        .optional()
        .await?
        .ok_or(ApiError::BadRequest)?;

    if *website.owner.key() != user.uuid {
        return Err(ApiError::Unauthenticated);
    }

    Website::F.domains.populate(&mut tx, &mut website).await?;

    tx.commit().await?;

    // Unwrap is okay as backref was populated above
    #[allow(clippy::unwrap_used)]
    let domains: Vec<_> = website.domains.cached.unwrap();

    let session_id = session.id().unwrap();

    let task = Uuid::new_v4();
    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_secs(1)).await;

        info!("Start resolving domains:");

        for domain in domains {
            let result = GLOBAL.dns.resolve(&domain.domain).await.unwrap();

            debug!("Resolved {domain}: {result:?}", domain = domain.domain);

            GLOBAL
                .ws
                .send_to_session(
                    user.uuid,
                    session_id,
                    WsServerMsg::DnsUpdate {
                        task,
                        result: DnsQueryResult {
                            uuid: domain.uuid,
                            result,
                        },
                    },
                )
                .await;
        }

        GLOBAL
            .ws
            .send_to_session(user.uuid, session_id, WsServerMsg::DnsFinished { task })
            .await;
    });

    Ok(ApiJson(SingleUuid { uuid: task }))
}

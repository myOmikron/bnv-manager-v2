/*
pub(crate) async fn ensure_existing_website(
    website: Uuid,
    test_certificate: bool,
    website_owner: Uuid,
    exe: impl Executor<'_>,
) -> Result<Website, rorm::Error> {
    let mut guard = exe.ensure_transaction().await?;
    return if let Some(existing_website) = query!(guard.get_transaction(), Website)
        .condition(Website::F.uuid.equals(website))
        .optional()
        .await?
    {
        for d in &existing_website.domains.cached {
            info!("Domain {:#?}", d);
        }
        if test_certificate != existing_website.test_cert {}
        guard.commit().await?;
        Ok(existing_website)
    } else {
        let website = insert!(guard.get_transaction(), NewWebsite)
            .single(&NewWebsite {
                uuid: website.clone(),
                owner: ForeignModelByField::Key(website_owner),
                test_cert: test_certificate,
            })
            .await?;
        guard.commit().await?;
        Ok(website.into())
    };
}
 */

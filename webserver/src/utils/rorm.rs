use rorm::conditions::Condition;
use rorm::crud::selector::Selector;
use rorm::db::Executor;
use tracing::instrument;

#[instrument(level = "debug", skip_all)]
pub async fn debug_assert_non_existence(
    executor: impl Executor<'_>,
    model: impl Selector,
    condition: impl Condition<'_>,
) {
    if cfg!(debug_assertions) {
        let result = rorm::query(executor, model)
            .condition(condition)
            .optional()
            .await;

        assert!(matches!(result, Ok(None)));
    }
}

#[instrument(level = "debug", skip_all)]
pub async fn debug_assert_existence(
    executor: impl Executor<'_>,
    model: impl Selector,
    condition: impl Condition<'_>,
) {
    if cfg!(debug_assertions) {
        let result = rorm::query(executor, model)
            .condition(condition)
            .optional()
            .await;

        assert!(matches!(result, Ok(Some(_))));
    }
}

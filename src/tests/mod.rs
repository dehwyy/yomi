use super::core::animation::{Animation, Process};

#[tokio::test]
async fn invoke_sequentially() {
    let mut animation = Animation::builder();
    animation
        .add(
            Process::new(async {
                tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
            })
            .set_text_during_execution("sleeping..."),
        )
        .invoke_sequentially("sleep done!".to_string())
        .await;
}

#[tokio::test]
async fn invoke_concurrently() {
    let mut animation = Animation::builder();
    animation
        .add(
            Process::new(async {
                tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
            })
            .set_text_during_execution("sleeping 1..."),
        )
        .add(
            Process::new(async {
                tokio::time::sleep(std::time::Duration::from_millis(2000)).await;
            })
            .set_text_during_execution("sleeping 2..."),
        )
        .invoke_parallel()
        .await;
}

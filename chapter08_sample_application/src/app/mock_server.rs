use chapter08_sample_application::infrastructure::{users_api, MockApi, MockContext};

#[tokio::main]
async fn main() {
    let ctx = MockContext::default();
    let api = MockApi::new(ctx);

    warp::serve(users_api(api))
        .run(([127, 0, 0, 1], 8080))
        .await;
}

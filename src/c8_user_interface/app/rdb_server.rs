use ddd::c8_user_interface::infrastructure::{users_api, RDBApi, RDBContext};

#[tokio::main]
async fn main() {
    let ctx = RDBContext::default();
    let api = RDBApi::new(ctx);

    warp::serve(users_api(api))
        .run(([127, 0, 0, 1], 8080))
        .await;
}

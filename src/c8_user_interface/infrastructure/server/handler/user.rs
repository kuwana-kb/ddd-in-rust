use warp::{Rejection, Reply};

use crate::c8_user_interface::{
    domain::UserId,
    usecase::{CreateUserCommand, HaveUserApplicationService, UserApplicationService},
};

pub async fn get_user_handler<T>(app: &T, id: UserId) -> Result<impl Reply, Rejection>
where
    T: HaveUserApplicationService,
{
    let service = app.provide_user_service();
    match service.get(id) {
        Err(_) => Err(warp::reject::not_found()),
        Ok(user) => Ok(warp::reply::json(&user)),
    }
}

pub async fn register_user_handler<T>(
    app: &T,
    cmd: CreateUserCommand,
) -> Result<impl Reply, Rejection>
where
    T: HaveUserApplicationService,
{
    let service = app.provide_user_service();
    match service.register(cmd) {
        Err(_) => Err(warp::reject::reject()),
        Ok(_) => Ok(warp::reply::with_status(
            "success",
            warp::http::StatusCode::OK,
        )),
    }
}

// trait として表現したかったが、traitのメソッドでは動的ディスパッチができないようだ
//
// error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
//   --> src/c8_user_interface/app/main.rs:27:50
//    |
// 27 |     async fn list_users_handler(&self) -> Result<impl Reply, Rejection> {
//
// use async_trait::async_trait;
// #[async_trait]
// pub trait Handlers: UserApplicationService {
//     async fn list_users_handler(&self) -> Result<impl Reply, Rejection> {
//         self.
//         Ok(warp::reply::json(&users))
//     }
// }

use warp::{Filter, Rejection, Reply};

use crate::c8_user_interface::{
    infrastructure::{get_user_handler, register_user_handler},
    usecase::HaveUserApplicationService,
};

// TODO: appを参照で渡せたほうがよいかも？
pub fn users_api<T>(app: T) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone
where
    T: HaveUserApplicationService + std::marker::Sync + std::marker::Send + Clone,
{
    get_user_by_name(app.clone()).or(register(app))
}

fn get_user_by_name<T>(app: T) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone
where
    T: std::marker::Sync + HaveUserApplicationService + std::marker::Send + Clone,
{
    users()
        .and(warp::get())
        .and(warp::body::json())
        .and_then(move |name| get_user_handler(app.clone(), name))
}

fn register<T>(app: T) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone
where
    T: std::marker::Sync + HaveUserApplicationService + std::marker::Send + Clone,
{
    users()
        .and(warp::put())
        .and(warp::body::json())
        .and_then(move |cmd| register_user_handler(app.clone(), cmd))
}

fn users() -> warp::filters::BoxedFilter<()> {
    warp::path("users").boxed()
}

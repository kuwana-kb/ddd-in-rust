use warp::{Filter, Rejection, Reply};

use crate::c8_user_interface::{
    infrastructure::{get_user_handler, register_user_handler},
    usecase::HaveUserApplicationService,
};

pub fn users_api<T>(app: &T) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone + '_
where
    T: HaveUserApplicationService + std::marker::Sync + std::marker::Send,
{
    get_user(app).or(register(app))
}

fn get_user<T>(app: &T) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone + '_
where
    T: std::marker::Sync + HaveUserApplicationService + std::marker::Send,
{
    users()
        .and(warp::get())
        .and(warp::body::json())
        .and_then(move |id| get_user_handler(app, id))
}

fn register<T>(app: &T) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone + '_
where
    T: std::marker::Sync + HaveUserApplicationService + std::marker::Send,
{
    users()
        .and(warp::put())
        .and(warp::body::json())
        .and_then(move |cmd| register_user_handler(app, cmd))
}

fn users() -> warp::filters::BoxedFilter<()> {
    warp::path("users").boxed()
}

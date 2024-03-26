use shuttle_axum::AxumService;
use shuttle_runtime::{CustomError, Error};

mod errors;
mod models;
mod router;
mod routes;
mod templates;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = router::init_router();

    Ok(router.into())
}

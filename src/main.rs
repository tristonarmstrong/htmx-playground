use shuttle_axum::AxumService;
use shuttle_runtime;

mod db_models;
mod errors;
mod router;
mod routes;
mod structs;
mod templates;
mod utils;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = router::init_router();

    Ok(router.into())
}

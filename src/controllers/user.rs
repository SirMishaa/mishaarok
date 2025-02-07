use crate::{models::_entities::users, views::user::CurrentResponse};
use axum::debug_handler;
use loco_rs::prelude::*;
use tracing::info;

#[debug_handler]
async fn current(auth: auth::JWT, State(ctx): State<AppContext>) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    info!("Current user: {:?}", user);
    format::json(CurrentResponse::new(&user))
}

pub fn routes() -> Routes {
    Routes::new().prefix("user").add("/current", get(current))
}

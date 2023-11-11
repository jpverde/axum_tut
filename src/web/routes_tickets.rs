use axum::extract::{Path, State};
use axum::routing::{delete, post};
use axum::{Json, Router};
use tracing::debug;

use crate::ctx::Ctx;
use crate::model::{ModelManager, Ticket, TicketForCreate};
use crate::Result;

pub fn routes(mc: ModelManager) -> Router {
    Router::new()
        .route("/tickets", post(create_ticket).get(list_ticket))
        .route("/tickets/:id", delete(delete_ticket))
        .with_state(mc)
}

// region:      --- REST Handlers
async fn create_ticket(
    State(mc): State<ModelManager>,
    ctx: Ctx,
    Json(ticket_fc): Json<TicketForCreate>,
) -> Result<Json<Ticket>> {
    debug!("{:<12} - create_ticket", "HANDLER");

    let ticket = mc.create_ticket(ctx, ticket_fc).await?;

    Ok(Json(ticket))
}

async fn list_ticket(
    State(mc): State<ModelManager>,
    ctx: Ctx,
) -> Result<Json<Vec<Ticket>>> {
    debug!("{:<12} - list_ticket", "HANDLER");

    let tickets = mc.list_tickets(ctx).await?;

    Ok(Json(tickets))
}

async fn delete_ticket(
    State(mc): State<ModelManager>,
    ctx: Ctx,
    Path(id): Path<u64>,
) -> Result<Json<Ticket>> {
    debug!("{:<12} - delete_ticket", "HANDLER");

    let ticket = mc.delete_ticket(ctx, id).await?;

    Ok(Json(ticket))
}
// endregion:   --- Rest Handlers

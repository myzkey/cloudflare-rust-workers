use crate::types::GenericResponse;
use worker::{Request, Response, RouteContext};

pub async fn handle_post(_: Request, _ctx: RouteContext<()>) -> worker::Result<Response> {
    Response::from_json(&GenericResponse::success("You reached a POST route!"))
}

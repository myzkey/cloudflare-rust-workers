use crate::handlers::{handle_delete, handle_get, handle_post};
use worker::Router;

pub fn configure_api_routes(router: Router<()>) -> Router<()> {
    router
        .get_async("/foo", handle_get)
        .post_async("/bar", handle_post)
        .delete_async("/baz", handle_delete)
    // 新しいルートを簡単に追加できます
    // .get_async("/health", handle_health)
    // .post_async("/users", handle_create_user)
}

// より細かく分けたい場合の例
// pub fn configure_user_routes(router: Router<()>) -> Router<()> {
//     router
//         .get_async("/users", handle_list_users)
//         .post_async("/users", handle_create_user)
//         .get_async("/users/:id", handle_get_user)
//         .put_async("/users/:id", handle_update_user)
//         .delete_async("/users/:id", handle_delete_user)
// }

// pub fn configure_health_routes(router: Router<()>) -> Router<()> {
//     router
//         .get_async("/health", handle_health)
//         .get_async("/health/ready", handle_readiness)
//         .get_async("/health/live", handle_liveness)
// }

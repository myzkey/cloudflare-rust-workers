use worker::*;

mod types;
mod handlers;
mod routes;

use routes::configure_api_routes;

#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    // Routerを作成し、各モジュールから設定を適用
    let router = Router::new();

    // APIルートを設定
    let router = configure_api_routes(router);

    // 将来的に他のルートグループも追加可能
    // let router = configure_user_routes(router);
    // let router = configure_health_routes(router);

    router.run(req, env).await
}

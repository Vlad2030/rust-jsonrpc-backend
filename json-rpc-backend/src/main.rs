use ntex::web;

use json_rpc_backend::core::data;
use json_rpc_backend::core::rpc;
use json_rpc_backend::handlers;
use json_rpc_backend::utils::envs;

#[ntex::main]
async fn main() -> std::io::Result<()> {
    let rpc_service: data::RpcService = data::RPC_SERVICE.clone();

    let ip: &str = &rpc_service.ip;
    let port: u16 = rpc_service.port.parse::<u16>().unwrap();
    let logging: String = rpc_service.logging;
    let workers: usize = rpc_service.workers.parse::<usize>().unwrap();

    envs::update_env("RUST_LOG", format!("ntex={}", logging));
    env_logger::init();

    web::HttpServer::new(|| {
        web::App::new()
            .wrap(web::middleware::Compress::default())
            .wrap(web::middleware::Logger::default())
            .default_service(
                ntex::web::route()
                    .guard(ntex::web::guard::Not(
                        ntex::web::guard::Any(ntex::web::guard::Get())
                            // .or(ntex::web::guard::Post())
                            .or(ntex::web::guard::Delete())
                            .or(ntex::web::guard::Put())
                            .or(ntex::web::guard::Patch()),
                    ))
                    .to(handlers::error::method_not_allowed),
            )
            .service(web::resource("/").route(web::post().to(rpc::handler)))
    })
    .bind((ip, port))?
    .workers(workers)
    .run()
    .await
}

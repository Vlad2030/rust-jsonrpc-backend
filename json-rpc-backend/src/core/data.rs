use crate::utils::envs;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref RPC_SERVICE: RpcService = RpcService::new();
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct RpcService {
    pub ip: String,
    pub port: String,
    pub logging: String,
    pub workers: String,
}

impl RpcService {
    pub fn new() -> Self {
        Self {
            ip: envs::get_env("RPC_SERVICE_IP").unwrap_or("0.0.0.0".into()),
            port: envs::get_env("RPC_SERVICE_PORT").unwrap_or("80".into()),
            logging: envs::get_env("RPC_SERVICE_LOGGING").unwrap_or("info".into()),
            workers: envs::get_env("RPC_SERVICE_WORKERS").unwrap_or("1".into()),
        }
    }

    pub fn backend_url(
        &self,
        schema: &'static str,
    ) -> String {
        return format!("{}://{}:{}", schema, self.ip, self.port);
    }
}

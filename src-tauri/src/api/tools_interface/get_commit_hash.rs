use std::vec;

use super::Response;
use crate::{app, ssh::ssh_api::*};
use log::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::task;

// let app_paths = vec![
//     "/opt/ivauto_ivs_server/ivauto_ivs_server",
//     "/opt/ivauto_ivs_server/ivauto_quality_detection",
//     "/opt/ivauto_ivs_server/ivauto_summary_server",
//     "/opt/StreamServer/StreamServer",
//     "/opt/LB_intercom/LB_intercom",
// ];

// let git_info_paths = vec![
//     "/opt/ivauto_ivs_server/ivs_ver.txt",
//     "/opt/ivauto_ivs_server/qd_ver.txt",
//     "/opt/StreamServer/git_commit_version.txt",
//     "/opt/LB_intercom/git_commit_version.txt",
//     "/opt/ivauto_ivs_server/model_ver.txt",
// ];

// let model_paths = vec![
//     "/opt/ivauto_ivs_server/data/prison-rt",
//     "/opt/ivauto_ivs_server/data/coeff-prison",
// ];

async fn get_commit_hash_l(host: &str, path: &str) -> Result<String, String> {
    let host_clone = host.to_string();
    let command = format!("sed -n '2p' {} | tr -d '\\n'", path);

    match task::spawn_blocking(move || exec_ssh_command(host_clone.as_str(), &command)).await {
        Ok(result) => result,
        Err(join_error) => Err(join_error.to_string()),
    }
}

#[tauri::command]
pub async fn get_commit_hash(host: &str, path: &str) -> Result<String, String> {
    match get_commit_hash_l(host, path).await {
        Ok(data) => {
            let response = Response {
                code: 0,
                message: "success".to_string(),
                data: Some(data),
            };
            serde_json::to_string(&response).map_err(|e| e.to_string())
        }
        Err(err) => {
            let response = Response::<String> {
                code: -1,
                message: err.clone(),
                data: None,
            };
            error!("get_commit_hash failed, err: {}", err);
            serde_json::to_string(&response).map_err(|e| e.to_string())
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::ssh::ssh_api::*;
    use dotenv::dotenv;
    use log4rs;
    use std::env;
    #[tokio::test]
    async fn test_net_info() {
        dotenv::from_path("../.env").ok();
        log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
        let host = env::var("VITE_HOST").unwrap();
        let port = env::var("VITE_PORT").unwrap();
        let user = env::var("VITE_USER").unwrap();
        let passwd = env::var("VITE_PASSWORD").unwrap();

        add_ssh_connect(
            format!("{}:{}", host, port).as_str(),
            user.as_str(),
            passwd.as_str(),
        );
        let result = get_commit_hash(
            format!("{}:{}", host, port,).as_str(),
            "/opt/ivauto_ivs_server/ivs_ver.txt",
        )
        .await;
        info!("{}", result.unwrap());
        // assert!(result.is_ok());
        disconnect_ssh(format!("{}:{}", host, port).as_str());
    }
}
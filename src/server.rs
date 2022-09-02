use rocket::serde::{json::Json, Serialize};
use std::collections::HashMap;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerStatusResponse<'r> {
    name: &'r str,
    server: &'r str,
    status: &'r str,
    population: &'r str,
}

#[get("/deploymentStatus")]
pub fn server_status() -> Json<HashMap<String, ServerStatusResponse<'static>>> {
    let mut servers = HashMap::new();
    servers.insert(
        "test".to_string(),
        ServerStatusResponse {
            name: "Worlds Adrift Reborn",
            server: "worlds-adrift-reborn",
            status: "up",
            population: "0",
        },
    );
    Json(servers)
}

use rocket::serde::{json::Json, Deserialize, Serialize};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SteamCredentials<'r> {
    platform_id: &'r str,
    secret: &'r str,
    user_key: &'r str,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AuthRequest<'r> {
    app_id: &'r str,
    steam_credential: SteamCredentials<'r>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthResponse<'r> {
    token: &'r str,
    player_id: &'r str,
    bossa_id: &'r str,
    screen_name: &'r str,
    desc: &'r str,
    success: bool,
}

#[post("/authenticate", data = "<_input>")]
pub fn login(_input: Json<AuthRequest<'_>>) -> Json<AuthResponse<'_>> {
    Json(AuthResponse {
        token: "549714499925361",
        player_id: "896004403083683",
        bossa_id: "1",
        screen_name: "Player",
        desc: "",
        success: true,
    })
}

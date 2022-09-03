use rocket::serde::{json::Json, Serialize};
use std::collections::HashMap;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CharacterListResponse<'r> {
    character_list: Vec<CharacterCreationData<'r>>,
    unlocked_slots: i32,
    has_main_character: bool,
    haven_finished: bool,
}

#[derive(Serialize)]
pub struct UnityColor {
    a: f32,
    b: f32,
    g: f32,
    r: f32,
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ColorProperties {
    primary_color: UnityColor,
    secondary_color: UnityColor,
    tertiary_color: UnityColor,
    spec_color: UnityColor,
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct CharacterUniversalColors {
    hair_color: UnityColor,
    skin_color: UnityColor,
    lip_color: UnityColor,
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ItemData<'r> {
    id: &'r str,
    prefab: &'r str,
    color_props: ColorProperties,
    health: f32,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CharacterCreationData<'r> {
    #[serde(rename = "Id")]
    id: i32,
    character_uid: &'r str,
    #[serde(rename = "Name")]
    name: &'r str,
    #[serde(rename = "Server")]
    server: &'r str,
    server_identifier: &'r str,
    #[serde(rename = "Cosmetics")]
    cosmetics: Option<HashMap<String, ItemData<'r>>>,
    #[serde(rename = "UniversalColors")]
    universal_colors: CharacterUniversalColors,
    is_male: bool,
    seen_intro: bool,
    skipped_tutorial: bool,
}

#[get("/characterList/<_>/steam/1234")]
pub fn character_list() -> Json<CharacterListResponse<'static>> {
    Json(CharacterListResponse {
        character_list: vec![CharacterCreationData {
            id: 0,
            character_uid: "ed9c1af7-00b7-4ecb-a3c4-3e2832f25c85",
            name: "Lea",
            server: "test",
            server_identifier: "test",
            cosmetics: Some(HashMap::new()),
            universal_colors: CharacterUniversalColors {
                hair_color: UnityColor {
                    a: 1f32,
                    b: 66f32 / 255f32,
                    g: 126f32 / 255f32,
                    r: 229f32 / 255f32,
                },
                skin_color: UnityColor {
                    a: 1f32,
                    b: 222f32 / 255f32,
                    g: 245f32 / 255f32,
                    r: 252f32 / 255f32,
                },
                lip_color: UnityColor {
                    a: 1f32,
                    b: 5f32 / 255f32,
                    g: 5f32 / 255f32,
                    r: 147f32 / 255f32,
                },
            },
            is_male: false,
            seen_intro: true,
            skipped_tutorial: true,
        }],
        unlocked_slots: 2,
        has_main_character: true,
        haven_finished: true,
    })
}

#[post("/reserveCharacterSlot/<_>/steam/1234")]
pub fn character_reserve() -> Json<CharacterListResponse<'static>> {
    Json(CharacterListResponse {
        character_list: vec![
            CharacterCreationData {
                id: 0,
                character_uid: "ed9c1af7-00b7-4ecb-a3c4-3e2832f25c85",
                name: "Lea",
                server: "test",
                server_identifier: "test",
                cosmetics: Some(HashMap::new()),
                universal_colors: CharacterUniversalColors {
                    hair_color: UnityColor {
                        a: 1f32,
                        b: 66f32 / 255f32,
                        g: 126f32 / 255f32,
                        r: 229f32 / 255f32,
                    },
                    skin_color: UnityColor {
                        a: 1f32,
                        b: 222f32 / 255f32,
                        g: 245f32 / 255f32,
                        r: 252f32 / 255f32,
                    },
                    lip_color: UnityColor {
                        a: 1f32,
                        b: 5f32 / 255f32,
                        g: 5f32 / 255f32,
                        r: 147f32 / 255f32,
                    },
                },
                is_male: false,
                seen_intro: true,
                skipped_tutorial: true,
            },
            CharacterCreationData {
                id: 1,
                character_uid: "49c5f41f-4597-443e-a57f-2111dc10334c",
                name: "",
                server: "test",
                server_identifier: "test",
                cosmetics: None,
                universal_colors: CharacterUniversalColors {
                    hair_color: UnityColor {
                        a: 1f32,
                        b: 66f32 / 255f32,
                        g: 126f32 / 255f32,
                        r: 229f32 / 255f32,
                    },
                    skin_color: UnityColor {
                        a: 1f32,
                        b: 222f32 / 255f32,
                        g: 245f32 / 255f32,
                        r: 252f32 / 255f32,
                    },
                    lip_color: UnityColor {
                        a: 1f32,
                        b: 5f32 / 255f32,
                        g: 5f32 / 255f32,
                        r: 147f32 / 255f32,
                    },
                },
                is_male: false,
                seen_intro: true,
                skipped_tutorial: true,
            },
        ],
        unlocked_slots: 2,
        has_main_character: true,
        haven_finished: true,
    })
}

//POST /player/reserveName

#[derive(Serialize)]
pub struct AuthorizeCharacterResponse<'r> {
    token: &'r str,
    success: bool,
}

#[get("/authorizeCharacter")]
pub fn authorize_character() -> Json<AuthorizeCharacterResponse<'static>> {
    Json(AuthorizeCharacterResponse {
        token: "test",
        success: true,
    })
}

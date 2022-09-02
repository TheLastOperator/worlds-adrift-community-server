#[macro_use]
extern crate rocket;

mod auth;
mod character;
mod server;

#[launch]
fn rocket() -> _ {
    let figment = rocket::Config::figment().merge(("port", 8080));
    rocket::custom(figment).mount(
        "/",
        routes![
            crate::auth::login,
            crate::server::server_status,
            crate::character::character_list,
            crate::character::character_reserve,
        ],
    )
}

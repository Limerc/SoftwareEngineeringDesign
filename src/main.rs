use proj::routes::askllm::*;
use proj::routes::get_history::*;
use proj::routes::jwt::*;
use proj::routes::set_token::*;
use rocket::{launch, routes};

#[launch]
fn rocket() -> _ {
    rocket::build().mount(
        "/",
        routes![
            protected_route,
            login,
            processed_stream,
            get_history,
            set_token_api
        ],
    )
}

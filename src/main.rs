use proj::llm::askQuestion::*;
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

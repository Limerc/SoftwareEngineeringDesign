use rocket::State;
use rocket::response::stream::{Event, EventStream};
use rocket::{get};
#[get("/events")]
pub fn events(ctxt: &State<bool>) -> EventStream![Event + '_] {
    EventStream! {
        // By using `ctxt` in the stream, the borrow is moved into it. Thus,
        // the stream object contains a borrow, prompting the '_ annotation.
        if *ctxt.inner() {
            yield Event::data("hi");
        }
    }
}
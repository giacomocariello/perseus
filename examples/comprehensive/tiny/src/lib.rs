use perseus::{Html, PerseusApp, Template};
use sycamore::view;

pub fn main<G: Html>() -> PerseusApp<G> {
    PerseusApp::new().template(|| {
        Template::new("index").template(|_| {
            view! {
                p { "Hello World!" }
            }
        })
    })
}

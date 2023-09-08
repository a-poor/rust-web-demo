use axum::Router;
use axum::routing::get;
use maud::{html, DOCTYPE, Markup};
use std::net::SocketAddr;


// Load the minified Alpine.js (v3.x.x) source code...
static ALPINE_SCRIPT_SRC: &'static str = "https://cdn.jsdelivr.net/npm/alpinejs@3.x.x/dist/cdn.min.js";

async fn index() -> Markup {
    html! {
        (DOCTYPE)
        head {
            script defer src=(ALPINE_SCRIPT_SRC) {}
        }
        main {
            h1 {
                "Hello, World!"
            }

            h1 x-data="{ message: 'I ❤️ Alpine' }" x-text="message" {}

            div x-data="{ count: 0 }" {
                button type="button" x-on:click="count++" {
                    "Clicked "
                    span x-text="count" {}
                    " times"
                }
                button type="button" x-on:click="count = 0" {
                    "Reset"
                }
            }
        }
        
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

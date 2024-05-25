#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus::web::Config;
use tracing::Level;
use wasm_bindgen::prelude::*;
pub mod ws;

#[wasm_bindgen]
pub fn run() {
    console_error_panic_hook::set_once();
    // Init logger
    launch_run();
}

pub fn launch_run(){
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    LaunchBuilder::web()
    .with_cfg(Config::new().rootname("better-spider-box"))
    .launch(App);
}

fn App() -> Element {
    let document = web_sys::window().unwrap().document().unwrap();
    let mut count = use_signal(|| 0);

    rsx! {
        div {
            h1 { "High-Five counter: {count}" }
            button { onclick: move |_| count += 1, "Up high!" }
            button { onclick: move |_| count -= 1, "Down low!" }
        }
    }
}
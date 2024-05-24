#![allow(non_snake_case)]


use dioxus::prelude::*;
use dioxus::web::Config;
use tracing::Level;
use extension::App;

fn main() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    LaunchBuilder::web()
    .with_cfg(Config::new().rootname("main"))
    .launch(App);
}
#![allow(non_snake_case)]

use components::editor::Editor;
use diamond_types::list::{Branch as ListBranch, OpLog as ListOpLog};
use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level}; // OpLog と Branch を正しくインポート

mod components;
mod config;
mod func;
mod lib;
mod types;

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    launch(App);
}

#[component]
fn App() -> Element {
    Editor {}
}

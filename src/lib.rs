pub mod components;
pub mod solver;

use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[wasm_bindgen(start)]
pub fn run_app() {
  App::<components::entry::Entry>::new().mount_to_body();
}

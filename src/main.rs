mod solver;

extern crate colored;
use solver::solve;
use yew::prelude::*;

#[function_component(Entry)]
pub fn entry() -> Html {
  html! {
    <app />
  }
}

fn main() {
  solve()
  // yew::start_app::<Entry>();
}

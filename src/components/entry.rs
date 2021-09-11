use crate::solver;
use yew::prelude::*;

pub struct Entry {
  link: ComponentLink<Self>,
  processing: bool,
  result: [[i8; 8]; 8],
}

pub enum Msg {
  Solve,
}

impl Component for Entry {
  type Message = Msg;
  type Properties = ();
  fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
    Self {
      link,
      processing: false,
      result: [
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
      ],
    }
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    match msg {
      Msg::Solve => {
        self.processing = true;
        self.result = solver::solve();
        self.processing = false;
      }
    }
    true
  }

  fn change(&mut self, _props: Self::Properties) -> ShouldRender {
    // Should only return "true" if new properties are different to
    // previously received properties.
    // This component has no properties so we will always return "false".
    false
  }

  fn view(&self) -> Html {
    let result = self.result;

    if self.processing {
      return html! {
        <div>{"processing"}</div>
      };
    }

    html! {
        <main class="container">
          <h1 class="text-3xl">{ "puzzle solver" }</h1>
          <div>
            <button
              onclick=self.link.callback(|_| Msg::Solve)
              class="border border-gray py-2 px-4"
            >
              { "Solve" }
            </button>
            <hr />
            <div>
              {
                for result.iter().map(|line| {
                  html! {
                    <div class="flex">
                    {
                      for line.iter().map(|cell| {
                        html! {
                          <div class={
                            [
                              match cell {
                                1 => { "bg-[#32A2ED]" },
                                2 => { "bg-[#24E8B6]" },
                                3 => { "bg-[#F9F871]" },
                                4 => { "bg-[#A4449B]" },
                                5 => { "bg-[#265E58]" },
                                6 => { "bg-[#002F6B]" },
                                7 => { "bg-[#79FAC5]" },
                                8 => { "bg-[#EF795B]" },
                                9 => { "bg-[#E379A6]" },
                                10 => { "bg-[#885300]" },
                                11 => { "bg-[#FFC85B]" },
                                12 => { "bg-[#FF49DF]" },
                                13 => { "bg-[#EF2B2B]" },
                                _ => { "bg-[#ffffff]" }
                            },
                              " border-gray border w-[20px] h-[20px] border-box mr-[1px] mb-[1px]"
                            ].join("")
                          }>{" "}</div>
                        }
                      })
                    }
                    </div>
                  }
                })
              }
            </div>
          </div>
        </main>
    }
  }
}

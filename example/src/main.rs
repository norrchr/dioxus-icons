use dioxus::prelude::*;

use dioxus_icons::icons::heroicons::academic_cap_outline_24;
use dioxus_icons::icons::materialicons::action::icons::accessibility_24px;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx!{
        div {
            "Dioxus Icons example"

            div {
                "Heroicons: academic_cap_outline_24"

                academic_cap_outline_24 {}

                academic_cap_outline_24 {
                    width: "50",
                    height: "50",
                    fill: "red"
                }
            }

            div {
                "Material Icons: accessibility_24px"

                accessibility_24px {}

                accessibility_24px {
                    fill: "none",
                    stroke_width: "2",
                    stroke: "orange"
                }
            }
        }
    }
}

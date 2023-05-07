use dioxus::prelude::*;

use crate::components::CronTab;

use super::Page;

pub fn CronPage(cx: Scope) -> Element {
    cx.render(rsx! {
        Page {
            div {
                class: "flex flex-col space-y-12",

                    h1 {
                        class: "text-2xl font-semibold mb-6",
                        "Cron Analyzer"
                    }

                CronTab {}
            }
        }
    })
}

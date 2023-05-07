use dioxus::{html::input_data::keyboard_types::Key, prelude::*};

use crate::utils::CronAnalyzer;

pub fn CronTab(cx: Scope) -> Element {
    let cron_state = use_state(cx, || "0 30 9,12,15 1,15 May-Aug Mon,Wed,Fri 2018/2".to_owned());
    let cron_result = use_state(cx, || "".to_owned());
    let expr = cron_state.get();

    // Move the focus to the search bar.
    // autofocus property on input is having issues: https://github.com/DioxusLabs/dioxus/issues/725

    cx.render(rsx! {
        input {
            class: "rounded bg-[#0e0e10] border-2 border-white focus:border-0 text-slate-100 p-5 w-full focus:ring-0 focus:outline-0 text-base",
            id: "cron-tab",
            r#type: "text",
            placeholder: "Enter 7 field cron expression e.g */10 * * * * * *",
            value: "{expr}",
            oninput: move |e| {
                // v = e.value.clone().as_str().to_string();
                cron_state.set(e.value.clone().as_str().to_string());
            },
            onclick: move |e| e.stop_propagation(),
            onkeydown: move |e| {
                if e.key() == Key::Enter {
                    cron_result.set(CronAnalyzer::from_expr(expr.clone()))
                } 
            },
        }
        p {
            "{cron_result.get()}"
        }
    })
}

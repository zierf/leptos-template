use leptos::leptos_dom::ev::SubmitEvent;
use leptos::*;
use leptos_meta::*;
use serde::{Deserialize, Serialize};
use stylers::style_sheet_str;

use crate::{invoke, GlobalState};

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[component]
pub fn Greeter() -> impl IntoView {
    let state = expect_context::<RwSignal<GlobalState>>();

    // `create_slice` lets us create a "lens" into the data
    let (name, set_name) = create_slice(
        // we take a slice *from* `state`
        state,
        // our getter returns a "slice" of the data
        |state| state.name.clone(),
        // our setter describes how to mutate that slice, given a new value
        |state, n| state.name = n,
    );

    let (greet_msg, set_greet_msg) = create_signal(String::new());

    let update_name = move |ev| {
        let v = event_target_value(&ev);
        set_name(v);
    };

    let greet = move |ev: SubmitEvent| {
        ev.prevent_default();
        spawn_local(async move {
            let name = name.get_untracked();
            if name.is_empty() {
                return;
            }

            let args = serde_wasm_bindgen::to_value(&GreetArgs { name: &name })
                .expect("greeting arguments");

            // Learn more about Tauri commands at https://v2.tauri.app/develop/calling-rust/
            let new_msg = invoke("greet", args)
                .await
                .as_string()
                .expect("returned message string");

            set_greet_msg(new_msg);
        });
    };

    let (style_scope, style_str) = style_sheet_str!("./styles/greeter.css");

    view! { class=style_scope,
        <Style>{style_str}</Style>

        <div>"Current Name: \"" {move || name()} "\""</div>

        <form class="row" on:submit=greet>
            <input
                id="greet-input"
                placeholder="Enter a name …"
                on:input=update_name
                value=name()
            />
            <button type="submit">"Greet"</button>
        </form>

        <p>
            <b>{move || greet_msg()}</b>
        </p>
    }
}

use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};
use stylers::style_sheet_str;
use wasm_bindgen::prelude::*;

mod counter;
mod greeter;
mod multimedia;

use self::{counter::Counter, greeter::Greeter, multimedia::Multimedia};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    pub async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct GlobalState {
    pub count: i32,
    pub name: String,
}

#[component]
pub fn App() -> impl IntoView {
    // provide global state with persistence in localStorage
    let local_storage = web_sys::window()
        .expect("globalThis object")
        .local_storage()
        .expect("local storage property")
        .expect("storage instance");

    let storage_value = local_storage
        .get_item("global-state")
        .expect("optional storage item");

    logging::log!("localStorage = {}", format!("{:?}", storage_value));

    provide_context(create_rw_signal(match storage_value {
        None => GlobalState::default(),
        Some(value_string) => serde_json::from_str(&value_string).expect("global-state"),
    }));

    let state = expect_context::<RwSignal<GlobalState>>();

    // update changes to global state in localStorage
    create_effect(move |_| {
        logging::log!(
            "StateChange = {}",
            state.with(|state| format!("{:?}", state))
        );

        let serialized =
            serde_json::to_string(&state.with(|state| state.clone())).expect("serialized value");

        local_storage
            .set_item("global-state", &serialized)
            .expect("setting an item");
    });

    let (style_scope, style_str) = style_sheet_str!("./styles/app.css");

    view! { class=style_scope,
        <Html lang="en" dir="ltr" />
        <Body class="dark" attr:data-theme="dark" />
        <Style>{style_str}</Style>

        <main class="container">
            <div class="row">
                <a href="https://tauri.app" target="_blank">
                    <img src="./assets/tauri.svg" class="logo tauri" alt="Tauri logo" />
                </a>
                <a href="https://docs.rs/leptos/" target="_blank">
                    <img src="./assets/leptos.svg" class="logo leptos" alt="Leptos logo" />
                </a>
            </div>

            <p>"Click on the Tauri and Leptos logos to learn more."</p>

            <p>{move || state.with(|state| format!("{:?}", state))}</p>

            <Router>
                <nav>
                    <a href="/">"Home"</a>
                    " | "
                    <a href="/count">"Counter"</a>
                    " | "
                    <a href="/media">"Multimedia"</a>
                    " | "
                    <a href="/doesnotexist">"Unavailable"</a>
                </nav>
                <br />
                <Routes>
                    <Route path="/" view=Greeter />
                    <Route path="/count" view=Counter />
                    <Route path="/media" view=Multimedia />
                    <Route path="/*any" view=|| view! { <h1>"Not Found"</h1> } />
                </Routes>
            </Router>
        </main>
    }
}

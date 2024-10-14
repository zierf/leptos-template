use leptos::{leptos_dom::ev::SubmitEvent, *};
use leptos_router::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
struct GlobalState {
    count: i32,
    name: String,
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

    view! {
        <main class="container">
            <div class="row">
                <a href="https://tauri.app" target="_blank">
                    <img src="public/tauri.svg" class="logo tauri" alt="Tauri logo" />
                </a>
                <a href="https://docs.rs/leptos/" target="_blank">
                    <img src="public/leptos.svg" class="logo leptos" alt="Leptos logo" />
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
                    <a href="/doesnotexist">"Unavailable"</a>
                </nav>
                <br />
                <Routes>
                    <Route path="/" view=Greeter />
                    <Route path="/count" view=Counter />
                    <Route path="/*any" view=|| view! { <h1>"Not Found"</h1> } />
                </Routes>
            </Router>
        </main>
    }
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

    view! {
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

#[component]
fn Counter() -> impl IntoView {
    let state = expect_context::<RwSignal<GlobalState>>();

    // `create_slice` lets us create a "lens" into the data
    let (count, set_count) = create_slice(
        // we take a slice *from* `state`
        state,
        // our getter returns a "slice" of the data
        |state| state.count,
        // our setter describes how to mutate that slice, given a new value
        |state, n| state.count = n,
    );

    let double_count = Signal::derive(move || count.get() * 2);
    let memoized_triple_count = create_memo(move |_| count.get() * 3);

    view! {
        <div>
            <div>"Count is: " {count}</div>
            <div>"Double-Count: " {double_count}</div>
            <div>"Triple-Count: " {memoized_triple_count}</div>

            <br />

            <button on:click=move |_| {
                set_count(count() + 1);
            }>"+"</button>
            <button on:click=move |_| {
                set_count(count() - 1);
            }>"-"</button>
        </div>
    }
}

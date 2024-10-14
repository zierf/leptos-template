use leptos::*;

use crate::GlobalState;

#[component]
pub fn Counter() -> impl IntoView {
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

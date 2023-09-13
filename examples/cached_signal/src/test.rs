use dioxus::prelude::*;
use dioxus_std::cached_states::use_cached_signal;

#[allow(non_snake_case)]
pub fn Route1(cx: Scope) -> Element {
    let mut signal = use_cached_signal(cx, || 0);
    render! {
        div {
            "Route1"
        }
        div {
            "{signal}"
        }
        button {
            onclick: move |_| {
                *signal.write() += 3;
            },
            "Increment3"
        }
    }
}
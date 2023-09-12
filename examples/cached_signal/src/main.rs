use dioxus::prelude::*;
use dioxus_router::prelude::*;
use dioxus_std::cached_signal::use_cached_signal;
use std::env;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    dioxus_desktop::launch(app);
}

fn app(cx: Scope) -> Element {
    render! {
        Router::<Route> {}
    }
}

#[derive(Clone, Debug, PartialEq, Routable)]
enum Route {
    #[layout(Layout)]
    #[route("/route1")]
    Route1 {},
    #[route("/route2")]
    Route2 {},
    #[end_layout]
    #[route("/")]
    Index {},
}

fn Layout(cx: Scope) -> Element {
    render! {
        Link {
            to: Route::Index {},
            "Index"
        }
        Outlet::<Route> {}
    }
}

fn Index(cx: Scope) -> Element {
    render! {
        div {
            "Hello world"
        }
        Link {
            to: Route::Route1 {},
            "Route1"
        }
        Link {
            to: Route::Route2 {},
            "Route2"
        }
    }
}

#[allow(non_snake_case)]
fn Route1(cx: Scope) -> Element {
    let mut signal = use_cached_signal(cx, || 0);
    render! {
        div {
            "Route1"
            "{signal}"
        }
        button {
            onclick: move |_| {
                *signal.write() += 1;
            },
            "Increment"
        }
    }
}

#[allow(non_snake_case)]
fn Route2(cx: Scope) -> Element {
    let mut signal = use_cached_signal(cx, || 0);
    render! {
        div {
            "Route2"
            "{signal}"
        }
        button {
            onclick: move |_| {
                *signal.write() += 2;
            },
            "Increment"
        }
    }
}

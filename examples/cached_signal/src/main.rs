use dioxus::prelude::*;
use dioxus_router::prelude::*;
use dioxus_signals::use_signal;
use dioxus_std::cached_states::use_cached_signal;
use std::env;
use log::set_boxed_logger;
mod simple_logger;
mod test;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    match set_boxed_logger(Box::new(simple_logger::SimpleLogger)) {
        Ok(_) => log::set_max_level(log::Level::Info.to_level_filter()),
        Err(e) => panic!("Failed to initialize logger: {}", e),
    }
    dioxus_desktop::launch(app);
}

fn app(cx: Scope) -> Element {
    render! {
        Router::<Route> {}
    }
}

#[rustfmt::skip]
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
        p {
            Link {
                to: Route::Index {},
                "Index"
            }
        }
        Outlet::<Route> {}
    }
}

fn Index(cx: Scope) -> Element {
    render! {
        p {
            Link {
                to: Route::Route1 {},
                "Route1"
            }
        }
        p {
            Link {
                to: Route::Route2 {},
                "Route2"
            }
        }
    }
}

#[allow(non_snake_case)]
fn Route1(cx: Scope) -> Element {
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
                *signal.write() += 1;
            },
            "Increment1"
        }
        test::Route1 {}
    }
}

#[allow(non_snake_case)]
fn Route2(cx: Scope) -> Element {
    let mut signal = use_signal(cx, || 0);
    render! {
        div {
            "Route2"
        }
        div {
            "{signal}"
        }
        button {
            onclick: move |_| {
                *signal.write() += 2;
            },
            "Increment2"
        }
    }
}

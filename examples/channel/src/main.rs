use dioxus::prelude::*;
use dioxus_sdk::utils::channel::{use_channel, use_listen_channel};

fn main() {
    // init debug tool for WebAssembly
    wasm_logger::init(wasm_logger::Config::default());
    console_error_panic_hook::set_once();

    launch(app);
}

fn app() -> Element {
    let channel = use_channel::<String>(5);

    use_listen_channel(&channel, |message| async {
        match message {
            Ok(value) => log::info!("Incoming message: {value}"),
            Err(err) => log::info!("Error: {err:?}"),
        }
    });

    let send = move |_: MouseEvent| {
        to_owned![channel];
        async move {
            channel.send("Hello").await.ok();
        }
    };

    rsx!(
        button {
            onclick: send,
            "Send hello"
        }
    )
}

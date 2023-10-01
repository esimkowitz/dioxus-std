use dioxus::prelude::*;
use cacao::defaults::{UserDefaults, Value};
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Mutex;

static USER_DEFAULTS: Lazy<Mutex<UserDefaults>> = Lazy::new(|| {
    Mutex::new(UserDefaults::standard())
});

fn main() {
    USER_DEFAULTS.lock().unwrap().register({
        let mut map = HashMap::new();
        map.insert("test", Value::string("value1"));
        map
    });

    dioxus_desktop::launch(app);
}

fn app(cx: Scope) -> Element {
    let value = USER_DEFAULTS.lock().unwrap().get("test").unwrap().as_str().unwrap().to_string();
    render! {
        input {
            onchange: move |event| {
                let value = event.value.clone();
                USER_DEFAULTS.lock().unwrap().insert("test", Value::string(value));
            },
            value: "{value}"
        }
    }
}

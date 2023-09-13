use dioxus::prelude::*;
use dioxus_signals::Signal;
use once_cell::sync::Lazy;
use serde::{de::DeserializeOwned, Serialize};
use std::{collections::HashMap, sync::Mutex};

static SIGNAL_CACHE: Lazy<Mutex<HashMap<String, String>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

pub fn use_cached_signal<T: 'static + Serialize + DeserializeOwned>(
    cx: &ScopeState,
    f: impl FnOnce() -> T,
) -> Signal<T> {
    let scope_name = cx.name();
    let key = format!("signal-{}{}", scope_name, &std::any::type_name::<T>());
    log::info!("Key: {}", key);
    let hook = *cx.use_hook(|| {
        if let Some(signal) = SIGNAL_CACHE.lock().unwrap().get(&key.clone()) {
            if let Ok(signal) = serde_json::from_str::<Signal<T>>(signal) {
                return signal;
            }
        }
        Signal::new(f())
    });

    hook.with(|val| {
        SIGNAL_CACHE
            .lock()
            .unwrap()
            .insert(key, serde_json::to_string(val).unwrap())
    });

    hook
}

#[test]
fn test_use_cached_state() {
    fn app(cx: Scope) -> Element {
        let signal = use_cached_signal(cx, || 0);
        render! {
            "{signal}"
        }
    }
}

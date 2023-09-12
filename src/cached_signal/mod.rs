// use dioxus::prelude::{use_state, UseState, use_shared_state_provider, ScopeState};
use dioxus::prelude::*;
use dioxus_signals::Signal;
use serde::{Serialize, de::DeserializeOwned};
use std::{collections::HashMap, sync::Mutex};
use once_cell::sync::Lazy;

static SIGNAL_CACHE: Lazy<Mutex<HashMap<String, String>>> = Lazy::new(|| Mutex::new(HashMap::new()));

pub fn use_cached_signal<T: 'static + Serialize + DeserializeOwned>(cx: &ScopeState, f: impl FnOnce() -> T) -> Signal<T> {
    let key = format!("{}{}", cx.scope_id().0, &std::any::type_name::<T>());
    let hook = *cx.use_hook(|| {
        if let Some(signal) = SIGNAL_CACHE.lock().unwrap().get(&key.clone()) {
            if let Ok(signal) = serde_json::from_str::<Signal<T>>(signal) {
                return signal;
            }
        }
        let signal = Signal::new(f());
        SIGNAL_CACHE.lock().unwrap().insert(key.clone(), serde_json::to_string(&signal).unwrap());
        return signal;
    });

    hook.with(|val| SIGNAL_CACHE.lock().unwrap().insert(key, serde_json::to_string(val).unwrap()));

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
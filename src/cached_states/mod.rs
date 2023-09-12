// use dioxus::prelude::{use_state, UseState, use_shared_state_provider, ScopeState};
use dioxus::prelude::*;
use dioxus_signals::Signal;
use serde::{Serialize, de::DeserializeOwned};
use std::{collections::HashMap, sync::Mutex};
use once_cell::sync::Lazy;

static SIGNAL_CACHE: Lazy<Mutex<HashMap<String, String>>> = Lazy::new(|| Mutex::new(HashMap::new()));

pub fn use_cached_signal<T: 'static + Serialize + DeserializeOwned>(cx: &ScopeState, f: impl FnOnce() -> T) -> Signal<T> {
    let key = format!("signal-{}{}", cx.scope_id().0, &std::any::type_name::<T>());
    let hook = *cx.use_hook(|| {
        if let Some(signal) = SIGNAL_CACHE.lock().unwrap().get(&key.clone()) {
            if let Ok(signal) = serde_json::from_str::<Signal<T>>(signal) {
                return signal;
            }
        }
        return Signal::new(f());
    });

    hook.with(|val| SIGNAL_CACHE.lock().unwrap().insert(key, serde_json::to_string(val).unwrap()));

    hook
}

pub fn use_cached_state<T: 'static + Serialize + DeserializeOwned + Clone>(cx: &ScopeState, f: impl FnOnce() -> T) -> &UseState<T> {
    let key = format!("state-{}{}", cx.scope_id().0, &std::any::type_name::<T>());
    let hook = cx.use_hook(|| {
        if let Some(state) = SIGNAL_CACHE.lock().unwrap().get(&key.clone()) {
            if let Ok(state) = serde_json::from_str::<T>(state) {
                return use_state(cx, move || state.clone());
            }
        }
        return use_state(cx, f);
    });

    SIGNAL_CACHE.lock().unwrap().insert(key, serde_json::to_string(hook.get()).unwrap());
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
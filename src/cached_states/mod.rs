use dioxus::prelude::*;
use dioxus_router::prelude::*;
use dioxus_signals::Signal;
use once_cell::sync::Lazy;
use serde::{de::DeserializeOwned, Serialize};
use std::{vec::Vec, sync::{atomic::{AtomicUsize, Ordering::SeqCst}, Mutex}, panic, any::Any, collections::HashMap, hash::Hash};

static SIGNAL_STACK_MAP: Lazy<Mutex<HashMap<String, SignalStack>>> = Lazy::new(|| Mutex::new(HashMap::new()));

struct SignalStack {
    stack: Vec<String>,
    cursor: usize,
}

fn get_map_key(cx: &ScopeState) -> String {
    format!("{}-{}", cx.name(), cx.height())
}

pub fn use_cached_signal<T: 'static + Serialize + DeserializeOwned>(
    cx: &ScopeState,
    f: impl FnOnce() -> T,
) -> Signal<T> {
    let hook = *cx.use_hook(|| {
        let mut signal_stack = SIGNAL_STACK.lock().unwrap();
        let mut cursor = SIGNAL_STACK_CURSOR.lock().unwrap();
        
        let signal = {
            if signal_stack.len() > *cursor {
                serde_json::from_str::<Signal<T>>(&signal_stack[*cursor]).map_or_else(|_| Signal::new(f()), |s| s)
            } else {
                let init_val = f();
                signal_stack.push(serde_json::to_string(&init_val).unwrap());
                Signal::new(init_val)
            }
        };
        *cursor += 1;
        signal
    });

    if cx.generation() == 0 {
        return hook;
    }
    else {
        let mut signal_stack = SIGNAL_STACK.lock().unwrap();
        let mut cursor = SIGNAL_STACK_CURSOR.lock().unwrap();
        log::info!("gen!=0; cursor: {}, Signal stack size: {}", cursor, signal_stack.len());
        if *cursor >= signal_stack.len() - 1 {
            *cursor += 1;
        } else {
            *cursor = 0;
        }
        log::info!("gen!=0; cursor to use: {}", cursor);
        signal_stack[*cursor] = serde_json::to_string(&hook).unwrap();
    }

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

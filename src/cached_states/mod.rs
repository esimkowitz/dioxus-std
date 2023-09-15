use dioxus::prelude::*;
use dioxus_router::prelude::*;
use dioxus_signals::Signal;
use once_cell::sync::Lazy;
use serde::{de::DeserializeOwned, Serialize};
use std::{vec::Vec, sync::{atomic::{AtomicUsize, Ordering::SeqCst}, Mutex}, panic, any::Any};

static SIGNAL_STACK: Lazy<Mutex<Vec<String>>> = Lazy::new(|| Mutex::new(Vec::new()));

static SIGNAL_STACK_CURSOR: Lazy<Mutex<usize>> = Lazy::new(|| Mutex::new(0));

pub fn use_cached_signal<T: 'static + Serialize + DeserializeOwned>(
    cx: &ScopeState,
    f: impl FnOnce() -> T,
) -> Signal<T> {
    log::info!("Scope ID: {}, Scope generation: {}", cx.scope_id().0, cx.generation());
    log::info!("Signal stack size before: {}", SIGNAL_STACK.lock().unwrap().len());
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

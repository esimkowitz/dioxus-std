// use dioxus::prelude::{use_state, UseState, use_shared_state_provider, ScopeState};
use dioxus::prelude::*;
use dioxus_hooks::UseState;
use serde::{Serialize, de::DeserializeOwned};
use std::{collections::HashMap, any::Any};
use once_cell::sync::Lazy;

static STATE_CACHE: Lazy<HashMap<String, String>> = Lazy::new(|| HashMap::new());

pub fn use_cached_state<T: 'static + Serialize + DeserializeOwned>(
    cx: &ScopeState,
    initial_state_fn: impl FnOnce() -> T,
) -> &UseState<T> {
    let hook = cx.use_hook(move || {
        let current_val = Rc::new(initial_state_fn());
        let update_callback = cx.schedule_update();
        let slot = Rc::new(RefCell::new(current_val.clone()));
        let setter = Rc::new({
            to_owned![update_callback, slot];
            move |new| {
                {
                    let mut slot = slot.borrow_mut();

                    // if there's only one reference (weak or otherwise), we can just swap the values
                    // Typically happens when the state is set multiple times - we don't want to create a new Rc for each new value
                    if let Some(val) = Rc::get_mut(&mut slot) {
                        *val = new;
                    } else {
                        *slot = Rc::new(new);
                    }
                }
                update_callback();
            }
        });

        UseState {
            current_val,
            update_callback,
            setter,
            slot,
        }
    });

    hook.current_val = hook.slot.borrow().clone();

    hook
    *new_use_state.update_callback = 
}

#[test]
fn test_use_cached_state() {
    fn app(cx: Scope) -> Element {
        use_cached_state(cx, || 0);


    }
}
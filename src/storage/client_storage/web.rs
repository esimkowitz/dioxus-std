#![allow(unused)]
use async_broadcast::Receiver;
use dioxus::prelude::*;
use once_cell::sync::{Lazy, OnceCell};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::Closure;
use std::any::TypeId;
use std::cell::{Ref, RefMut};
use std::collections::HashMap;
use std::fmt::Debug;
use std::io::Write;
use std::rc::Rc;
use std::sync::Mutex;
use std::thread::LocalKey;
use std::{
    fmt::Display,
    ops::{Deref, DerefMut},
};
use web_sys::{window, Storage};

use crate::storage::storage::{
    serde_from_string, serde_to_string, storage_entry, try_serde_from_string,
    use_synced_storage_entry, StorageBacking, StorageEntry, StorageEntryMut,
};

fn local_storage() -> Option<Storage> {
    window()?.local_storage().ok()?
}

fn set<T: Serialize>(key: String, value: &T) {
    #[cfg(not(feature = "ssr"))]
    {
        let as_str = serde_to_string(value);
        local_storage().unwrap().set_item(&key, &as_str).unwrap();
    }
}

fn get<T: DeserializeOwned>(key: &str) -> Option<T> {
    #[cfg(not(feature = "ssr"))]
    {
        let s: String = local_storage()?.get_item(key).ok()??;
        try_serde_from_string(&s)
    }
    #[cfg(feature = "ssr")]
    None
}

#[derive(Clone)]
pub struct ClientStorage;

impl StorageBacking for ClientStorage {
    type Key = String;

    fn use_onchange<T: DeserializeOwned + 'static>(_: &ScopeState, key: String, action: impl Fn() + 'static) {
        let closure = Closure::<dyn FnMut(web_sys::StorageEvent)>::new(move |e: web_sys::StorageEvent| {
            let event_key = e.key().unwrap();
            log::info!("Incoming storage event key: {}", event_key);
            if event_key == key {
                action();
            }
        });
        window()
            .unwrap()
            .add_event_listener_with_callback("storage", closure.as_ref().unchecked_ref())
            .unwrap();
    }

    fn set<T: Serialize>(key: String, value: &T) {
        set(key, value);
    }

    fn get<T: DeserializeOwned>(key: &String) -> Option<T> {
        get(key)
    }
}

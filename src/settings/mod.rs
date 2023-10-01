use dioxus::prelude::*;
use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))] {
        mod desktop;
    }
    else if #[cfg(target_family = "wasm")]{
        mod wasm;
    }
    else if #[cfg(target_os = "ios")] {
        mod apple;
    }
    else {
        compile_error!("Unsupported target");
    }
}

pub(crate) trait SettingsStore {
    
}



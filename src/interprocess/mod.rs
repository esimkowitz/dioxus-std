use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(target_family = "wasm")] {
        use web_sys::BroadcastChannel;
    }
    else {
        use interprocess:
    }
}
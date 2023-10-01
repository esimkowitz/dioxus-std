use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "color_scheme")] {
        pub mod color_scheme;
    }
}

cfg_if! {
    if #[cfg(feature = "geolocation")] {
        pub mod geolocation;
    }
}

cfg_if! {
    if #[cfg(any(feature = "utils"))] {
        pub mod utils;
    }
}

cfg_if! {
    if #[cfg(feature = "i18n")] {
        pub mod i18n;
    }
}

cfg_if! {
    if #[cfg(feature = "clipboard")] {
        pub mod clipboard;
    }
}

cfg_if! {
    if #[cfg(feature = "settings")] {
        pub mod settings;
    }
}

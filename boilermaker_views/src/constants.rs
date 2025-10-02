use dioxus::prelude::*;

pub const FAVICON: Asset = asset!("/assets/logo-flame.png");
pub const MAIN_CSS: Asset = asset!("/assets/main.css");
pub const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

pub const HIGHLIGHT_JS: Asset = asset!("/assets/highlight.min.js");
pub const GITHUB_LIGHT_CSS: Asset = asset!("/assets/github.min.css");
pub const GITHUB_DARK_CSS: Asset = asset!("/assets/github-dark.min.css");

pub const LINK_STYLE: &str = "text-blue-400 px-1";

pub const BTN_CREATE_STYLE: &str = "bg-green-600 hover:bg-green-700 text-white py-2 px-4 rounded";
pub const BTN_EDIT_STYLE: &str =
    "bg-neutral-300 hover:bg-blue-700 dark:bg-neutral-700 text-white py-1 px-1 rounded";
pub const BTN_DELETE_STYLE: &str =
    "bg-neutral-300 hover:bg-red-700 dark:bg-neutral-700 text-white py-1 px-1 rounded";

pub const TH_STYLE: &str = "p-2 text-left text-blue-400";
pub const TD_STYLE: &str = "p-2 border-b border-b-neutral-700";

pub const DROPDOWN_LINK_STYLE: &str =
    "block px-4 py-2 hover:bg-neutral-100 dark:hover:bg-neutral-700";
pub const INDENTED_DROPDOWN_LINK_STYLE: &str =
    "block px-8 py-2 hover:bg-neutral-100 dark:hover:bg-neutral-700";

pub const LABEL_STYLE: &str = "block text-sm font-bold mb-2";
pub const INPUT_STYLE: &str =
    "w-full p-2 border border-neutral-200 dark:border-neutral-400 dark:border-neutral-700 rounded";
pub const TEXTAREA_STYLE: &str = "w-full p-2 border border-neutral-200 dark:border-neutral-400 dark:border-neutral-700 rounded h-24";

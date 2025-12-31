use dioxus::prelude::*;

pub const FAVICON: Asset = asset!("/assets/img/logo-flame.png");
pub const MAIN_CSS: Asset = asset!("/assets/css/main.css");
pub const PRELOADER: Asset = asset!("/assets/img/preloader.gif");

pub const DROPDOWN_MENU_STYLE: &str = "absolute left-0 top-12 w-48 bg-white dark:bg-neutral-900 rounded shadow-lg border border-l-0 border-t-0 border-neutral-300 dark:border-neutral-700 z-10 text-sm ";

pub const DROPDOWN_LINK_STYLE: &str =
    "block px-4 py-2 hover:bg-neutral-100 dark:hover:bg-neutral-700";

pub const INDENTED_DROPDOWN_LINK_STYLE: &str =
    "block px-8 py-2 hover:bg-neutral-100 dark:hover:bg-neutral-700";

pub const NAVBAR_STYLE: &str = "flex flex-row space-x-4 p-2 items-center justify-between bg-gradient-to-b from-white to-neutral-100 dark:from-neutral-800 dark:to-neutral-900 border-b border-solid border-neutral-300 dark:border-neutral-950 text-neutral-600 dark:text-neutral-300";

pub const LINK_STYLE: &str = "text-blue-400 px-1";

pub const BTN_BLUE_STYLE: &str =
    "bg-neutral-300 hover:bg-blue-500 dark:bg-neutral-700 text-white py-1 px-2 rounded";
pub const BTN_GREEN_STYLE: &str =
    "bg-neutral-300 hover:bg-green-700 dark:bg-neutral-700 text-white py-1 px-2 rounded";
pub const BTN_RED_STYLE: &str =
    "bg-neutral-300 hover:bg-red-700 dark:bg-neutral-700 text-white py-1 px-2 rounded";

pub const TH_STYLE: &str = "p-2 text-left text-blue-400";
pub const TH_MUTED_STYLE: &str = "p-2 text-left text-neutral-400";
pub const TD_STYLE: &str = "p-2 border-b border-b-neutral-700";

pub const LABEL_STYLE: &str = "block text-sm font-bold mb-2";
pub const INPUT_STYLE: &str =
    "w-full p-2 border border-neutral-200 dark:border-neutral-400 dark:border-neutral-800 rounded";
pub const TEXTAREA_STYLE: &str = "w-full p-2 border border-neutral-200 dark:border-neutral-400 dark:border-neutral-800 rounded h-24";

pub const SEARCH_INPUT_STYLE: &str = "block w-full rounded-md py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 dark:ring-neutral-700 placeholder:text-gray-700 dark:placeholder:text-gray-100 focus:ring-2 focus:ring-inset focus:ring-neutral-600 sm:text-sm sm:leading-6 px-2 border border-neutral-300 dark:border-neutral-700 bg-white dark:bg-neutral-900";

pub mod prelude;

mod abstracts;

#[macro_use]
mod hooks;
mod components;
mod fns;

use dioxus::prelude::*;

static MODAL: GlobalSignal<Option<Element>> = Signal::global(|| None);

pub use fns::close;
pub use hooks::_use_modal;

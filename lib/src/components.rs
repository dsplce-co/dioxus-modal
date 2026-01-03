use crate::MODAL;
use crate::fns::close;
use crate::hooks::use_window_keydown;
use dioxus::prelude::*;

use dioxus_transition::prelude::*;

#[component]
pub fn ModalCollector() -> Element {
    #[cfg(not(feature = "ssr"))]
    use_window_keydown(move |event| {
        if event.key() == "Escape" {
            close();
        }
    });

    rsx! {
        div {
            role: "dialog",
            position: "relative",
            z_index: "2147483647",
            "aria-modal": MODAL().is_some().to_string(),
            "aria-labelledby": "modal-title",

            Transition {
                id: "dioxus_modal-overlay",
                kind: "blur",
                duration: 300,

                if MODAL().is_some() {
                    div {
                        "aria-hidden": "true",
                        id: "dioxus_modal-overlay",
                        position: "fixed",
                        top: "0",
                        right: "0",
                        bottom: "0",
                        left: "0",
                    }
                }
            }

            Transition {
                id: "dioxus_modal-node",
                kind: "modal",
                duration: 300,

                if let Some(children) = MODAL() {
                    div {
                        id: "dioxus_modal-node",
                        position: "fixed",
                        top: "0",
                        right: "0",
                        bottom: "0",
                        left: "0",
                        z_index: "2147483646",
                        width: "100vw",
                        overflow_y: "auto",

                        div {
                            padding: "1rem",
                            text_align: "center",
                            justify_content: "center",
                            align_items: "center",
                            min_height: "100%",
                            display: "flex",

                            {children}
                        }
                    }
                }
            }
        }
    }
}

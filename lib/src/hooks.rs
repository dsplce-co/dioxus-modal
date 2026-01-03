use crate::MODAL;
use crate::abstracts::Modal;
use crate::dioxus_core::use_hook_with_cleanup;
use crate::fns::close;
use dioxus::prelude::*;
use std::{cell::RefCell, ops::Deref, rc::Rc};
use web_sys::{EventTarget, KeyboardEvent};

use wasm_bindgen::{
    JsCast,
    closure::Closure,
    convert::{FromWasmAbi, RefFromWasmAbi},
};

pub fn _use_modal<U: 'static, V: 'static + Clone>(
    component: fn(U, V, fn()) -> Element,
    ctx: V,
) -> Modal<U> {
    Modal {
        open: Box::new(move |item: U| {
            *MODAL.write() = Some(component(item, ctx.clone(), close));
        }),
        close,
    }
}

#[derive(Clone)]
pub(super) struct EventListenerHandle {
    cleanup: Rc<RefCell<Option<Box<dyn FnOnce()>>>>,
}

impl EventListenerHandle {
    pub(super) fn new<EventKind, T>(
        target_element: T,
        event_name: &'static str,
        mut callback: impl FnMut(EventKind) + 'static,
    ) -> Self
    where
        EventKind: Sized + RefFromWasmAbi + FromWasmAbi + Clone + 'static,
        T: Clone + Deref<Target = EventTarget> + std::fmt::Debug + 'static,
    {
        let closure = Closure::wrap(Box::new(move |event: EventKind| {
            callback(event);
        }) as Box<dyn FnMut(_)>);

        if let Err(e) = target_element
            .add_event_listener_with_callback(event_name, closure.as_ref().unchecked_ref())
        {
            tracing::error!("failed to add event listener: {e:?}");
        }

        let cleanup = Rc::new(RefCell::new(Some(Box::new(move || {
            if let Err(e) = target_element
                .remove_event_listener_with_callback(event_name, closure.as_ref().unchecked_ref())
            {
                tracing::error!("failed to remove event listener: {e:?}");
            }
        }) as Box<dyn FnOnce()>)));
        Self { cleanup }
    }

    pub(super) fn cleanup(&self) {
        let cleanup = self.cleanup.borrow_mut().take();
        if let Some(cleanup) = cleanup {
            cleanup();
        }
    }
}

impl Drop for EventListenerHandle {
    fn drop(&mut self) {
        // Only cleanup if this is the last reference.
        if Rc::strong_count(&self.cleanup) == 1 {
            self.cleanup();
        }
    }
}

pub(crate) fn use_on_event<EventKind, T>(
    target_element: &T,
    event_name: &'static str,
    mut callback: impl FnMut(EventKind) + 'static,
) where
    EventKind: Sized + RefFromWasmAbi + FromWasmAbi + Clone + 'static,
    T: Clone + Deref<Target = EventTarget> + std::fmt::Debug + 'static,
{
    let hook = || {
        EventListenerHandle::new(target_element.clone(), event_name, move |kind| {
            callback(kind)
        })
    };

    let cleanup = |f: EventListenerHandle| {
        f.cleanup();
    };

    use_hook_with_cleanup(hook, cleanup);
}

pub(super) fn use_window_keydown(mut callback: impl FnMut(KeyboardEvent) + 'static) {
    let window = gloo::utils::window();

    let dioxus_callback = use_callback(move |event| {
        callback(event);
    });

    use_on_event(&window, "keydown", move |event: KeyboardEvent| {
        dioxus_callback.call(event);
    });
}

#[macro_export]
macro_rules! use_modal {
    ($component:expr, $ctx:expr) => {
        $crate::_use_modal($component, $ctx)
    };
    ($component:expr) => {
        $crate::_use_modal($component, ())
    };
}

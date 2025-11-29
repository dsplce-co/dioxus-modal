use crate::MODAL;

pub fn close() {
    *MODAL.write() = None;
}

use phper::sys;

use crate::hock;

pub fn init() {
    unsafe {
        sys::zend_observer_fcall_register(Some(hock::observer_handler))
    }
}

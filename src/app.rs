use std::sync::{Arc, Mutex};

use cef::{
    App, BrowserProcessHandler, ImplApp, Window, WrapApp,
    rc::{Rc, RcImpl},
    sys,
};

use crate::process::DemoBrowserProcessHandler;

pub struct DemoApp {
    object: *mut RcImpl<sys::_cef_app_t, Self>,
    window: Arc<Mutex<Option<Window>>>,
}

impl DemoApp {
    pub fn new(window: Arc<Mutex<Option<Window>>>) -> App {
        App::new(Self {
            object: std::ptr::null_mut(),
            window,
        })
    }
}

impl ImplApp for DemoApp {
    fn get_raw(&self) -> *mut sys::_cef_app_t {
        self.object as *mut sys::_cef_app_t
    }

    fn get_browser_process_handler(&self) -> Option<BrowserProcessHandler> {
        Some(DemoBrowserProcessHandler::new(self.window.clone()))
    }
}

//
//
//
//
// //////////////////////////////
// /          HELPERS           /
// //////////////////////////////
//
//
//
//

//
// DemoApp
//

impl WrapApp for DemoApp {
    fn wrap_rc(&mut self, object: *mut RcImpl<sys::_cef_app_t, Self>) {
        self.object = object;
    }
}

impl Clone for DemoApp {
    fn clone(&self) -> Self {
        let object = unsafe {
            let rc_impl = &mut *self.object;
            rc_impl.interface.add_ref();
            self.object
        };
        let window = self.window.clone();

        Self { object, window }
    }
}

impl Rc for DemoApp {
    fn as_base(&self) -> &sys::cef_base_ref_counted_t {
        unsafe {
            let base = &*self.object;
            std::mem::transmute(&base.cef_object)
        }
    }
}

use std::sync::{Arc, Mutex};

use cef::{
    BrowserProcessHandler, BrowserViewDelegate, CefString, DictionaryValue,
    ImplBrowserProcessHandler, RequestContext, Window, WrapBrowserProcessHandler,
    browser_view_create,
    rc::{Rc, RcImpl},
    sys, window_create_top_level,
};

use crate::{client::DemoClient, window::DemoWindowDelegate};

pub struct DemoBrowserProcessHandler {
    object: *mut RcImpl<sys::cef_browser_process_handler_t, Self>,
    window: Arc<Mutex<Option<Window>>>,
}

impl DemoBrowserProcessHandler {
    pub fn new(window: Arc<Mutex<Option<Window>>>) -> BrowserProcessHandler {
        BrowserProcessHandler::new(Self {
            object: std::ptr::null_mut(),
            window,
        })
    }
}

impl Rc for DemoBrowserProcessHandler {
    fn as_base(&self) -> &sys::cef_base_ref_counted_t {
        unsafe {
            let base = &*self.object;
            std::mem::transmute(&base.cef_object)
        }
    }
}

impl WrapBrowserProcessHandler for DemoBrowserProcessHandler {
    fn wrap_rc(&mut self, object: *mut RcImpl<sys::_cef_browser_process_handler_t, Self>) {
        self.object = object;
    }
}

impl Clone for DemoBrowserProcessHandler {
    fn clone(&self) -> Self {
        let object = unsafe {
            let rc_impl = &mut *self.object;
            rc_impl.interface.add_ref();
            rc_impl
        };

        let window = self.window.clone();

        Self { object, window }
    }
}

impl ImplBrowserProcessHandler for DemoBrowserProcessHandler {
    fn get_raw(&self) -> *mut sys::_cef_browser_process_handler_t {
        self.object.cast()
    }

    // The real lifespan of cef starts from `on_context_initialized`, so all the cef objects should be manipulated after that.
    fn on_context_initialized(&self) {
        println!("cef context intiialized");
        let mut client = DemoClient::new();
        let url = CefString::from("https://www.google.com");

        let browser_view = browser_view_create(
            Some(&mut client),
            Some(&url),
            Some(&Default::default()),
            Option::<&mut DictionaryValue>::None,
            Option::<&mut RequestContext>::None,
            Option::<&mut BrowserViewDelegate>::None,
        )
        .expect("Failed to create browser view");

        let mut delegate = DemoWindowDelegate::new(browser_view);
        if let Ok(mut window) = self.window.lock() {
            *window = Some(
                window_create_top_level(Some(&mut delegate)).expect("Failed to create window"),
            );
        }
    }
}

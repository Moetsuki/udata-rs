use cef::{
    BrowserView, ImplPanelDelegate, ImplView, ImplViewDelegate, ImplWindow, ImplWindowDelegate,
    WindowDelegate, WrapWindowDelegate, quit_message_loop,
    rc::{Rc, RcImpl},
    sys,
};

pub struct DemoWindowDelegate {
    base: *mut RcImpl<sys::_cef_window_delegate_t, Self>,
    browser_view: BrowserView,
}

impl DemoWindowDelegate {
    pub fn new(browser_view: BrowserView) -> WindowDelegate {
        WindowDelegate::new(Self {
            base: std::ptr::null_mut(),
            browser_view,
        })
    }
}

//
// View
//

impl ImplViewDelegate for DemoWindowDelegate {
    fn on_child_view_changed(
        &self,
        _view: Option<&mut impl ImplView>,
        _added: ::std::os::raw::c_int,
        _child: Option<&mut impl ImplView>,
    ) {
        // view.as_panel().map(|x| x.as_window().map(|w| w.close()));
    }

    fn get_raw(&self) -> *mut sys::_cef_view_delegate_t {
        self.base as *mut sys::_cef_view_delegate_t
    }
}

//
// Panel
//

impl ImplPanelDelegate for DemoWindowDelegate {}

//
// Window
//

impl ImplWindowDelegate for DemoWindowDelegate {
    fn on_window_created(&self, window: Option<&mut impl ImplWindow>) {
        if let Some(window) = window {
            let mut view = self.browser_view.clone();
            window.add_child_view(Some(&mut view));
            window.show();
        }
    }

    fn on_window_destroyed(&self, _window: Option<&mut impl ImplWindow>) {
        quit_message_loop();
    }

    fn with_standard_window_buttons(
        &self,
        _window: Option<&mut impl ImplWindow>,
    ) -> ::std::os::raw::c_int {
        1
    }

    fn can_resize(&self, _window: Option<&mut impl ImplWindow>) -> ::std::os::raw::c_int {
        1
    }

    fn can_maximize(&self, _window: Option<&mut impl ImplWindow>) -> ::std::os::raw::c_int {
        1
    }

    fn can_minimize(&self, _window: Option<&mut impl ImplWindow>) -> ::std::os::raw::c_int {
        1
    }

    fn can_close(&self, _window: Option<&mut impl ImplWindow>) -> ::std::os::raw::c_int {
        1
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
// DemoWindowDelegate
//

impl WrapWindowDelegate for DemoWindowDelegate {
    fn wrap_rc(&mut self, object: *mut RcImpl<sys::_cef_window_delegate_t, Self>) {
        self.base = object;
    }
}

impl Clone for DemoWindowDelegate {
    fn clone(&self) -> Self {
        unsafe {
            let rc_impl = &mut *self.base;
            rc_impl.interface.add_ref();
        }

        Self {
            base: self.base,
            browser_view: self.browser_view.clone(),
        }
    }
}

impl Rc for DemoWindowDelegate {
    fn as_base(&self) -> &sys::cef_base_ref_counted_t {
        unsafe {
            let base = &*self.base;
            std::mem::transmute(&base.cef_object)
        }
    }
}

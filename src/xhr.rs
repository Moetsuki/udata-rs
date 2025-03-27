use std::sync::{Arc, Mutex};

use crate::helpers::LimitString;
use cef::sys::cef_response_filter_status_t::{
    RESPONSE_FILTER_DONE, RESPONSE_FILTER_ERROR, RESPONSE_FILTER_NEED_MORE_DATA,
};
use cef::{
    CefString, CefStringUtf16, ImplBrowser, ImplFrame, ImplRequest, ImplRequestHandler,
    ImplResourceRequestHandler, ImplResponse, ImplResponseFilter, RequestHandler,
    ResourceRequestHandler, ResourceType, ResponseFilter, ResponseFilterStatus, UrlrequestStatus,
    WrapRequestHandler, WrapResourceRequestHandler, WrapResponseFilter,
    rc::{Rc, RcImpl},
    sys,
};

//
// RequestHandler
//

pub struct DemoRequestHandler(*mut RcImpl<sys::_cef_request_handler_t, Self>);

impl DemoRequestHandler {
    pub(crate) fn new() -> RequestHandler {
        RequestHandler::new(Self(std::ptr::null_mut()))
    }
}

impl ImplRequestHandler for DemoRequestHandler {
    fn get_resource_request_handler(
        &self,
        _browser: Option<&mut impl ImplBrowser>,
        _frame: Option<&mut impl ImplFrame>,
        _request: Option<&mut impl ImplRequest>,
        _is_navigation: ::std::os::raw::c_int,
        _is_download: ::std::os::raw::c_int,
        _request_initiator: Option<&CefString>,
        _disable_default_handling: Option<&mut ::std::os::raw::c_int>,
    ) -> Option<ResourceRequestHandler> {
        eprintln!(
            ">> +-----+ DemoRequestHandler::get_resource_request_handler --> DemoResourceRequestHandler"
        );

        let request = _request.unwrap();
        let url = CefStringUtf16::from(&request.get_url());
        eprintln!(">>       | URL:  {:}", url.to_string().limit(120));
        eprintln!(">>       | Mime: {:?}", request.get_resource_type());

        if let Some(request_initiator) = _request_initiator {
            eprintln!(">>       | Request initiator: {}", request_initiator);
        }

        eprintln!(">>       | Is navigation: {:}", _is_navigation);

        eprintln!(">>       | Is download: {:}", _is_download);

        let cache_control_str = CefString::from("Cache-Control");
        // let pragma_str = CefString::from("Pragma");
        // let expires_str = CefString::from("Expires");

        let cache_control_val = CefString::from("no-cache, no-store, must-revalidate");
        // let pragma_val = CefString::from("no-cache");
        // let expires_val = CefString::from("0");

        request.set_header_by_name(Some(&cache_control_str), Some(&cache_control_val), 1);

        // Verify the header was set correctly
        let header_value = request.get_header_by_name(Some(&cache_control_str));
        eprintln!(
            "Header set: {} = {}",
            cache_control_str,
            CefString::from(&header_value)
        );

        // Check if its an XHR request
        if request.get_resource_type() == ResourceType::from(sys::cef_resource_type_t::RT_XHR) {
            eprintln!(">>       | Found XHR request");
            return Some(DemoResourceRequestHandler::new());
        }

        if _is_download == 0 && _is_navigation == 0 {
            Some(DemoResourceRequestHandler::new())
        } else {
            None
        }
    }
    fn get_raw(&self) -> *mut sys::_cef_request_handler_t {
        self.0 as *mut sys::_cef_request_handler_t
    }
}

//
// ResourceHandler
//

pub struct DemoResourceRequestHandler(*mut RcImpl<sys::_cef_resource_request_handler_t, Self>);

impl DemoResourceRequestHandler {
    fn new() -> ResourceRequestHandler {
        ResourceRequestHandler::new(Self(std::ptr::null_mut()))
    }
}

impl ImplResourceRequestHandler for DemoResourceRequestHandler {
    fn on_resource_response(
        &self,
        _browser: Option<&mut impl ImplBrowser>,
        _frame: Option<&mut impl ImplFrame>,
        _request: Option<&mut impl ImplRequest>,
        _response: Option<&mut impl ImplResponse>,
    ) -> ::std::os::raw::c_int {
        eprintln!("DemoResourceRequestHandler::on_resource_response");

        let response = _response.unwrap();
        let request = _request.unwrap();
        let url = CefStringUtf16::from(&request.get_url());
        eprintln!("URL:    {:}", url.to_string().limit(120));
        eprintln!("Status: {}", response.get_status());
        eprintln!(
            "Mime:   {}",
            CefStringUtf16::from(&response.get_mime_type())
        );

        // Return 0 to allow the resource to load
        Default::default()
    }

    fn get_resource_response_filter(
        &self,
        _browser: Option<&mut impl ImplBrowser>,
        _frame: Option<&mut impl ImplFrame>,
        _request: Option<&mut impl ImplRequest>,
        _response: Option<&mut impl ImplResponse>,
    ) -> Option<ResponseFilter> {
        eprintln!(
            "DemoResourceRequestHandler::get_resource_response_filter --> DemoResponseFilter"
        );

        Some(DemoResponseFilter::new())

        //// Don't filter other resources
        // None
    }

    fn on_resource_load_complete(
        &self,
        _browser: Option<&mut impl ImplBrowser>,
        _frame: Option<&mut impl ImplFrame>,
        _request: Option<&mut impl ImplRequest>,
        _response: Option<&mut impl ImplResponse>,
        _status: UrlrequestStatus,
        _received_content_length: i64,
    ) {
        eprintln!(">>       +------+ DemoResourceRequestHandler::on_resource_load_complete");
        eprintln!(
            ">>              | Received content length: {}",
            _received_content_length
        );
    }

    fn get_raw(&self) -> *mut sys::_cef_resource_request_handler_t {
        self.0 as *mut sys::_cef_resource_request_handler_t
    }
}

//
// ResponseFilter
//

pub struct DemoResponseFilter {
    object: *mut RcImpl<sys::_cef_response_filter_t, Self>,
    buffer: Arc<Mutex<Vec<u8>>>,
}

impl DemoResponseFilter {
    fn new() -> ResponseFilter {
        ResponseFilter::new(Self {
            object: std::ptr::null_mut(),
            buffer: Arc::new(Mutex::new(Vec::new())),
        })
    }
}

impl ImplResponseFilter for DemoResponseFilter {
    fn init_filter(&self) -> ::std::os::raw::c_int {
        // Reset the buffer for a new request
        let mut buffer = self.buffer.lock().unwrap();
        buffer.clear();
        1 // Return true to indicate success
    }

    fn filter(
        &self,
        data_in: Option<&mut Vec<u8>>,
        data_in_read: Option<&mut usize>,
        data_out: Option<&mut Vec<u8>>,
        data_out_written: Option<&mut usize>,
    ) -> ResponseFilterStatus {
        if let Some(data_in) = data_in {
            let data_in_size = data_in.len();

            println!("Data in size: {}", data_in_size);

            if !data_in.is_empty() {
                if let Some(data_out) = data_out {
                    let copy_size = data_in_size.min(data_out.len());
                    data_out[..copy_size].copy_from_slice(&data_in[..copy_size]);

                    if let Some(data_in_read) = data_in_read {
                        *data_in_read = copy_size;
                    }

                    if let Some(data_out_written) = data_out_written {
                        *data_out_written = copy_size;
                    }

                    // Store data for later use
                    let mut buffer = self.buffer.lock().unwrap();
                    buffer.extend_from_slice(&data_in[..copy_size]);

                    // If we couldn't copy all the data, need more calls
                    if copy_size < data_in.len() {
                        eprintln!("Need more data");
                        return ResponseFilterStatus::from(RESPONSE_FILTER_NEED_MORE_DATA);
                    }

                    eprintln!("Done copying data");
                    return ResponseFilterStatus::from(RESPONSE_FILTER_DONE);
                }
            }
        } else if data_out.is_some() {
            // No input but output buffer provided - this is the flush case
            // Just indicate we're done since there's nothing to write
            if let Some(data_out_written) = data_out_written {
                *data_out_written = 0;
            }

            eprintln!("Done flushing data");
            return ResponseFilterStatus::from(RESPONSE_FILTER_DONE);
        }

        eprintln!("Error filtering data");
        ResponseFilterStatus::from(RESPONSE_FILTER_ERROR)
    }

    fn get_raw(&self) -> *mut sys::_cef_response_filter_t {
        self.object as *mut sys::_cef_response_filter_t
    }
}

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
// DemoRequestHandler
//

impl WrapRequestHandler for DemoRequestHandler {
    fn wrap_rc(&mut self, object: *mut RcImpl<sys::_cef_request_handler_t, Self>) {
        self.0 = object;
    }
}

impl Clone for DemoRequestHandler {
    fn clone(&self) -> Self {
        unsafe {
            let rc_impl = &mut *self.0;
            rc_impl.interface.add_ref();
        }
        Self(self.0)
    }
}

impl Rc for DemoRequestHandler {
    fn as_base(&self) -> &sys::cef_base_ref_counted_t {
        unsafe {
            let base = &*self.0;
            std::mem::transmute(&base.cef_object)
        }
    }
}

//
// DemoResourceRequestHandler
//

impl WrapResourceRequestHandler for DemoResourceRequestHandler {
    fn wrap_rc(&mut self, object: *mut RcImpl<sys::_cef_resource_request_handler_t, Self>) {
        self.0 = object;
    }
}

impl Clone for DemoResourceRequestHandler {
    fn clone(&self) -> Self {
        unsafe {
            let rc_impl = &mut *self.0;
            rc_impl.interface.add_ref();
        }
        Self(self.0)
    }
}

impl Rc for DemoResourceRequestHandler {
    fn as_base(&self) -> &sys::cef_base_ref_counted_t {
        unsafe {
            let base = &*self.0;
            std::mem::transmute(&base.cef_object)
        }
    }
}

//
// DemoResponseFilter
//

impl WrapResponseFilter for DemoResponseFilter {
    fn wrap_rc(&mut self, object: *mut RcImpl<sys::_cef_response_filter_t, Self>) {
        self.object = object;
    }
}

impl Clone for DemoResponseFilter {
    fn clone(&self) -> Self {
        unsafe {
            let rc_impl = &mut *self.object;
            rc_impl.interface.add_ref();
        }
        Self {
            object: self.object,
            buffer: self.buffer.clone(),
        }
    }
}

impl Rc for DemoResponseFilter {
    fn as_base(&self) -> &sys::cef_base_ref_counted_t {
        unsafe {
            let base = &*self.object;
            std::mem::transmute(&base.cef_object)
        }
    }
}

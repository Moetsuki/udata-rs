use std::sync::{Arc, Mutex};

use crate::helpers::LimitString;
use cef::sys::cef_response_filter_status_t::{
    RESPONSE_FILTER_DONE, RESPONSE_FILTER_NEED_MORE_DATA,
};
use cef::sys::cef_return_value_t;
use cef::{
    CefString, CefStringUtf16, ImplBrowser, ImplFrame, ImplRequest, ImplRequestHandler,
    ImplResourceRequestHandler, ImplResponse, ImplResponseFilter, RequestHandler,
    ResourceRequestHandler, ResourceType, ResponseFilter, ResponseFilterStatus, UrlrequestStatus,
    WrapRequestHandler, WrapResourceRequestHandler, WrapResponseFilter,
    rc::{Rc, RcImpl},
    sys,
};

use cef::{CefStringMultimap, ImplCallback, ReturnValue};

//
// RequestHandler
//

/// A custom implementation of `RequestHandler` for handling requests.
pub struct DemoRequestHandler(*mut RcImpl<sys::_cef_request_handler_t, Self>);

impl DemoRequestHandler {
    /// Creates a new instance of `DemoRequestHandler`.
    pub(crate) fn new() -> RequestHandler {
        RequestHandler::new(Self(std::ptr::null_mut()))
    }
}

impl ImplRequestHandler for DemoRequestHandler {
    /// Provides a resource request handler for specific requests.
    ///
    /// # Parameters
    /// - `_browser`: The browser instance.
    /// - `_frame`: The frame instance.
    /// - `_request`: The request being processed.
    /// - `_is_navigation`: Indicates if the request is a navigation.
    /// - `_is_download`: Indicates if the request is a download.
    /// - `_request_initiator`: The origin of the request.
    /// - `_disable_default_handling`: Optionally disables default handling.
    ///
    /// # Returns
    /// An optional `ResourceRequestHandler` instance.
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
        let request = _request.unwrap();

        // Check if its an XHR request
        if request.get_resource_type() == ResourceType::from(sys::cef_resource_type_t::RT_XHR) {
            let cache_control_str = CefString::from("Cache-Control");
            let pragma_str = CefString::from("Pragma");
            let expires_str = CefString::from("Expires");

            let cache_control_val = CefString::from("no-cache, no-store, must-revalidate");
            let pragma_val = CefString::from("no-cache");
            let expires_val = CefString::from("0");

            // Verify the header was set correctly
            let mut map = CefStringMultimap::new().unwrap();
            request.get_header_map(Some(&mut map));

            map.append(&cache_control_str, &cache_control_val);
            map.append(&pragma_str, &pragma_val);
            map.append(&expires_str, &expires_val);

            request.set_header_map(Some(&mut map));

            let mut map2 = CefStringMultimap::new().unwrap();
            request.get_header_map(Some(&mut map2));

            eprintln!(">> {:?}", map2);

            let url = CefStringUtf16::from(&request.get_url());
            eprintln!(">>       | URL:  {:}", url.to_string().limit(120));
            eprintln!(">>       | Mime: {:?}", request.get_resource_type());

            if let Some(request_initiator) = _request_initiator {
                eprintln!(">>       | Request initiator: {}", request_initiator);
            }

            eprintln!(">>       | Is navigation: {:}", _is_navigation);

            eprintln!(">>       | Is download: {:}", _is_download);
            eprintln!(">>       | Found XHR request");
            return Some(DemoResourceRequestHandler::new());
        }

        if _is_download == 0 && _is_navigation == 0 {
            Some(DemoResourceRequestHandler::new())
        } else {
            None
        }
    }

    /// Returns the raw pointer to the underlying CEF request handler.
    fn get_raw(&self) -> *mut sys::_cef_request_handler_t {
        self.0 as *mut sys::_cef_request_handler_t
    }
}

//
// ResourceHandler
//

/// A custom implementation of `ResourceRequestHandler` for handling resource requests.
pub struct DemoResourceRequestHandler(*mut RcImpl<sys::_cef_resource_request_handler_t, Self>);

impl DemoResourceRequestHandler {
    /// Creates a new instance of `DemoResourceRequestHandler`.
    fn new() -> ResourceRequestHandler {
        ResourceRequestHandler::new(Self(std::ptr::null_mut()))
    }
}

impl ImplResourceRequestHandler for DemoResourceRequestHandler {
    /// Called before a resource is loaded.
    ///
    /// # Parameters
    /// - `_browser`: The browser instance.
    /// - `_frame`: The frame instance.
    /// - `_request`: The request being processed.
    /// - `_callback`: A callback for asynchronous handling.
    ///
    /// # Returns
    /// A `ReturnValue` indicating how the resource load should proceed.
    fn on_before_resource_load(
        &self,
        _browser: Option<&mut impl ImplBrowser>,
        _frame: Option<&mut impl ImplFrame>,
        _request: Option<&mut impl ImplRequest>,
        _callback: Option<&mut impl ImplCallback>,
    ) -> ReturnValue {
        ReturnValue::from(cef_return_value_t::RV_CONTINUE)
    }

    /// Called when a resource response is received.
    ///
    /// # Parameters
    /// - `_browser`: The browser instance.
    /// - `_frame`: The frame instance.
    /// - `_request`: The request being processed.
    /// - `_response`: The response received.
    ///
    /// # Returns
    /// An integer indicating the result of the response handling.
    fn on_resource_response(
        &self,
        _browser: Option<&mut impl ImplBrowser>,
        _frame: Option<&mut impl ImplFrame>,
        _request: Option<&mut impl ImplRequest>,
        _response: Option<&mut impl ImplResponse>,
    ) -> ::std::os::raw::c_int {
        Default::default()
    }

    /// Provides a response filter for modifying resource responses.
    ///
    /// # Parameters
    /// - `_browser`: The browser instance.
    /// - `_frame`: The frame instance.
    /// - `_request`: The request being processed.
    /// - `_response`: The response received.
    ///
    /// # Returns
    /// An optional `ResponseFilter` instance.
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

        let mut headers = CefStringMultimap::new().unwrap();

        if let Some(req) = _request.as_ref() { req.get_header_map(Some(&mut headers)); }
        
        let url = {
            if let Some(req) = _request.as_ref() {
                let uri = req.get_url();
                CefString::from(&uri).to_string()
            } else {
                String::from("")
            }
        };

       Some(DemoResponseFilter::new(headers, url))
    }

    /// Called when a resource load is complete.
    ///
    /// # Parameters
    /// - `_browser`: The browser instance.
    /// - `_frame`: The frame instance.
    /// - `_request`: The request being processed.
    /// - `_response`: The response received.
    /// - `_status`: The status of the request.
    /// - `_received_content_length`: The length of the content received.
    fn on_resource_load_complete(
        &self,
        _browser: Option<&mut impl ImplBrowser>,
        _frame: Option<&mut impl ImplFrame>,
        _request: Option<&mut impl ImplRequest>,
        _response: Option<&mut impl ImplResponse>,
        _status: UrlrequestStatus,
        _received_content_length: i64,
    ) {
    }

    /// Returns the raw pointer to the underlying CEF resource request handler.
    fn get_raw(&self) -> *mut sys::_cef_resource_request_handler_t {
        self.0 as *mut sys::_cef_resource_request_handler_t
    }
}

//
// ResponseFilter
//

/// A custom implementation of `ResponseFilter` for filtering response data.
pub struct DemoResponseFilter {
    object: *mut RcImpl<sys::_cef_response_filter_t, Self>,
    buffer: Arc<Mutex<Vec<u8>>>,
    request_headers: CefStringMultimap,
    url: String,
}

impl DemoResponseFilter {
    /// Creates a new instance of `DemoResponseFilter`.
    ///
    /// # Parameters
    /// - `request_headers`: The headers of the request.
    /// - `url`: The URL of the request.
    ///
    /// # Returns
    /// A new `ResponseFilter` instance.
    fn new(request_headers: CefStringMultimap, url: String) -> ResponseFilter {
        ResponseFilter::new(Self {
            object: std::ptr::null_mut(),
            buffer: Arc::new(Mutex::new(Vec::new())),
            request_headers,
            url,
        })
    }
}

impl ImplResponseFilter for DemoResponseFilter {
    /// Initializes the filter.
    ///
    /// # Returns
    /// An integer indicating success (1) or failure (0).
    fn init_filter(&self) -> ::std::os::raw::c_int {
        let mut buffer = self.buffer.lock().unwrap();
        buffer.clear();
        1 // Return true to indicate success
    }

    /// Filters the response data.
    ///
    /// # Parameters
    /// - `_data_in`: The input data.
    /// - `_data_in_read`: The amount of input data read.
    /// - `_data_out`: The output data.
    /// - `_data_out_written`: The amount of output data written.
    ///
    /// # Returns
    /// A `ResponseFilterStatus` indicating the filter status.
    fn filter(
        &self,
        _data_in: Option<&mut Vec<u8>>,
        _data_in_read: Option<&mut usize>,
        _data_out: Option<&mut Vec<u8>>,
        _data_out_written: Option<&mut usize>,
    ) -> ResponseFilterStatus {
        eprintln!(">>       +------+ FILTA FILTA FILTAAAAA");
        eprintln!(">>              | URL:  {:}", self.url.limit(120));

        if _data_in.is_none() {
            return ResponseFilterStatus::from(RESPONSE_FILTER_DONE);
        }

        let data_in = _data_in.unwrap();
        let data_out = _data_out.unwrap();
        let data_in_read = _data_in_read.unwrap();
        let data_out_written = _data_out_written.unwrap();

        data_out[..data_in.len()].copy_from_slice(&data_in[..]);

        *data_in_read = data_in.len();
        *data_out_written = data_in.len();

        eprintln!("Error filtering data");
        ResponseFilterStatus::from(RESPONSE_FILTER_NEED_MORE_DATA)
    }

    /// Returns the raw pointer to the underlying CEF response filter.
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
            request_headers: self.request_headers.clone(),
            url: self.url.clone(),
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


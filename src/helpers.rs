pub trait LimitString {
    fn limit(&self, limit: usize) -> String;
}

impl LimitString for String {
    fn limit(&self, limit: usize) -> String {
        if self.len() > limit {
            format!("{}...", &self[..limit])
        } else {
            self.clone()
        }
    }
}

pub fn fmt_cef_string_utf16_t(s: &cef::sys::_cef_string_utf16_t) -> String {
    let slice = unsafe { std::slice::from_raw_parts(s.str_, s.length) };
    String::from_utf16_lossy(slice)
}

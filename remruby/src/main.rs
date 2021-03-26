use libc::c_char;
use std::env::args;
use std::ffi::CString;

enum MrbState {}

#[link(name = "mruby", kind = "static")]
extern "C" {
    fn mrb_open() -> *mut MrbState;
    fn mrb_load_string(mrb: *mut MrbState, code: *const c_char);
    fn mrb_close(mrb: *mut MrbState);
}

fn main() {
    let code = args().nth(2);
    if let Some(c) = code {
        let bytes = c.into_bytes();
        let code = CString::new(bytes).unwrap();

        unsafe {
            let mrb = mrb_open();
            mrb_load_string(mrb, code.as_ptr());
            mrb_close(mrb);
        }
    } else {
        println!("Usage: remruby -e CODE");
    }
}

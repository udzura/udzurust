use std::env::args;
use std::ffi::CString;
use libc::c_char;

#[repr(C)]
struct mrb_state_s;
type mrb_state = *mut mrb_state_s;

#[link(name="mruby", kind="static")]
extern {
    fn mrb_open() -> mrb_state;
    fn mrb_load_string(mrb: mrb_state, code: *const c_char) -> ();
    fn mrb_close(mrb: mrb_state) -> ();
}

fn main() {
    let code = args().nth(2);
    if let Some(c) = code {
        let bytes = c.into_bytes();

        unsafe {
            let mut mrb = mrb_open();
            mrb_load_string(mrb, CString::new(bytes).unwrap().as_ptr());
            mrb_close(mrb);
        }
    } else {
        println!("Usage: remruby -e CODE");
    }
}

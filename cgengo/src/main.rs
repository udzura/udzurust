#[link(name="hoge", kind="static")]
extern {
    fn hoge_hoge();
}

fn main() {
    println!("From Rust!!");
    unsafe {
        hoge_hoge();
    }
}

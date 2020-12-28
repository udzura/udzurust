fn main() {
    cc::Build::new()
        .file("c-src/hoge.c")
        .compile("libhoge.a");
}

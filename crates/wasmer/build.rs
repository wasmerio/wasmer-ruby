fn main() {
    // To satisfy `rutie-test`'s macros.
    println!(
        "cargo:rustc-env=TARGET={}",
        std::env::var("TARGET").unwrap()
    );
}

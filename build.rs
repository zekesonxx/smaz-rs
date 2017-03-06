#[cfg(feature = "cbinding")]
extern crate gcc;

#[cfg(feature = "cbinding")]
fn main() {
    gcc::compile_library("libsmaz.a", &["src/smaz.c"]);
}

#[cfg(not(feature = "cbinding"))]
fn main() {}

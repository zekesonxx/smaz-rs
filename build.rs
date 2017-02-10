extern crate gcc;

fn main() {
    gcc::compile_library("libsmaz.a", &["src/smaz.c"]);
}

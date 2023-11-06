extern crate cc;

use std::env;
use std::path::PathBuf;

fn main() {
    cc::Build::new()
        .cpp(true)
        .warnings(false)
        .flag("-std=c++17")
        .file("src/c/binding.cc")
        .include("voyager/cpp")
        .compile("binding");
}
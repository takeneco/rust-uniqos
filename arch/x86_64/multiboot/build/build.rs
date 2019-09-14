/// Build script for multiboot.

extern crate cc;

use std::env;

fn dumpenv()
{
    for (key, val) in env::vars() {
        eprintln!("{}={}", key, val);
    }
}

fn main()
{
    dumpenv();

    let out_dir = env::var("OUT_DIR").unwrap();

    let mut cfg = cc::Build::new();
    cfg
    .out_dir(&out_dir)
    .flag("-m32")
    .include("external/multiboot")
    .file("asm/start.S")
    .file("asm/mb2_header.S")
    .compile("mb");

    println!("cargo:rustc-link-search=native={}", String::from(out_dir));
    println!("cargo:rustc-link-lib=static=mb");
    println!("cargo:rustc-link-search=all=target/sysroot/lib/rustlib/i686-uniqos/lib");
}

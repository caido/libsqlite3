use std::env;

fn main() {
    println!("cargo:rerun-if-changed=sqlite3/sha1.c");
    cc::Build::new()
        .include(env::var_os("DEP_SQLITE3_LIB_DIR").expect("Missing DEP_SQLITE3_LIB_DIR"))
        .file("sqlite3/sha1.c")
        .compile("sha1");
}

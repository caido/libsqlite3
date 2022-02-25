fn main() {
    println!("cargo:rerun-if-changed=sqlite3/ext/sha1.c");
    println!("cargo:rerun-if-changed=sqlite3/include/sqlite3.h");
    println!("cargo:rerun-if-changed=sqlite3/include/sqlite3ext.h");
    cc::Build::new()
        .include("sqlite3/include")
        .file("sqlite3/ext/sha1.c")
        .compile("sqlite3ext");
}

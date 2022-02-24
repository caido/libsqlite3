fn main() {
    println!("cargo:rerun-if-changed=sqlite3/sha1.c");
    cc::Build::new().file("sqlite3/sha1.c").compile("sha1");
}

fn main() {
    println!("cargo:rerun-if-changed=src/weak_on_trace.c");
    cc::Build::new()
        .file("src/weak_on_trace.c")
        .compile("weak_on_trace");
}

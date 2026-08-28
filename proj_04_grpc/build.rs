// build.rs — generates Rust code from the .proto at build time.
// prost-build needs a `protoc` binary. Instead of requiring a system install,
// we use the prebuilt one from `protoc-bin-vendored` and point PROTOC at it.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let protoc = protoc_bin_vendored::protoc_bin_path()?;
    // SAFETY: single-threaded build script; setting an env var for this process.
    unsafe {
        std::env::set_var("PROTOC", protoc);
    }
    tonic_prost_build::compile_protos("proto/greeter.proto")?;
    Ok(())
}

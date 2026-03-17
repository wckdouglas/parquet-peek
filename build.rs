fn main() {
    // On macOS, conda's libiconv uses @rpath install_name.
    // Add /usr/lib to rpath so it resolves from the dyld shared cache.
    #[cfg(target_os = "macos")]
    println!("cargo:rustc-link-arg=-Wl,-rpath,/usr/lib");
}

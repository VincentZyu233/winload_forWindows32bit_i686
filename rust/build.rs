fn main() {
    let target = std::env::var("TARGET").unwrap();
    println!("cargo:rustc-env=TARGET={target}");

    // Windows MSVC + npcap feature: delay-load wpcap.dll
    // Without this, the binary fails to start on machines without Npcap installed,
    // because the OS loader cannot find wpcap.dll at process startup.
    // With /DELAYLOAD, wpcap.dll is only loaded when pcap functions are first called
    // (i.e., only when --npcap flag is used at runtime).
    if target.contains("windows-msvc") && cfg!(feature = "npcap") {
        println!("cargo:rustc-link-lib=delayimp");
        println!("cargo:rustc-link-arg=/DELAYLOAD:wpcap.dll");
    }

    // Note: xp_shim.c (MinGW GetFileInformationByHandleEx compat) was removed
    // as Rust 1.77+ requires Windows 10+ for all targets.
}

fn main() {
    let target = std::env::var("TARGET").unwrap();
    println!("cargo:rustc-env=TARGET={target}");

    // Windows + npcap feature: delay-load wpcap.dll
    // Without this, the binary fails to start on machines without Npcap installed,
    // because the OS loader cannot find wpcap.dll at process startup.
    // With /DELAYLOAD, wpcap.dll is only loaded when pcap functions are first called
    // (i.e., only when --npcap flag is used at runtime).
    if target.contains("windows") && cfg!(feature = "npcap") {
        println!("cargo:rustc-link-lib=delayimp");
        println!("cargo:rustc-link-arg=/DELAYLOAD:wpcap.dll");
    }

    // MinGW XP compat shim: provide GetFileInformationByHandleEx at link time
    // to avoid Vista+ PE import table entries. Falls back to GetProcAddress at runtime.
    if target.contains("windows-gnu") {
        cc::Build::new()
            .file("src/xp_shim.c")
            .compile("xp_shim");
    }
}

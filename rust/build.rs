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
    // Uses gcc directly (no cc crate) because the niXman MinGW builds lack `ar`.
    if target.contains("windows-gnu") {
        let cc_var = format!("CC_{}", target.replace('-', "_"));
        let cc = std::env::var(&cc_var)
            .or_else(|_| std::env::var("CC"))
            .unwrap_or_else(|_| "gcc".to_string());

        let out_dir = std::env::var("OUT_DIR").unwrap();
        let obj = format!("{}/xp_shim.o", out_dir);

        let status = std::process::Command::new(&cc)
            .args(["-c", "-O2", "-ffunction-sections", "src/xp_shim.c", "-o", &obj])
            .status()
            .expect("failed to compile xp_shim.c");
        assert!(status.success(), "xp_shim.c compilation failed");

        // Allow overriding the winapi import library definition
        println!("cargo:rustc-link-arg=-Wl,--allow-multiple-definition");
        println!("cargo:rustc-link-arg={}", obj);
    }
}

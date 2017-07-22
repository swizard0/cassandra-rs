fn main() {
    if let Some(datastax_dir) = option_env!("CASSANDRA_SYS_LIB_PATH") {
        for p in datastax_dir.split(";") {
            println!("cargo:rustc-link-search={}", p);
        }
    }
    if cfg!(target_os = "macos") {
        println!("cargo:rustc-link-search={}", "/opt/local/lib");
    }
    println!("cargo:rustc-flags=-l dylib=crypto");
    println!("cargo:rustc-flags=-l dylib=ssl");
    if cfg!(target_os = "macos") {
        println!("cargo:rustc-flags=-l dylib=c++");
    } else {
        println!("cargo:rustc-flags=-l dylib=stdc++");
    }
    println!("cargo:rustc-flags=-l dylib=uv");
    println!("cargo:rustc-link-search={}", "/usr/lib/x86_64-linux-gnu");
    println!("cargo:rustc-link-search={}", "/usr/local/lib/x86_64-linux-gnu");
    println!("cargo:rustc-link-search={}", "/usr/local/lib64");
    println!("cargo:rustc-link-search={}", "/usr/local/lib");
    println!("cargo:rustc-link-search={}", "/usr/lib64/");
    println!("cargo:rustc-link-search={}", "/usr/lib/");
    println!("cargo:rustc-link-lib=static=cassandra_static");
}

use super::{generic, prelude::*};

pub struct Wasm32Wasip1Threads;

impl PlatformDetails for Wasm32Wasip1Threads {
    fn uses_freetype(&self, _config: &BuildConfiguration) -> bool {
        true
    }

    fn gn_args(&self, config: &BuildConfiguration, builder: &mut GnArgsBuilder) {
        let features = &config.features;

        generic::gn_args(config, builder);

        builder
            .arg("cc", quote(&format!("{}/bin/clang", wasi_sdk_base_dir())))
            .arg(
                "cxx",
                quote(&format!("{}/bin/clang++", wasi_sdk_base_dir())),
            )
            .arg("ar", quote(&format!("{}/bin/llvm-ar", wasi_sdk_base_dir())))
            .arg("skia_gl_standard", quote("webgl"))
            .arg("skia_use_webgl", yes_if(features.gpu()))
            .arg("skia_use_freetype_woff2", yes())
            .arg("target_cpu", quote("wasm"))
            // The custom embedded font manager is enabled by default on WASM, but depends
            // on the undefined symbol `SK_EMBEDDED_FONTS`. Enable the custom empty font
            // manager instead so typeface creation still works.
            // See https://github.com/rust-skia/rust-skia/issues/648
            .arg("skia_enable_fontmgr_custom_embedded", no())
            .arg("skia_enable_fontmgr_custom_empty", yes())
            .cflags(cflags());
    }

    fn bindgen_args(&self, _target: &Target, builder: &mut BindgenArgsBuilder) {
        builder.args(cflags());
    }

    fn link_libraries(&self, _features: &Features) -> Vec<String> {
        vec![
            format!("c++"),
            format!("c++abi"),
            format!("c++experimental"),
            format!("c-printscan-long-double"),
            format!("c-printscan-no-floating-point"),
            format!("c"),
            format!("crypt"),
            format!("dl"),
            format!("m"),
            format!("pthread"),
            format!("resolv"),
            format!("rt"),
            format!("setjmp"),
            format!("util"),
            format!("wasi-emulated-getpid"),
            format!("wasi-emulated-mman"),
            format!("wasi-emulated-process-clocks"),
            format!("wasi-emulated-signal"),
            format!("xnet"),
            // /opt/wasi-sdk/lib/clang/18/lib/wasip1/libclang_rt.builtins-wasm32.a
            format!("clang_rt.builtins-wasm32"),
        ]
    }
}

fn cflags() -> Vec<String> {
    format!(
        "
    -DSK_BUILD_FOR_UNIX
    -D__EMSCRIPTEN__

    -mllvm
    -wasm-enable-sjlj
    -mtail-call
    -D_WASI_EMULATED_MMAN
    -pthread

    -fvisibility=default

    -Xclang -target-feature -Xclang +atomics
    -Xclang -target-feature -Xclang +bulk-memory
    -Xclang -target-feature -Xclang +mutable-globals

    --sysroot=/{wasi_sdk_base_dir}/share/wasi-sysroot
    -I/{wasi_sdk_base_dir}/lib/clang/18/include
    -I{emsdk_system_include}
",
        emsdk_system_include = emsdk_system_include(),
        wasi_sdk_base_dir = wasi_sdk_base_dir(),
    )
    .split_whitespace()
    .map(|s| s.to_string())
    .collect()
}

fn emsdk_system_include() -> String {
    match std::env::var("EMSDK_SYSTEM_INCLUDE") {
        Ok(val) => val,
        Err(_e) => panic!(
            "please set the EMSDK_SYSTEM_INCLUDE environment variable to the {{emsdk}}/system/include directory"
        ),
    }
}

fn wasi_sdk_base_dir() -> String {
    match std::env::var("WASI_SDK") {
        Ok(val) => val,
        Err(_e) => {
            panic!("please set the WASI_SDK environment variable to the root of your wasi-sdk")
        }
    }
}

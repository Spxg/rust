use crate::spec::{Arch, Cc, Env, LinkerFlavor, Os, PanicStrategy, Target, TargetMetadata, base};

pub(crate) fn target() -> Target {
    let mut options = base::wasm::options();
    options.os = Os::Web;
    options.env = Env::Wabi;
    options.linker = Some("js-bindgen-ld".into());
    options.panic_strategy = PanicStrategy::Unwind;
    options.main_needs_argc_argv = true;

    options.add_pre_link_args(
        LinkerFlavor::WasmLld(Cc::No),
        &[
            // For now this target just never has an entry symbol no matter the output
            // type, so unconditionally pass this.
            "--no-entry",
            "-mwasm64",
            // `wabi` target_env
            "--wabi",
        ],
    );
    options.add_pre_link_args(
        LinkerFlavor::WasmLld(Cc::Yes),
        &[
            // Make sure clang uses LLD as its linker and is configured appropriately
            // otherwise
            "--target=wasm64-web-wabi",
            "-Wl,--no-entry",
            // `wabi` target_env
            "-Wl,--wabi",
        ],
    );

    // Any engine that implements wasm64 will surely implement the rest of these
    // features since they were all merged into the official spec by the time
    // wasm64 was designed.
    options.features = "+bulk-memory,+mutable-globals,+sign-ext,+nontrapping-fptoint".into();

    Target {
        llvm_target: "wasm64-unknown-unknown".into(),
        metadata: TargetMetadata {
            description: Some("WebAssembly".into()),
            tier: Some(2),
            host_tools: Some(false),
            std: Some(true),
        },
        pointer_width: 64,
        data_layout: "e-m:e-p:64:64-p10:8:8-p20:8:8-i64:64-i128:128-n32:64-S128-ni:1:10:20".into(),
        arch: Arch::Wasm64,
        options,
    }
}

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
            // `wabi` target_env
            "--wabi",
        ],
    );
    options.add_pre_link_args(
        LinkerFlavor::WasmLld(Cc::Yes),
        &[
            // Make sure clang uses LLD as its linker and is configured appropriately
            // otherwise
            "--target=wasm32-web-wabi",
            "-Wl,--no-entry",
            // `wabi` target_env
            "-Wl,--wabi",
        ],
    );

    Target {
        llvm_target: "wasm32-unknown-unknown".into(),
        metadata: TargetMetadata {
            description: Some("WebAssembly".into()),
            tier: Some(2),
            host_tools: Some(false),
            std: Some(true),
        },
        pointer_width: 32,
        data_layout: "e-m:e-p:32:32-p10:8:8-p20:8:8-i64:64-i128:128-n32:64-S128-ni:1:10:20".into(),
        arch: Arch::Wasm32,
        options,
    }
}

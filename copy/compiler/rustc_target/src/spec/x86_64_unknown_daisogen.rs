use crate::spec::{PanicStrategy, Target};

pub fn target() -> Target {
    let mut base = super::daisogen_base::opts();
    base.cpu = "x86-64".into();
    base.disable_redzone = true;
    base.panic_strategy = PanicStrategy::Abort;
    base.features = "-mmx,-sse,+soft-float".into();

    Target {
        llvm_target: "x86_64-unknown-none".into(),
        pointer_width: 64,
        data_layout: "e-m:e-i64:64-f80:128-n8:16:32:64-S128".into(),
        arch: "x86_64".into(),
        options: base,
    }
}

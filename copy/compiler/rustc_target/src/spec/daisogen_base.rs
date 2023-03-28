use crate::spec::TargetOptions;

pub fn opts() -> TargetOptions {
    TargetOptions {
        os: "daisogen".into(),
        executables: true,
        disable_redzone: true,
        has_thread_local: false,
        ..Default::default()
    }
}

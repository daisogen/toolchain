use crate::spec::TargetOptions;

pub fn opts() -> TargetOptions {
    TargetOptions {
        os: "daisogen".into(),
        executables: true,
        disable_redzone: true,
        ..Default::default()
    }
}

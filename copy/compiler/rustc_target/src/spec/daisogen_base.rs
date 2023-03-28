use crate::spec::TargetOptions;

pub fn opts() -> TargetOptions {
    TargetOptions { os: "daisogen".into(), executables: true, ..Default::default() }
}

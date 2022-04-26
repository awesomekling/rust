use crate::spec::{cvs, FramePointer, PanicStrategy, RelroLevel, TargetOptions, TlsModel};

pub fn opts() -> TargetOptions {
    TargetOptions {
        os: "serenity".into(),
        dynamic_linking: true,
        executables: true,
        tls_model: TlsModel::InitialExec,
        position_independent_executables: true,
        panic_strategy: PanicStrategy::Abort,
        families: cvs!["unix"],
        relro_level: RelroLevel::Full,
        frame_pointer: FramePointer::Always,
        ..Default::default()
    }
}

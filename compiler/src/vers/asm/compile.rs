use crate::PROJECT_NAME;
use std::fs::rename;
use std::process::{Command};

extern "C" { fn compileInternal() -> !; }
extern "C" { fn compileExternal() -> !; }
extern "C" { fn link(name: *const u8) -> !; }

pub(crate) unsafe fn compile_asm() {
    compileInternal();
    compileExternal();
    link(PROJECT_NAME.as_ptr());
}

use std::fs::File;
use std::io::Write;

pub(crate) use super::messages::errors::compiler_error;

pub(crate) fn il_asm(inline_asm: &str) -> String {
    let removed_name = inline_asm.replace("il_asm('", "");
    let remove_end_asm_fun = removed_name.replace("');", "");
    let asm = remove_end_asm_fun;
    let write_asm = format_args!("{}", asm).to_string();
    return write_asm
}


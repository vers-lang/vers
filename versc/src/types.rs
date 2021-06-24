#[derive(Copy, Clone, PartialEq)]
#[repr(C)]
pub(crate) enum Action {
    None,
    FunDec,
    VarDec,
}

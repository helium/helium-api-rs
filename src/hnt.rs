use serde::Serialize;

const HNT_TO_BONES_SCALAR: i32 = 100_000_000;
#[derive(Clone, Copy, Debug, Serialize)]
pub struct Hnt(Decimal);

decimal_bones!(Hnt, HNT_TO_BONES_SCALAR);

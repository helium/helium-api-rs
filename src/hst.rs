const HST_TO_BONES_SCALAR: i32 = 100_000_000;
#[derive(Clone, Copy, Debug)]
pub struct Hst(Decimal);

decimal_bones!(Hst, HST_TO_BONES_SCALAR);

use super::decimal_bones;

const ORACLE_PRICE_TO_BONES_SCALAR: i32 = 100_000_000;
#[derive(Clone, Copy, Debug)]
pub struct OraclePrice(Decimal);

decimal_bones!(OraclePrice, ORACLE_PRICE_TO_BONES_SCALAR);

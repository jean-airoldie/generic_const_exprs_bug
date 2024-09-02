use std::marker::PhantomData;

pub struct DecI64<const F: usize, R = HalfUp>(i64, PhantomData<R>)
where
    [u8; F]: FractionalDigits,
    R: Rounding;

impl FractionalDigits for [u8; 0] {}

trait FractionalDigits {}

impl<const F: usize, R> import::FromSlice for DecI64<F, R>
where
    [u8; F]: FractionalDigits,
    R: Rounding,
{
    fn validate_slice(_: &[[u8; Self::SIZE]]) -> Result<(), import::Error> {
        Ok(())
    }
}

trait Rounding {
    const MODE: RoundingMode;
}

struct HalfUp;

impl Rounding for HalfUp {
    const MODE: RoundingMode = RoundingMode::HalfUp;
}

enum RoundingMode {
    HalfUp,
}

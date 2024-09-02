struct Wrapper<const F: usize>(i64);

impl<const F: usize> import::FromSlice for Wrapper<F> {
    fn validate_slice(_: &[[u8; Self::SIZE]]) -> Result<(), import::Error> {
        Ok(())
    }
}

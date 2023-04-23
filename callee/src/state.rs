use crate::*;
use solana_program::{
    program_error::ProgramError,
    program_pack::{Pack, Sealed},
};

#[derive(Debug, Default)]
pub struct Counter {
    pub number: u32,
}

impl Sealed for Counter {}

impl Pack for Counter {
    const LEN: usize = 4;

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let number: u32;

        // Deserialize
        let new_src: &[u8] = src;
        (_, number) = rust_utils::unpack_u32(new_src);

        Ok(Counter { number })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let new_dst: &mut [u8] = dst;

        // Destructure self
        let Counter { number } = self;

        // Serialize each field
        rust_utils::pack_u32(new_dst, *number);
    }
}

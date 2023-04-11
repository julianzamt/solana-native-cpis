use crate::*;
use solana_program::{
    program_error::ProgramError,
    program_pack::{Pack, Sealed},
};

#[derive(Debug, Default)]
pub struct Adder {
    pub number: u8,
}

impl Sealed for Adder {}

impl Pack for Adder {
    const LEN: usize = 1;

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let number: u8;

        // Deserialize
        let new_src: &[u8] = src;
        (_, number) = rust_utils::unpack_u8(new_src);

        Ok(Adder { number })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let new_dst: &mut [u8] = dst;

        // Destructure self
        let Adder { number } = self;

        // Serialize each field
        rust_utils::pack_u8(new_dst, *number);
    }
}

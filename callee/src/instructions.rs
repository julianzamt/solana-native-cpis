use crate::*;
use solana_program::{msg, program_error::ProgramError};

#[derive(Debug)]
pub enum CounterInstruction {
    /// Accounts expected:
    ///
    /// 0. `[writable]` counter
    /// 1. `[]` signer
    /// 2. `[]` system_program
    Increase {
        number: u32,
    },
    /// Accounts expected:
    ///
    /// 0. `[]` counter
    LogCounter {},
}

impl CounterInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        msg!("Unpacking instruction...");

        let (tag, rest) = input
            .split_first()
            .ok_or(errors::AdderError::InvalidInstruction)?;

        Ok(match tag {
            0 => {

                let (_, number) = rust_utils::unpack_u32(rest);

                Self::Increase {number}
        },

            1 => Self::LogCounter {},

            _ => return Err(errors::AdderError::InvalidInstruction.into()),
        })
    }
}

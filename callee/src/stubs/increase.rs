use crate::state::*;
use solana_program::entrypoint::ProgramResult;

pub fn increase(number: u32, counter_data: &mut Counter) -> ProgramResult {
    // Place your custom code here...
    counter_data.number += number;
    Ok(())
}

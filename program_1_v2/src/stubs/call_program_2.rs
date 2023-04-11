use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, msg};

use crate::stubs::cpi_program_2;

pub fn call_program_2<'a>(
    program_2_account_info: &AccountInfo<'a>,
    program_3_account_info: &AccountInfo<'a>,
    counter_program_account_info: &AccountInfo<'a>,
    counter_account_info: &AccountInfo<'a>,
    signer_account_info: &AccountInfo<'a>,
    system_account_info: &AccountInfo<'a>,
) -> ProgramResult {
    // Place your custom code here...
    msg!("call_counter stub in program 3");

    cpi_program_2::cpi_program_2(
        program_2_account_info,
        program_3_account_info,
        counter_program_account_info,
        counter_account_info,
        signer_account_info,
        system_account_info,
    )
}
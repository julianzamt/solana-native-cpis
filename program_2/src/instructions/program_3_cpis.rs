use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    instruction::{AccountMeta, Instruction},
    msg,
    program::invoke,
};

use crate::instructions::rust_utils::*;

pub fn cpi_program_3<'a>(
    number: u8,
    program_3_account_info: &AccountInfo<'a>,
    add_program_account_info: &AccountInfo<'a>,
    adder_account_info: &AccountInfo<'a>,
    signer_account_info: &AccountInfo<'a>,
    system_account_info: &AccountInfo<'a>,
) -> ProgramResult {
    msg!("cpi to program_3 in program_2...");

    let buf: &mut [u8] = &mut [0; 1];

    // pack data
    pack_u8(buf, number);

    let b_ix = Instruction::new_with_bytes(
        *program_3_account_info.key,
        buf,
        [
            AccountMeta::new_readonly(*add_program_account_info.key, false),
            AccountMeta::new(*adder_account_info.key, false),
            AccountMeta::new(*signer_account_info.key, true),
            AccountMeta::new_readonly(*system_account_info.key, false),
        ]
        .to_vec(),
    );
    invoke(
        &b_ix,
        &[
            add_program_account_info.clone(),
            adder_account_info.clone(),
            signer_account_info.clone(),
            system_account_info.clone(),
        ],
    )?;
    Ok(())
}

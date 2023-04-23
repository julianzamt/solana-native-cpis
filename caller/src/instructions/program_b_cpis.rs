use core::num;

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    instruction::{AccountMeta, Instruction},
    msg,
    program::invoke,
    pubkey,
    pubkey::Pubkey,
};

use crate::instructions::rust_utils::*;

pub fn cpi_add<'a>(
    number: u32,
    _program_b_account_info: &AccountInfo<'a>,
    counter_account_info: &AccountInfo<'a>,
    signer_account_info: &AccountInfo<'a>,
    system_account_info: &AccountInfo<'a>,
) -> ProgramResult {
    msg!("cpi_add execution...");

    let buf: &mut [u8] = &mut [0; 5];
    let new_buf = pack_u8(buf, 0);

    // pack data
    pack_u32(new_buf, number);

    msg!("BUF en programb_cpis 0.1: {:?}", buf);

    let b_ix = Instruction::new_with_bytes(
        pubkey!("HyYjeFzX6evd8waHu9YmEWkJzxYX8ryAp2AnKF5jeeVJ"),
        buf,
        [
            AccountMeta::new(*counter_account_info.key, false),
            AccountMeta::new(*signer_account_info.key, true),
            AccountMeta::new_readonly(*system_account_info.key, false),
        ]
        .to_vec(),
    );
    invoke(
        &b_ix,
        &[
            _program_b_account_info.clone(),
            counter_account_info.clone(),
            signer_account_info.clone(),
            system_account_info.clone(),
        ],
    )?;
    Ok(())
}

pub fn cpi_log_counter<'a>(
    _program_b_account_info: &AccountInfo<'a>,
    counter_account_info: &AccountInfo<'a>,
) -> ProgramResult {
    msg!("cpi_log_counter execution...");

    let buf: &mut [u8] = &mut [0; 1];
    pack_u8(buf, 1);

    msg!("BUF en programb_cpis 1.0: {:?}", buf);

    let b_ix = Instruction::new_with_bytes(
        pubkey!("HyYjeFzX6evd8waHu9YmEWkJzxYX8ryAp2AnKF5jeeVJ"),
        buf,
        [
            AccountMeta::new_readonly(*counter_account_info.key, false),
        ]
        .to_vec(),
    );
    invoke(
        &b_ix,
        &[
            _program_b_account_info.clone(),
            counter_account_info.clone(),
        ],
    )?;
    Ok(())
}

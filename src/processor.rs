use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};
use spl_token::state::Account as TokenAccount;

#[derive(Debug)]
pub struct MemCoin;

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Processing MemCoin transaction");

    let accounts_iter = &mut accounts.iter();
    let sender = next_account_info(accounts_iter)?;
    let receiver = next_account_info(accounts_iter)?;
    let token_account = next_account_info(accounts_iter)?;

    if !sender.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let amount = u64::from_le_bytes(
        instruction_data
            .try_into()
            .map_err(|_| ProgramError::InvalidInstructionData)?,
    );

    let mut token_data = TokenAccount::unpack(&token_account.try_borrow_data()?)?;

    if token_data.amount < amount {
        return Err(ProgramError::InsufficientFunds);
    }

    token_data.amount -= amount;
    TokenAccount::pack(token_data, &mut token_account.try_borrow_mut_data()?)?;

    msg!("Transferred {} MemCoins", amount);
    Ok(())
}

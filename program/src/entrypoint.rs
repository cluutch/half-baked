use {
  solana_program::{
      account_info::{next_account_info, AccountInfo}, entrypoint,
      entrypoint::ProgramResult, msg, pubkey::Pubkey, 
      program_error::ProgramError,
      program_pack::{IsInitialized, Pack, Sealed},
  },
};
use borsh::{BorshDeserialize, BorshSerialize};

use std::str::from_utf8;

entrypoint!(process_instruction);

pub fn process_instruction(
  _program_id: &Pubkey,
  accounts: &[AccountInfo], // 1. metdata account, 2. api data account
  instruction_data: &[u8],
) -> ProgramResult {
  msg!("Greetings from the Half Baked oracle. Remember, there is no spoon.");

  let accounts_iter = &mut accounts.iter();
  let meta_data_account = next_account_info(accounts_iter)?;
  let api_data_account = next_account_info(accounts_iter)?;

  let endpoint_url_https = from_utf8(&instruction_data[0..99]).map_err(|err| {
      msg!("Invalid UTF-8, for endpointUrlHttps {}", err.valid_up_to());
      ProgramError::InvalidInstructionData
  })?;

  let description = from_utf8(&instruction_data[100..199]).map_err(|err| {
      msg!("Invalid UTF-8, for description {}", err.valid_up_to());
      ProgramError::InvalidInstructionData
  })?;
  msg!("Metadata goes in: {}", meta_data_account.key);

  // let all_api_pubkeys = &meta_data_account.try_borrow_mut_data()?;
  let mut all_api_pubkeys = meta_data_account.data.borrow_mut();
  let api_pubkey = Pubkey::new(&all_api_pubkeys[0..32]);
  
  msg!("Metadata contains: {}", api_pubkey);
  msg!("Oracle for: {}", description);
  msg!("Channeling the spirit of: {}", endpoint_url_https);
  msg!("API data account: {}", api_data_account.key);

  // TODO ENSURE WRITING TO CORRECT ACCOUNT
  // TODO ENSURE PROGRAM IS THE OWNER OF WHERE WRITING TO

  // write_data(api_data_account, &meta_data_account.key.to_bytes(), 0);
  all_api_pubkeys[0..32].copy_from_slice(&api_data_account.key.to_bytes());


  let mut api_data = api_data_account.data.borrow_mut();
  let api_data_description = from_utf8(&api_data[0..99]).map_err(|err| {
    msg!("Invalid UTF-8, for previous description {}", err.valid_up_to());
    ProgramError::InvalidInstructionData
  })?;  
  msg!("Prevous val: {} to API data account", api_data_description);
  api_data[0..99].copy_from_slice(&instruction_data[100..199]);
  msg!("Wrote description: {} to API data account", description);

  Ok(())
}

pub fn write_data(account: &AccountInfo, input: &[u8], offset: usize) {
  let mut account_data = account.data.borrow_mut();
  account_data[offset..offset + input.len()].copy_from_slice(input);
}


#[cfg(test)]
mod test {
    use {
        super::*,
        solana_program_test::*,
    };

    #[tokio::test]
    async fn test_transaction() {
      msg!("arstarst323prastarstarstStarting test");
      let endpoint_url_https = "api.cluutch.io/v2/daily?date=2021-06-05";
      let description = "api.cluutch.io/v2/daily?date=2021-06-05";

      let program_id = Pubkey::default();
      let accounts: Vec<AccountInfo> = vec![];

      let endpoint_url_https_bytes = endpoint_url_https.as_bytes();
      if endpoint_url_https_bytes.len() > 100 {
        panic!("Endpoint is too long! {}", endpoint_url_https);
      }

      let mut endpoint_url_https_bytes_padded: Vec<u8> = endpoint_url_https_bytes.to_vec();
      endpoint_url_https_bytes_padded.resize(100, 0);


      let description_bytes = description.as_bytes();
      if description_bytes.len() > 100 {
        panic!("Endpoint is too long! {}", description);
      }

      let mut description_bytes_padded: Vec<u8> = description_bytes.to_vec();
      description_bytes_padded.resize(100, 0);

      endpoint_url_https_bytes_padded.append(&mut description_bytes_padded);

      assert_eq!(
        Ok(()), 
        process_instruction(&program_id, &accounts, &endpoint_url_https_bytes_padded));
    }
}

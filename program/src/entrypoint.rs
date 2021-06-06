use {
  solana_program::{
      account_info::AccountInfo, entrypoint,
      entrypoint::ProgramResult, msg, pubkey::Pubkey, 
      program_error::ProgramError,
  },
};

use std::str::from_utf8;

entrypoint!(process_instruction);

pub fn process_instruction(
  _program_id: &Pubkey,
  _accounts: &[AccountInfo],
  instruction_data: &[u8],
) -> ProgramResult {
  msg!("Greetings from the Half Baked oracle. Remember, there is no spoon.");

  let endpoint_url_https = from_utf8(&instruction_data[0..99]).map_err(|err| {
      msg!("Invalid UTF-8, for endpointUrlHttps {}", err.valid_up_to());
      ProgramError::InvalidInstructionData
  })?;

  let description = from_utf8(&instruction_data[100..199]).map_err(|err| {
      msg!("Invalid UTF-8, for description {}", err.valid_up_to());
      ProgramError::InvalidInstructionData
  })?;

  // msg!("I see the future at {}", endpoint_url_https);
  msg!("Oracle for: {}", description);
  msg!("Channeling the spirit of: {}", endpoint_url_https);

  Ok(())
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

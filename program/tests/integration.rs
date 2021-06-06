#![cfg(feature = "test-bpf")]
use solana_program_test::*;

use {
    assert_matches::*,
    solana_program::{
        account_info::AccountInfo,
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
    },
    solana_sdk::{signature::Signer, transaction::Transaction},
    solana_validator::test_validator::*,
};

#[test]
fn test_validator_transaction() {
    let program_id = Pubkey::new_unique();

    let (test_validator, payer) = TestValidatorGenesis::default()
        .add_program("half_baked", program_id)
        .start();
    let (rpc_client, recent_blockhash, _fee_calculator) = test_validator.rpc_client();


    // let endpoint_url_https = "api.cluutch.io/v2/daily?date=2021-06-05";
    // let description = "api.cluutch.io/v2/daily?date=2021-06-05";

    let program_id = Pubkey::default();
    // let accounts: Vec<AccountInfo> = vec![AccountMeta::new(payer.pubkey(), false)];
    // // let instruction_data = [0; 200];

    // let endpoint_url_https_bytes = endpoint_url_https.as_bytes();
    // if endpoint_url_https_bytes.len() > 100 {
    //   panic!("Endpoint is too long! {}", endpoint_url_https);
    // }

    // let mut endpoint_url_https_bytes_padded: Vec<u8> = endpoint_url_https_bytes.to_vec();
    // endpoint_url_https_bytes_padded.resize(100, 0);


    // let description_bytes = description.as_bytes();
    // if description_bytes.len() > 100 {
    //   panic!("Endpoint is too long! {}", description);
    // }

    // let mut description_bytes_padded: Vec<u8> = description_bytes.to_vec();
    // description_bytes_padded.resize(100, 0);

    // endpoint_url_https_bytes_padded.append(&mut description_bytes_padded);
    let burn: Vec<u8> = [0; 100].to_vec();

    let mut transaction = Transaction::new_with_payer(
        &[Instruction {
            program_id,
            accounts: vec![AccountMeta::new(payer.pubkey(), false)],
            data: burn,
        }],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer], recent_blockhash);

    assert_matches!(rpc_client.send_and_confirm_transaction(&transaction), Ok(_));
}

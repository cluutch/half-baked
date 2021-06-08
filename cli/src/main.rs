use byteorder::{ByteOrder, LittleEndian};
use solana_sdk::instruction::Instruction;
use solana_sdk::instruction::AccountMeta;
use solana_sdk::pubkey::Pubkey;
use core::str::FromStr;

use {
    clap::{crate_description, crate_name, crate_version, App, AppSettings, Arg, SubCommand},
    solana_clap_utils::{
        input_parsers::pubkey_of,
        input_validators::{is_url, is_valid_pubkey, is_valid_signer},
        keypair::DefaultSigner,
    },
    solana_client::rpc_client::RpcClient,
    solana_remote_wallet::remote_wallet::RemoteWalletManager,
    solana_sdk::{
        commitment_config::CommitmentConfig,
        message::Message,
        native_token::Sol,
        signature::{Signature, Signer},
        system_instruction,
        transaction::Transaction,
    },
    std::{process::exit, sync::Arc},
};

use clokwerk::{Scheduler, TimeUnits};
use std::thread;
use std::time::Duration;

use std::collections::HashMap;

struct Config {
    commitment_config: CommitmentConfig,
    default_signer: Box<dyn Signer>,
    json_rpc_url: String,
    verbose: bool,
}
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Quote {
    date: String,
    avg_price_per_ounce: String,
    jurisdiction: String
}
fn process_api_payload(
    rpc_client: &RpcClient,
    signer: &dyn Signer,
    commitment_config: CommitmentConfig,
) -> Result<Signature, Box<dyn std::error::Error>> {
    println!("Starting to process string");
    let program_id = Pubkey::from_str("HS4DFksToksNYF3fyU8Rxmg2B7sAq5JYhHKzitaVbDCp")?;
    let meta_data_pub = Pubkey::from_str("52iJqwHvBsUhEaNg3wJJLzNSBzksaVC6kFSV18w215ZJ")?;
    let cluutch_data_pub = Pubkey::from_str("EtmqeZo4r1pFXonVCegK8S5YwcnmG9eEXKj4BVbR5Sun")?;
    let from = signer.pubkey();
    println!("From ID: {}", from);
    println!("Program ID: {}", program_id);


    let endpoint_url_https = "api.cluutch.io/v2/daily?date=2021-06-05";
    let description = "Price for 1oz of weed";
    let value = "$159.00";
    let json_value_path = "price";
    let img_url_https = "cluutch.io/clutch.svg";

    // endpoint_url_https
    let endpoint_url_https_bytes = endpoint_url_https.as_bytes();
    if endpoint_url_https_bytes.len() > 100 {
      panic!("Endpoint is too long! {}", endpoint_url_https);
    }
    let mut endpoint_url_https_bytes_padded: Vec<u8> = endpoint_url_https_bytes.to_vec();
    endpoint_url_https_bytes_padded.resize(100, 0);

    // description
    let description_bytes = description.as_bytes();
    if description_bytes.len() > 100 {
      panic!("Description is too long! {}", description);
    }
    let mut description_bytes_padded: Vec<u8> = description_bytes.to_vec();
    description_bytes_padded.resize(100, 0);

    // value
    let value_bytes = value.as_bytes();
    if value_bytes.len() > 100 {
      panic!("Value is too long! {}", value);
    }
    let mut value_bytes_padded: Vec<u8> = value_bytes.to_vec();
    value_bytes_padded.resize(100, 0);

    // json_value_path
    let json_value_path_bytes = json_value_path.as_bytes();
    if json_value_path_bytes.len() > 100 {
      panic!("json_value_path is too long! {}", json_value_path);
    }
    let mut json_value_path_bytes_padded: Vec<u8> = json_value_path_bytes.to_vec();
    json_value_path_bytes_padded.resize(100, 0);

    // img_url_https
    let img_url_https_bytes = img_url_https.as_bytes();
    if img_url_https_bytes.len() > 100 {
      panic!("img_url_https is too long! {}", img_url_https);
    }
    let mut img_url_https_bytes_padded: Vec<u8> = img_url_https_bytes.to_vec();
    img_url_https_bytes_padded.resize(100, 0);

    endpoint_url_https_bytes_padded.append(&mut description_bytes_padded);
    endpoint_url_https_bytes_padded.append(&mut value_bytes_padded);
    endpoint_url_https_bytes_padded.append(&mut json_value_path_bytes_padded);
    endpoint_url_https_bytes_padded.append(&mut img_url_https_bytes_padded);

    println!("Sending {} bytes", endpoint_url_https_bytes_padded.len());
   
    let mut transaction = Transaction::new_with_payer(
        &[Instruction::new_with_bytes(
            program_id,
            &endpoint_url_https_bytes_padded,
            vec![AccountMeta::new(meta_data_pub, false), AccountMeta::new(cluutch_data_pub, false)],
        )],
        Some(&from),
    );
    println!("Constructed transaction");

    let (recent_blockhash, _fee_calculator) = rpc_client
        .get_recent_blockhash()
        .map_err(|err| format!("error: unable to get recent blockhash: {}", err))?;
    println!("Got recent blockhash");

    transaction
        .try_sign(&vec![signer], recent_blockhash)
        .map_err(|err| format!("error: failed to sign transaction: {}", err))?;
    println!("Signed transaction");

    let signature = rpc_client
        .send_and_confirm_transaction_with_spinner_and_commitment(&transaction, commitment_config)
        .map_err(|err| format!("error: send transaction: {}", err))?;
    println!("Processed transaction");

    println!("{}", signature);
    Ok(signature)
}

fn create_metadata_account(
    rpc_client: &RpcClient,
    signer: &dyn Signer,
    commitment_config: CommitmentConfig,
) -> Result<Signature, Box<dyn std::error::Error>> {
    let seed = "half-baked-meta-data";
    let program_id = Pubkey::from_str("HS4DFksToksNYF3fyU8Rxmg2B7sAq5JYhHKzitaVbDCp")?;
    let data_num_bytes = 3200;
    let data_lamports = 11235813;

    let signer_pub = signer.pubkey();
    println!("Starting to create meta data account from {} with owner {}", signer_pub, program_id);

    let data_account_pubkey = Pubkey::create_with_seed(&signer.pubkey(), seed, &program_id).unwrap();
    println!("meta data account is {}", data_account_pubkey);
    let instruction = system_instruction::create_account_with_seed(
        &signer_pub, 
        &data_account_pubkey, 
        &signer_pub, 
        seed, 
        data_lamports, 
        data_num_bytes, 
        &program_id);

    let mut transaction = Transaction::new_with_payer(
        &[instruction],
        Some(&signer_pub),
    );
    println!("Constructed account transaction");

    let (recent_blockhash, _fee_calculator) = rpc_client
        .get_recent_blockhash()
        .map_err(|err| format!("error: unable to get recent blockhash: {}", err))?;
    println!("Got recent blockhash");

    transaction
        .try_sign(&vec![signer], recent_blockhash)
        .map_err(|err| format!("error: failed to sign transaction: {}", err))?;
    println!("Signed account transaction");

    let signature = rpc_client
        .send_and_confirm_transaction_with_spinner_and_commitment(&transaction, commitment_config)
        .map_err(|err| format!("error: send transaction: {}", err))?;
    println!("Processed account transaction");
    
    Ok(signature)
}

fn create_cluutch_data_account(
    rpc_client: &RpcClient,
    signer: &dyn Signer,
    commitment_config: CommitmentConfig,
) -> Result<Signature, Box<dyn std::error::Error>> {
    // let seed = "api.cluutch.io/v2/daily?date=2021-06-05";
    let seed = "api.cluutch.io";
    // let seed = "random-data-api.com";

    let program_id = Pubkey::from_str("HS4DFksToksNYF3fyU8Rxmg2B7sAq5JYhHKzitaVbDCp")?;
    let data_num_bytes = 500;
    let data_lamports = 11235813;

    let signer_pub = signer.pubkey();
    println!("Starting to create data account from {} with owner {}", signer_pub, program_id);

    let data_account_pubkey = Pubkey::create_with_seed(&signer.pubkey(), seed, &program_id).unwrap();
    println!("Data account is {}", data_account_pubkey);
    let instruction = system_instruction::create_account_with_seed(
        &signer_pub, 
        &data_account_pubkey, 
        &signer_pub, 
        seed, 
        data_lamports, 
        data_num_bytes, 
        &program_id);

    let mut transaction = Transaction::new_with_payer(
        &[instruction],
        Some(&signer_pub),
    );
    println!("Constructed account transaction");

    let (recent_blockhash, _fee_calculator) = rpc_client
        .get_recent_blockhash()
        .map_err(|err| format!("error: unable to get recent blockhash: {}", err))?;
    println!("Got recent blockhash");

    transaction
        .try_sign(&vec![signer], recent_blockhash)
        .map_err(|err| format!("error: failed to sign transaction: {}", err))?;
    println!("Signed account transaction");

    let signature = rpc_client
        .send_and_confirm_transaction_with_spinner_and_commitment(&transaction, commitment_config)
        .map_err(|err| format!("error: send transaction: {}", err))?;
    println!("Processed account transaction");
    
    Ok(signature)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app_matches = App::new(crate_name!())
        .about(crate_description!())
        .version(crate_version!())
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .arg({
            let arg = Arg::with_name("config_file")
                .short("C")
                .long("config")
                .value_name("PATH")
                .takes_value(true)
                .global(true)
                .help("Configuration file to use");
            if let Some(ref config_file) = *solana_cli_config::CONFIG_FILE {
                arg.default_value(&config_file)
            } else {
                arg
            }
        })
        .arg(
            Arg::with_name("keypair")
                .long("keypair")
                .value_name("KEYPAIR")
                .validator(is_valid_signer)
                .takes_value(true)
                .global(true)
                .help("Filepath or URL to a keypair [default: client keypair]"),
        )
        .arg(
            Arg::with_name("verbose")
                .long("verbose")
                .short("v")
                .takes_value(false)
                .global(true)
                .help("Show additional information"),
        )
        .arg(
            Arg::with_name("json_rpc_url")
                .long("url")
                .value_name("URL")
                .takes_value(true)
                .global(true)
                .validator(is_url)
                .help("JSON RPC URL for the cluster [default: value from configuration file]"),
        )
        .subcommand(SubCommand::with_name("register").about("Get balance").arg(
            Arg::with_name("endpoint")
            .long("endpoint")
            .short("e")
            .value_name("ENDPOINT")
            .takes_value(true)
            .global(true)
            .validator(is_url)
            .help("Full URL of the API's endpoint"),
        ))
        .subcommand(SubCommand::with_name("create-meta-data-account").about("Create meta data account"))
        .subcommand(SubCommand::with_name("create-cluutch-data-account").about("Create cluutch api data account"))
        .subcommand(SubCommand::with_name("process-api-payload").about("Seed oracle with data from cluutch.io"))
        .get_matches();

    let (sub_command, sub_matches) = app_matches.subcommand();
    let matches = sub_matches.unwrap();
    let mut wallet_manager: Option<Arc<RemoteWalletManager>> = None;

    let config = {
        let cli_config = if let Some(config_file) = matches.value_of("config_file") {
            solana_cli_config::Config::load(config_file).unwrap_or_default()
        } else {
            solana_cli_config::Config::default()
        };

        let default_signer = DefaultSigner {
            path: matches
                .value_of(&"keypair")
                .map(|s| s.to_string())
                .unwrap_or_else(|| cli_config.keypair_path.clone()),
            arg_name: "keypair".to_string(),
        };

        Config {
            json_rpc_url: matches
                .value_of("json_rpc_url")
                .unwrap_or(&cli_config.json_rpc_url)
                .to_string(),
            default_signer: default_signer
                .signer_from_path(&matches, &mut wallet_manager)
                .unwrap_or_else(|err| {
                    eprintln!("error: {}", err);
                    exit(1);
                }),
            verbose: matches.is_present("verbose"),
            commitment_config: CommitmentConfig::confirmed(),
        }
    };
    solana_logger::setup_with_default("solana=info");

    if config.verbose {
        println!("JSON RPC URL: {}", config.json_rpc_url);
    }
    let rpc_client = RpcClient::new(config.json_rpc_url.clone());

    match (sub_command, sub_matches) {
        ("process-api-payload", Some(_arg_matches)) => {
            let signature = process_api_payload(
                &rpc_client,
                config.default_signer.as_ref(),
                config.commitment_config,
            )
            .unwrap_or_else(|err| {
                eprintln!("error: send transaction: {}", err);
                exit(1);
            });
            println!("Signature: {}", signature);
        }
        ("create-meta-data-account", Some(_arg_matches)) => {
            let mut signature = create_metadata_account(
                &rpc_client,
                config.default_signer.as_ref(),
                config.commitment_config,
            )
            .unwrap_or_else(|err| {
                eprintln!("error: send transaction: {}", err);
                exit(1);
            });
            println!("Signature: {}", signature);
        }
        ("create-cluutch-data-account", Some(_arg_matches)) => {
            let mut signature = create_cluutch_data_account(
                &rpc_client,
                config.default_signer.as_ref(),
                config.commitment_config,
            )
            .unwrap_or_else(|err| {
                eprintln!("error: send transaction: {}", err);
                exit(1);
            });
            println!("Signature: {}", signature);
        }
        
        _ => unreachable!(),
    };

    Ok(())
}

#[cfg(test)]
mod test {
    use {super::*, solana_validator::test_validator::*};

    // #[test]
    // fn test_ping() {
    //     let (test_validator, payer) = TestValidatorGenesis::default().start();
    //     let (rpc_client, _recent_blockhash, _fee_calculator) = test_validator.rpc_client();

    //     assert!(matches!(
    //         process_ping(
    //             &rpc_client,
    //             &payer,
    //             CommitmentConfig::single_gossip()
    //         ),
    //         Ok(_)
    //     ));
    // }
}

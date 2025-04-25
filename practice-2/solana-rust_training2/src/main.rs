use std::str::FromStr;
use solana_sdk::signer;
use solana_sdk::signer::Signer;
use solana_sdk::signer::keypair::Keypair;
use solana_sdk::system_transaction;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_client::rpc_client::RpcClient;

use  spl_token_2022::{
      id as token_2022_program_id,
    //  instruction::initialize_mint,
    //  state::Mint
    };
use spl_token_client::{
    client::{ProgramRpcClient, ProgramRpcClientSendTransaction},
    token::{ExtensionInitializationParams, Token},
  }; 

use std::sync::Arc;
//use solana_program::instruction::Instruction;
use mpl_token_metadata::instructions::CreateV1InstructionArgs;
use mpl_token_metadata::instructions::CreateV1;
use mpl_token_metadata::accounts::Metadata;
use mpl_token_metadata::types::PrintSupply;
use mpl_token_metadata::types::TokenStandard;
//use mpl_token_metadata



//use futures::executor::block_on;
//use std::thread;
//use std::sync::mpsc;
//use std::time::Duration;


 const URL_DEVNET: &str = "https://api.devnet.solana.com";
  
 const TOKEN_METADATA_PROGRAM_ID: &str = "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s";
  
  #[tokio::main]
 async fn main() -> std::io::Result<()>{    
    println! ("\n Тестове завдання 2.8 для solana bootcamp 3 ");
   //читаю свій ключ
    let read_sec_key = signer::keypair::read_keypair_file(".env_s").expect("Помилка читання даниху");
    let read_pub_key = read_sec_key.pubkey();
    println!("\n read_my_pub_key: {read_pub_key}");

    let connect = RpcClient::new(URL_DEVNET);
   // let connect = RpcClient::new("https://api.devnet.solana.com");
    let balance = connect.get_balance(&read_pub_key).expect("Помилка читання даниху");
    println!("My balance before transaction: {:?}", balance);
    let rec1: &str = "AnZAwGgvAgW5ABqbWfCAi4b4UPKWbJ3MEv3ut3kXEofA";
    let recipient1 = FromStr::from_str(&rec1).unwrap();
    println!("Hello to students1 bootcamp! {:?}",recipient1);

    let latest_blockhash = connect.get_latest_blockhash().expect("Помилка отримання останнього блокчейну");
    let tx = system_transaction::transfer(&read_sec_key, &recipient1, 5000, latest_blockhash);
    
    let signature: solana_sdk::signature::Signature = connect.send_transaction(&tx).expect("Помилка виконання транзакції");
    loop {
        let confirmed = connect.confirm_transaction(&signature).expect("Очікуєм виконання транзакції");
        if confirmed {
            break;
        }
    }
    println!(" send 5000 lampotrs to : {:?}", rec1);
    println!("signature= {:?}",signature);
    let balance = connect.get_balance(&read_pub_key).expect("Помилка читання даних");
    println!("My balance after transaction: {:?}", balance);

    println!("\n Створюю для себе tokenMint як поєднання свого і нового ключа ");
    //Generate keypair to use as address of mint

    let mint_account_sec = Keypair::new();
    //let mint_account_pub = &mint_account_sec.pubkey().to_string()); приватний ключ
    let mint_account_pub = &mint_account_sec.pubkey();
    //let mint_account_private = &mint_account_sec.to_base58_string());
    //let mint_account_JSON = &mint_account_sec.to_bytes());

    // Set up program client for Token client
    let rpc_client = solana_client::nonblocking::rpc_client::RpcClient::new_with_commitment(
        String::from("https://api.devnet.solana.com"),
        CommitmentConfig::confirmed(),
    );
   
     let program_client =
     ProgramRpcClient::new(Arc::new(rpc_client), ProgramRpcClientSendTransaction);
     
     let decimals = 6;
     
     let token = Token::new(
        Arc::new(program_client),
        &token_2022_program_id(),
        &mint_account_pub,
        Some(decimals),
        Arc::new(read_sec_key.insecure_clone()),
    );

    // Create and initialize the mint
    let extension_initialization_params: Vec<ExtensionInitializationParams> = Vec::new();

    let transaction_signature =
         token
                         .create_mint(
                             &read_sec_key.pubkey(),                 // mint authority
                             Some(&read_sec_key.pubkey()),           // freeze authority
                             extension_initialization_params, // no extensions
                             &[&mint_account_sec],                        // mint keypair needed as signer
                         ).await.expect("Помилка виконання транзакції");
           
 
    println!("Mint Address: {:?}", mint_account_sec.pubkey());
    println!("Transaction Signature: {:?}",transaction_signature);

    // який буде повязаний адрес Token Account
    let associated_token_address = token.get_associated_token_address(&read_sec_key.pubkey());
    println!("повязаний адрес Token Account буде: {}",  associated_token_address );

    // створимо повязаний адрес Token Account
     let create_ata_result = token
        .create_associated_token_account(
            &read_sec_key.pubkey(), // owner
        )
        .await.expect("Помилка створення повязаний адрес Token Account");

    println!("повязаний адрес Token Account: {}", create_ata_result);

    // Mint tokens to the associated token account
    let amount = 10; 
    let mint_to_result = token
        .mint_to(
            &associated_token_address, // destination
            &read_sec_key.pubkey(),           // authority (mint authority)
            amount,                    // amount
            &[&read_sec_key],                 // additional signers (providing payer as a signer)
        )
        .await.expect("Помилка транзакції Token mint");

    println!("Minted {} tokens to associated token account", amount);
    println!("Mint Transaction: {}", mint_to_result);

    // Get token account balance to verify the minting
    let balance = token.get_account_info(&associated_token_address)
    .await.expect("Помилка отримання балансу Token Account");
    println!("Token Account Balance: {} tokens", balance.base.amount);     

// instruction args
let args = CreateV1InstructionArgs {
    name: String::from("YarosWit on Bootcamp"),
    symbol: String::from("TOKN"),
    uri: String::from("http://my.pnft"),
    seller_fee_basis_points: 500,
    primary_sale_happened: false,
    is_mutable: true,
    token_standard: TokenStandard::ProgrammableNonFungible,
    collection: None,
    uses: None,
    collection_details: None,
    creators: None,
    rule_set: None,
    decimals: Some(0),
    print_supply: Some(PrintSupply::Zero),
};

// instruction accounts
let create_ix = CreateV1 {
    //TOKEN_METADATA_PROGRAM_ID,
    metadata,
    master_edition: Some(master_edition),
    mint: (&mint_account_pub, true),
    authority: read_sec_key.pubkey(),
    payer: read_sec_key.pubkey(),
    update_authority: (read_sec_key.pubkey(), true),
    system_program: system_program::ID,
    sysvar_instructions: solana_program::sysvar::instructions::ID,
    spl_token_program: TOKEN_METADATA_PROGRAM_ID, //spl_token::ID,
};

// creates the instruction
let create_ix = create_ix.instruction(args);
    
    

    Ok(())
}
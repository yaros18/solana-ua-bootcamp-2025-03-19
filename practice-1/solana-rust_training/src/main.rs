use std::io;
use solana_sdk::*;
use signer::Signer;
use solana_client::rpc_client::RpcClient;

const LAMPORTS_PER_SOL: u64 = 1000000000;
const URL_DEVNET: &str = "https://api.devnet.solana.com";

fn main() {

    println!("\n Тестові завдання 1.6 для solana bootcamp 3 ");
    loop {
        println!("\n Виберіть варіант:");
        println!("1. Генеруєм і записуєм ключ");
        println!("2. Читаєм ключ і перевіряєм");
        println!("3. Аірдроп і перевірка балансу");
        println!("4. Вихід");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => {
                let sec_key = signature::Keypair::new();
                let pub_key = sec_key.pubkey();
                println!("pub_key: {pub_key}");
               
                signer::keypair::write_keypair_file(&sec_key,".env_s").expect("Помилка додавання даниху");
               
            }
            "2" => {
              let read_sec_key = signer::keypair::read_keypair_file(".env_s").expect("Помилка читання даниху");
              let read_pub_key = read_sec_key.pubkey();
              println!("read_pub_key: {read_pub_key}");
         
            }
            "3" => {
               let read_sec_key = signer::keypair::read_keypair_file(".env_s").expect("Помилка читання даниху");
               let read_pub_key = read_sec_key.pubkey();
  
               let connect = RpcClient::new(URL_DEVNET);
               let airdrop = connect.request_airdrop(&read_pub_key, LAMPORTS_PER_SOL).unwrap();
               println!("Airdrop : {:?}",  airdrop);

               let balance = connect.get_balance(&read_pub_key).expect("Помилка читання даниху");
                println!("balance: {:?}", balance);
            }

            "4" => {
                break;
            }
            _ => println!("Виберіть цифри від 1 до 4"),
        }
    }
}

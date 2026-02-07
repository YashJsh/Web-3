mod transfer_token;
mod get_balance;
use std::io::{self, Write};

use solana_sdk::native_token::LAMPORTS_PER_SOL;

#[tokio::main]
async fn main() {
    println!("======================================");
    println!("            Solana CLI Tool           ");
    println!("======================================");
    println!("Available commands:");
    println!("  send-sol <AMOUNT> <PRIVATE_KEY> <RECEIVER_ADDRESS>");
    println!("  balance <ADDRESS>");
    println!();
    println!("Example: send-sol 0.98 2812x... 110181a..");
    println!("======================================");
    println!();
    let mut command = String::new();
    print!(">");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut command).unwrap();

    let command = command.trim();

    if command.is_empty() {
        println!("No command entered.");
        return;
    }
    let command_vec: Vec<&str> = command.split(" ").collect();
    match command_vec[0] {
        "send-sol" => {
            let amount: u32 = command_vec[1].parse().unwrap();
            let private_key = command_vec[2].to_string();
            let receiver = command_vec[3].to_string();

            if amount == 0 || private_key.is_empty() || receiver.is_empty() {
                println!("Usage: send-sol <AMOUNT> <PRIVATE_KEY> <RECEIVER_ADDRESS>");
                return;
            }

            let transferring = transfer_token::send_sol(private_key, amount, receiver).await;
            match transferring {
                Ok(signature) => println!("Tx Signature: {}", signature),
                Err(error) => println!("Error sending SOL: {}", error),
            }
        },
        "balance" => {
            let address = command_vec[1].to_string();
            if address.is_empty(){
                println!("Usage : balance <ADDRESS>");
                return;
            }
            let balance = get_balance::get_balance(address).await;
            match balance{
                Ok(ans)=> println!("Your Balance is : {} lamports", ans/LAMPORTS_PER_SOL),
                Err(_)=>println!("Error fetching the balance")
            }
        },
        _ => println!("Command doesn't exists"),
    }
}

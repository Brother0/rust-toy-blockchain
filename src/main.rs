use std::io;
use std::io::Write;
use std::process;

mod blockchain;

fn main() {
    let mut miner_addr = String::new();
    let mut difficulty = String::new();
    let mut choice = String::new();

    print!("Input a miner address: ");
    let mut read_addr = || -> Result<(), std::io::Error> {
        io::stdout().flush()?;
        io::stdin().read_line(&mut miner_addr)?;
        Ok(())
    };

    if let Err(_err) = read_addr() {
        println!("Reading data failed with error: {}", _err);
    }
    print!("difficulty: ");

    let mut read_difficulty = || -> Result<(), std::io::Error> {
        io::stdout().flush()?;
        io::stdin().read_line(&mut difficulty)?;
        Ok(())
    };

    if let Err(_err) = read_difficulty() {
        println!("Reading data failed with error: {}", _err);
    }

    let diff = difficulty
        .trim()
        .parse::<u32>()
        .expect("we need an integer");
    println!("generating genesis block! ");
    let mut chain = blockchain::Chain::new(miner_addr.trim().to_string(), diff);

    loop {
        println!("Menu");
        println!("1) New Transaction");
        println!("2) Mine Block");
        println!("3) Change Difficulty");
        println!("4) Change Reward");
        println!("0) Exit");
        println!("Enter your choice: ");

        let mut read_choice = || -> Result<(), std::io::Error> {
            io::stdout().flush()?;
            choice.clear();
            io::stdin().read_line(&mut choice)?;
            Ok(())
        };
    
        if let Err(_err) = read_choice() {
            println!("Reading data failed with error: {}", _err);
        }
        print!("");

        match choice.trim().parse().unwrap() {
            0 => {
                println!("exiting!");
                process::exit(0);
            }
            1 => {
                let mut sender = String::new();
                let mut receiver = String::new();
                let mut amount = String::new();

                print!("enter sender address::");
                let mut read_sender = || -> Result<(), std::io::Error> {
                    io::stdout().flush()?;
                    io::stdin().read_line(&mut sender)?;
                    Ok(())
                };

                if let Err(_err) = read_sender() {
                    println!("Reading data failed with error: {}", _err);
                }



                print!("enter receiver address::");
                let mut read_receiver = || -> Result<(), std::io::Error> {
                    io::stdout().flush()?;
                    io::stdin().read_line(&mut receiver)?;
                    Ok(())
                };

                if let Err(_err) = read_receiver() {
                    println!("Reading data failed with error: {}", _err);
                }

                print!("enter amount address::");
                let mut read_amount = || -> Result<(), std::io::Error> {
                    io::stdout().flush()?;
                    io::stdin().read_line(&mut amount)?;
                    Ok(())
                };

                if let Err(_err) = read_amount() {
                    println!("Reading data failed with error: {}", _err);
                }

                let res = chain.new_transaction(
                    sender.trim().to_string(),
                    receiver.trim().to_string(),
                    amount.trim().parse().unwrap(),
                );

                match res {
                    true => println!("Transaction added"),
                    false => println!("Transaction failed"),
                }
            }
            2 => {
                println!("Generating block");
                let res = chain.generate_new_block();
                match res {
                    true => println!("Block generated successfuly"),
                    false => println!("Block generation failed"),
                }
            }
            3 => {
                let mut new_diff = String::new();
                print!("enter new difficulty: ");
                let mut read_difficulty = || -> Result<(), std::io::Error> {
                    io::stdout().flush()?;
                    io::stdin().read_line(&mut new_diff)?;
                    Ok(())
                };

                if let Err(_err) = read_difficulty() {
                    println!("Reading data failed with error: {}", _err);
                }

                let res = chain.update_dificulty(new_diff.trim().parse().unwrap());
                match res {
                    true => println!("Dificulty updated successfuly"),
                    false => println!("Dificulty update failed"),
                }
            }
            4 => {
                let mut new_reward = String::new();
                print!("enter new reward: ");
                let mut read_reward = || -> Result<(), std::io::Error> {
                    io::stdout().flush()?;
                    io::stdin().read_line(&mut new_reward)?;
                    Ok(())
                };

                if let Err(_err) = read_reward() {
                    println!("Reading data failed with error: {}", _err);
                }

                let res = chain.update_reward(new_reward.trim().parse().unwrap());
                match res {
                    true => println!("Reward updated successfuly"),
                    false => println!("Reward update failed"),
                }
            }
            _ => println!("Invalid option please retry"),
        }
    }
}

use colored::*;
use async_recursion::async_recursion;
use std::{
    io::stdin,
    clone::Clone,
    thread::sleep,
    time::Duration,
    error::Error
};

mod files_utils;
mod menus;
mod reqs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    print!("\x1B[2J\x1B[1;1H");
    call_functions().await?;

    Ok(())
}

async fn execute(telephone: String, token: String , repeat: u64) -> Result<(), Box< dyn Error>> {

    let mut count_false: u64 = 0;
    let mut count_true: u64 = 0;

    for _i in 0..repeat {
        println!("{} {}", "Testando com o token:".bright_yellow(), token.clone());

        let _req = reqs::request_to_api(
            telephone.clone(),
            token.clone()
        ).await?;

        if _req {
            count_true += 1;
        } else{
            count_false += 1;
        }
        print!("\x1B[2J\x1B[1;1H");
        println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".blue());
        println!(
            "{}: {}\n{}: {}",
            "True".bright_green(), count_true,
            "False".bright_red(), count_false
        );
        println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".blue());

    }
    Ok(())
}


#[async_recursion]
async fn opt_insert() -> Result<(), Box<dyn std::error::Error>> {
    let telephone = menus::get_telephone();
    if telephone.status {
        let checker_file = files_utils::verify_file(".data.json");

        if checker_file {
            let get_all_tokens = files_utils::read_file(".data.json");

            println!("{}", "Quantas vezes deseja repetir?".bright_yellow());
            let mut repeat_string = String::new();

            stdin().read_line(&mut repeat_string).unwrap();
            let repeat = repeat_string.replace("\n","").parse::<u64>().unwrap();

            for token in get_all_tokens.tokens.into_iter() {
               execute(telephone.number.to_string(), token, repeat).await?;
            }
            println!("{}", "Cabô".bright_yellow());

        } else {
            reqs::register_tokens().await.unwrap();
            println!("{}", "Os tokens foram atualizados".bright_red());
            sleep(Duration::from_millis(1000));
            opt_insert().await?;
        }
    } else {
        println!("{}: {}", "Número inválido".bright_red(), telephone.number.bright_red());
        sleep(Duration::from_millis(1000));
        opt_insert().await?;
    }

    Ok(())
}

#[async_recursion]
async fn personal_token() -> Result<(), Box<dyn std::error::Error>> {
    let telephone = menus::get_telephone();
    if telephone.status {
        println!("{}", "Quantas vezes deseja repetir?".bright_yellow());
        let mut repeat_string = String::new();

        stdin().read_line(&mut repeat_string).unwrap();
        let repeat = repeat_string.replace("\n","").parse::<u64>().unwrap();

        println!("{}", "Digite o seu token: ".bright_yellow());
        let mut token = String::new();

        stdin().read_line(&mut token).unwrap();

        execute(telephone.number.to_string(), token.replace("\n",""), repeat).await?;
        println!("{}", "Cabô".bright_yellow());

    } else {
        println!("{}: {}", "Número inválido".bright_red(), telephone.number.bright_red());
        sleep(Duration::from_millis(1000));
        personal_token().await?;
    }

    Ok(())
}


#[async_recursion]
async fn call_functions() -> Result<(), Box<dyn std::error::Error>> {
    print!("\x1B[2J\x1B[1;1H");
    let options = menus::show_menu();

    match options {
        1 => {
            print!("\x1B[2J\x1B[1;1H");
            opt_insert().await?;
        }
        2 => {
            println!("{}","Atualizando os tokens..".bright_yellow());
            reqs::register_tokens().await?;
            println!("{}", "Tokens atualizados".bright_yellow());
            sleep(Duration::from_millis(1000));
            call_functions().await?;
        },
        3 => {
            print!("\x1B[2J\x1B[1;1H");
            println!("{}", "Token \"personalizado\"".bright_yellow());
            personal_token().await?;
        }
        _ => {
            println!("{}", "Opção inválida".bright_red());
            sleep(Duration::from_millis(1000));
            call_functions().await?;
        }
    }

    Ok(())
}


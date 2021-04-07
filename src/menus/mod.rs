use regex::Regex;
use colored::*;
use std::io::stdin;

pub struct NumberStatus {
    pub msg: &'static str,
    pub number: String,
    pub status: bool
}


pub fn show_menu() -> u32 {
    let options = vec![
        format!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".blue()),
        format!("{}", "By: @ShuseiKagari".bright_red()),
        format!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".blue()),
        format!("{}", "Selecione uma das opções abaixo".green()),
        format!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".blue()),
        format!("{}) -> {}", "1".bright_blue(), "Iniciar".bright_green()),
        format!("{}) -> {}", "2".bright_blue(), "Atualizar tokens".bright_green()),
        format!("{}) -> {}", "3".bright_blue(), "Inserir \"token personalizado\"".bright_green()),
        format!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".blue()),
        format!("{}", "Insira a opção: ".green())
    ];

    print!("\x1B[2J\x1B[1;1H");

    for option in options.into_iter() {
        println!("{}", option);
    }

    let mut choice = String::new();
    stdin().read_line(&mut choice)
        .expect("Ocorreu um erro ao ler o número");
    let choiced: u32 = choice.replace("\n", "").parse::<u32>().expect("Número inválido!");

    choiced
}


pub fn get_telephone() -> NumberStatus {
    let re = Regex::new(r"(\d{2})(\d{8,9})").unwrap();
    println!("{}", "Insira o seu número".bright_green());
    let mut number = String::new();

    stdin().read_line(&mut number)
        .expect(&"Ocorreu um erro ao ler o número".red());

    if re.is_match(&number) {
        NumberStatus{
            msg: "Número válido!",
            number: number.replace("\n",""),
            status: true
        }
    } else {
        NumberStatus{
            msg: "Número inválido!",
            number: "40028922".to_owned(),
            status: false
        }
    }

}


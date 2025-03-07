mod clients;
mod input;

use std::process::exit;
use clients::{initialize_db, validate_client, make_withdrawal, make_deposit, view_balance};
use input::{get_user_input};

fn main() {

    initialize_db();

    println!("Seja bem vindo ao rust-bank!");

    let mut client_name: String = String::new();
    let mut client_id: String = String::new();

    loop {
        let client_code = get_user_input("Insira seu código de cliente: ");

        if client_code.is_err() {
            println!("Erro lendo os dados do cliente, tente novamente!");
        }

        let client_found = validate_client(client_code.unwrap().as_str());

        if client_found.is_err() {
            println!("Cliente não encontrado, tente novamente!");
            continue;
        }

        let client = client_found.unwrap();
        client_name = client.name.clone();
        client_id = client.id.clone();

        println!("Seja bem-vindo, {}", client.name);

        while true {
            println!("Insira a opção para a operação desejada:");
            println!("1 - Verificar saldo");
            println!("2 - Fazer aplicação");
            println!("3 - Fazer saque");
            println!("4 - Deslogar");
            println!("5 - Sair");

            let option = get_user_input("Opção: ");

            if option.is_err() {
                println!("Erro ao obter dados do teclado!");
                continue;
            }

            match option.unwrap().as_str() {
                "1" => app_view_balance(&client_id),
                "2" => app_make_deposit(&client_id),
                "3" => app_make_withdrawal(&client_id),
                "4" => break,
                "5" => end(&client_name),
                _ => { println!("Opção inválida, tente novamente!"); }
            }
        }
    }

    fn end(client_name: &String) {
        println!("Até a próxima, {}", &client_name);
        exit(0);
    }

    fn app_view_balance(client_id: &String) {

        let account_balance = view_balance(&client_id);

        if account_balance.is_err() {
            println!("Erro: {}", &account_balance.clone().unwrap_err())
        }

        println!("Seu saldo é de: R$ {}", account_balance.unwrap());
    }

    fn app_make_deposit(client_id: &String) {

        let value = get_user_input("Insira o valor que deseja depositar (R$): ");

        if value.is_err() {
            println!("Erro ao obter dados do teclado!");
            return;
        }

        let parsed_value: Result<f64, std::num::ParseFloatError> = value.unwrap().parse();

        if parsed_value.is_err() {
            println!("Número inválido. Tente novamente mais tarde.");
        }

        let operation_return = make_deposit(&client_id, &parsed_value.as_ref().unwrap());

        if operation_return.is_err() {
            println!("Erro: {}", operation_return.unwrap_err());
            return;
        }

        println!("Sucesso ao realizar o depósito de RS {}!", &parsed_value.unwrap().to_string());
    }

    fn app_make_withdrawal(client_id: &String) {

        let value = get_user_input("Insira o valor que deseja sacar (R$): ");

        if value.is_err() {
            println!("Erro ao obter dados do teclado!");
            return;
        }

        let parsed_value: Result<f64, std::num::ParseFloatError> = value.unwrap().parse();

        if parsed_value.is_err() {
            println!("Número inválido. Tente novamente mais tarde.");
        }

        let operation_return = make_withdrawal(&client_id, &parsed_value.as_ref().unwrap());

        if operation_return.is_err() {
            println!("Erro: {}", operation_return.unwrap_err());
            return;
        }

        println!("Sucesso ao realizar o saque de RS {}!", &parsed_value.unwrap().to_string());
    }
}

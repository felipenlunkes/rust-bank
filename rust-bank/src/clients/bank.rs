pub mod bank {
    use std::sync::Mutex;

    #[derive(Clone)]
    pub struct BankClient {
        pub(crate) name: String,
        pub (crate) id: String,
        account_value: f64
    }

    struct Bank {
        clients: Vec<BankClient>
    }

    impl Bank {

        fn new() -> Self {
            Bank { clients: Vec::new() }
        }

        fn add_client(&mut self, name: String, id: String, account_value: f64) {
            self.clients.push(BankClient { name, id, account_value });
        }

        fn client_exists(&self, search_id: String) -> bool {
            self.clients.iter().any(|client| client.id == search_id)
        }

        fn get_client(&self, search_id: String) -> Option<&BankClient> {
            self.clients.iter().find(|client| client.id == search_id)
        }

        pub fn get_client_for_update(&mut self, search_id: String) -> Option<&mut BankClient> {
            self.clients.iter_mut().find(|client| client.id == search_id)
        }
    }

    lazy_static::lazy_static! {
        static ref BANK: Mutex<Bank> = Mutex::new(Bank::new());
    }

    pub fn validate_client(client_id: &str) -> Result<(BankClient), String> {

        let bank = BANK.lock().unwrap();

        let client_found = Bank::client_exists(&bank, client_id.to_string());

        if client_found {
            let bank_client = Bank::get_client(&bank, client_id.to_string());
            Ok(bank_client.unwrap().clone())
        } else {
            Err(format!("Cliente {} não encontrado!", client_id)) }
    }

    pub fn view_balance(client_id: &str) -> Result<(f64), String> {

        let bank = BANK.lock().unwrap();

        let client_found = Bank::client_exists(&bank, client_id.to_string());

        if !client_found {
            return Err(format!("Cliente {} não encontrado!", client_id));
        }

        let client = Bank::get_client(&bank, client_id.to_string()).unwrap();

        Ok(client.account_value)
    }

    pub fn make_deposit(client_id: &str, value: &f64) -> Result<(), String> {

        let mut bank = BANK.lock().unwrap();

        let client_found = Bank::client_exists(&bank, client_id.to_string());

        if !client_found {
            return Err(format!("Cliente {} não encontrado!", client_id));
        }

        let client = Bank::get_client_for_update(&mut bank, client_id.to_string()).unwrap();

        client.account_value += value;

        Ok(())
    }

    pub fn make_withdrawal(client_id: &str, value: &f64) -> Result<(), String> {

        let mut bank = BANK.lock().unwrap();

        let client_found = Bank::client_exists(&bank, client_id.to_string());

        if !client_found {
           return Err(format!("Cliente {} não encontrado!", client_id));
        }

        let client = Bank::get_client_for_update(&mut bank, client_id.to_string()).unwrap();

        if value > &client.account_value {
            return Err(format!("O saldo do cliente (R$ {}) é menor que o valor pedido para saque (R$ {})", client.account_value, &value));
        }

        client.account_value -= value;

        Ok(())
    }

    pub fn initialize_db() {

        let mut bank = BANK.lock().unwrap();

        bank.add_client(String::from("Felipe Lunkes"), String::from("001"), 10000.0);
        bank.add_client(String::from("Gabriel Corrêa"), String::from("002"), 20000.0) ;
        bank.add_client(String::from("Miguel Lunkes"), String::from("003"), 30000.0);

    }

}



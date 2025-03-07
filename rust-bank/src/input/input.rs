pub mod input {

    use std::io;

    pub fn get_user_input(prompt: &str) -> Result<String, ()> {
        println!("{}", prompt);
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_n) => { Ok(input.trim().to_string()) }
            Err(_error) => { Err(()) }
        }
    }
}

use p32::bank::{Bank, User};

fn main() {
    let mut my_bank = Bank::new(String::from("javi_bank"));
    my_bank.create_user("Floyd".to_string());
    my_bank.create_user("Issac".to_string());
    println!("{:?}", my_bank)
}

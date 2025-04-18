use p32::bank::Bank;

fn main() {
    let mut my_bank = Bank::new(String::from("javi_bank"));
    my_bank.create_user("Floyd".to_string(), 10);
    my_bank.create_user("Issac".to_string(), 10);
    println!("{:?}", my_bank)
}

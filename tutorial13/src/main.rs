use std::thread;

fn main() {
    pub struct  Bank{
        balance: f32,
    }
    fn withdraw(the_bank: &mut Bank, amt: f32){
        the_bank.balance-= amt;
        println!("{}",the_bank.balance);
    }
    let mut bank = Bank{balance: 1000.0};
    withdraw(&mut bank, 400.0);
    fn customer(the_bank: &mut Bank){
        withdraw(the_bank, 5.0);
    }
    thread::spawn(move||{
        customer(&mut bank);
    }).join().unwrap();

}

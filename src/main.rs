mod login {
    pub mod administrator {
        pub fn account () {
            println!("This is an administrator account.");
        }
    }
}

mod lib;
use::guest;

fn main() {
    
    crate::login::administrator::account();
    lib::login2::standard_user::account2();
    guest::login3::guest_acc::account3();

}

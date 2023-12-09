use std::{marker::PhantomData, fmt::Error};

use electron_rs::verifier::near::fr_from_str;

use stylus_sdk::{evm, msg, stylus_proc::sol_storage, storage::StorageString};

pub trait IDK {
    const NAME: &'static str;
}

sol_storage! {
    pub struct IDKContract<T> {
        string fr_string;
        PhantomData<T> phantom
    }
}

impl<T: IDK> IDKContract<T> {
    pub fn do_something(&mut self) -> Result<(),Error> {
        let fr = fr_from_str(String::from("139034790179591340742761703217010858871"));
        self.fr_string.set_str("text");
        Ok(())
    }
}

fn main() {
    println!("Hello, world!");
}

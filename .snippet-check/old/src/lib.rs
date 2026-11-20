use near_sdk::{near, env, log, Promise, Gas, PromiseError};
use serde_json::json;

#[near(contract_state)]
#[derive(Default)]
pub struct Contract { }

const HELLO_NEAR: &str = "hello-nearverse.testnet";
const NO_DEPOSIT: u128 = 0;
const CALL_GAS: Gas = Gas::from_tgas(5);

#[near]
impl Contract {
  pub fn call_method(&self){
    let args = json!({ "message": "howdy".to_string() })
              .to_string().into_bytes().to_vec();

    Promise::new(HELLO_NEAR.parse().unwrap())
    .function_call("set_greeting".to_string(), args, NO_DEPOSIT, CALL_GAS)
    .then(
      Promise::new(env::current_account_id())
      .function_call("callback".to_string(), Vec::new(), NO_DEPOSIT, CALL_GAS)
    );
  }

  pub fn callback(&self, #[callback_result] result: Result<(), PromiseError>){
    if result.is_err(){
        log!("Something went wrong")
    }else{
        log!("Message changed")
    }
  }
}

// nft.mdx / ft.mdx / dex.mdx: the YOCTO_NEAR constant and the 1-yocto assert
pub mod one_yocto {
  use near_sdk::{env, ext_contract, json_types::U128, AccountId, NearToken, Promise};
  const YOCTO_NEAR: u128 = 1;

  #[ext_contract(ext_ft)]
  pub trait Ft { fn ft_transfer(&mut self, receiver_id: AccountId, amount: U128, memo: Option<String>); }

  pub fn send_tokens(ft_contract: AccountId, receiver_id: AccountId, amount: U128) -> Promise {
    assert_eq!(env::attached_deposit(), 1, "Requires attached deposit of exactly 1 yoctoNEAR");
    let _unused: Option<NearToken> = None;
    ext_ft::ext(ft_contract)
      .with_attached_deposit(YOCTO_NEAR)
      .ft_transfer(receiver_id, amount, None)
  }
}

mod utils;
use std::str;

use cosmwasm_std::{MessageInfo, Addr};
use msg::{ExecuteMsg, QueryMsg};
use serde_json::to_string;
// use utils::MessageInfo;
use wasm_bindgen::prelude::*;

use utils::{create_lto_env, create_lto_deps};
// use wasm_bindgen_test::*;

pub mod msg;
pub mod state;
pub mod store;
pub mod error;
pub mod contract;


// use crate::msg::{CountResponse, ExecuteMsg, QueryMsg, InstantiateMsg};

#[wasm_bindgen]
pub fn initialize() {
    utils::set_panic_hook()
}

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

#[wasm_bindgen]
pub fn square(number: i32) -> i32 {
    alert("computing square...");
    number * number
}

#[wasm_bindgen]
pub async fn execute(msg: JsValue) {
    // load from indexed db 

    let mut deps = create_lto_deps().await;
    // add the storage to the deps
    
    let message: ExecuteMsg = msg.into_serde().unwrap();
    let result = contract::execute(deps.as_mut(), create_lto_env(), MessageInfo {sender: Addr::unchecked(""),funds: Vec::new()} , message);
    let _res = match result {
        Ok(response) => {
            let resp_json = to_string(&response);
            alert(&resp_json.unwrap());
            deps.storage.sync_to_db().await;
        },
        Err(error) => panic!("contract resulted in error :{:?}", error)
    
    };
}

#[wasm_bindgen]
pub async fn query_state() -> i32 {
    let deps = create_lto_deps().await;

    let msg = QueryMsg::GetCount();
    let query_result = contract::query(deps.as_ref(), msg);
    match query_result {
        Ok(count_response) => return count_response.count,
        Err(error) => panic!("contract query failed. {:?}", error)
    }
}

#[wasm_bindgen]
pub async fn query(msg: JsValue) -> i32 {
    let deps = create_lto_deps().await;
    // let msg_string: String = msg.as_string().unwrap();
    // log(&msg_string);
    let message: QueryMsg = msg.into_serde().unwrap();
    let query_result = contract::query(deps.as_ref(),message);
    match query_result {
        Ok(count_response) => return count_response.count,
        Err(error) => panic!("contract query failed. errpr {:?}", error)
    }
}

// 

// #[wasm_bindgen]
// pub fn instantiate(count: JsValue) {} 



// pub fn instantiate(info: MessageInfo ,msg: InstantiateMsg) -> State {
//     // create an initial event for the eventchain
//     let state = State {
//         count: msg.count,
//     };

//     return state;
// }

#[cfg(test)]
mod tests {

    use crate::state::State;

    extern crate serde_json;
    #[test]
    fn create_state_from_json() {
        let state = State {count: 0};
        let json_string = serde_json::to_string(&state);
        println!("{:?}",json_string)
    }


}
// #[wasm_bindgen_test]
// fn test_query_jsvalue() {
//     let value = JsValue::from_serde(&QueryMsg::GetCount()).unwrap();
//     query(&value);
// }
//
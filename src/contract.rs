use std::ops::Add;

use crate::error::ContractError;

use crate::msg::{ExecuteMsg, InstatiateMsg, QueryMsg};
use crate::state::{Role, UserDetails, USERS, ENTRY_SEQ, HR};
use cosmwasm_std::{
    entry_point, to_binary, Binary, DepsMut, Empty, Env, MessageInfo, Order, Response, StdError,
};
use cosmwasm_std::{Deps, StdResult};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstatiateMsg,
) -> Result<Response, StdError> {
    //here we are adding the HR/or Super user
    //validate should be done here
    HR.save(deps.storage, &msg.hr.username)
        .expect("Error in instantiating the admin");
    ENTRY_SEQ.save(deps.storage, &0u64)?;

    Ok(Response::new().add_attribute("action", "HR instantiated"))
}

#[cfg_attr(not(feature = "library"), entry_point)]

pub fn execute(
    dep: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<Empty>, ContractError> {
    match msg {
        ExecuteMsg::AddEmployee {
            name,
            age,
            address,
            role,
        } => addemploye(dep, env, info, name, age, address, role),
    }
}

pub fn addemploye(
    dep: DepsMut,
    _env: Env,
    info: MessageInfo,
    name: String,
    age: u64,
    address: String,
    role: Role,
) -> Result<Response, ContractError> {
    let id: u64;
    //check if the sender is HR
    let a = HR.load(dep.storage).expect("Super admin is not present");
    println!("the info sender is {}",info.sender);
    //checking if the sender matches the instatiated super admin
    if a == info.sender {
        println!("the success case ");
        let uid = ENTRY_SEQ
            .update::<_, cosmwasm_std::StdError>(dep.storage, |uid| Ok(uid.add(1)))?;
        id = uid.clone();
        let emp = UserDetails {
            uid,
            username: name,
            age,
            address,
            role,
        };

        USERS
            .save(dep.storage, 1, &emp)
            .expect("Error while adding the employee");
    } else {
        return Err(ContractError::InstateError {});
    }
    Ok(Response::new()
        .add_attribute("Action", "employee added")
        .add_attribute("userid ",id.to_string())
       )
}
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(dep: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    use QueryMsg::*;
    match msg {
        GetEmployess {} => to_binary(&getemployees(dep)),
        GetEmployee { uid } => to_binary(&getemployee(dep, uid)?),
        GetSuperAdmin {} => to_binary(&admin(dep)?),
    }
}
pub fn admin(dep: Deps) -> StdResult<String> {
    let ad = HR.load(dep.storage)?;
    Ok(ad)
}
pub fn getemployees(dep: Deps) -> Vec<UserDetails> {
    let mut re: Vec<UserDetails> = Vec::new();
    for result in USERS.range(dep.storage, None, None, Order::Ascending) {
        match result {
            Ok(res) => {
                re.push(res.1);
            }
            Err(_res) => {}
        }
    }
    re
}
pub fn getemployee(dep: Deps, userid: u64) -> StdResult<UserDetails> {
    let user = USERS.load(dep.storage, userid)?;
    Ok(user)
}

#[cfg(test)]
mod tests {
    use crate::state::User;

    use super::*;
    use cosmwasm_std::testing::mock_dependencies;
    use cosmwasm_std::Addr;
    use cw_multi_test::{App, ContractWrapper, Executor};
    #[test]
    fn instantiate_fun() {
        let _deps = mock_dependencies();
        let mut app: App = App::default();
        let code = ContractWrapper::new(execute, instantiate, query);
        let code_id = app.store_code(Box::new(code));
        let u = User {
            username: String::from("abc"),
            age: 25,
            address: String::from("skjkfksk"),
            role: Role::HR,
        };
        let addr = app
            .instantiate_contract(
                code_id,
                Addr::unchecked("sender"),
                &InstatiateMsg { hr: u },
                &[],
                "HR added or instantiated",
                None,
            )
            .unwrap();
            let a=addr.clone();
        let queriedsuperadmin :String= app
            .wrap()
            .query_wasm_smart(addr, &QueryMsg::GetSuperAdmin {})
            .unwrap();
        println!("the superadmin is {queriedsuperadmin}");

        let ss=app.execute_contract(
            Addr::unchecked("abc"), 
            a.clone(), 
            &ExecuteMsg::AddEmployee { name: String::from("Vishal"), age: 2, address: String::from("cosmos"), role: Role::Employee}, 
            &[]).expect("Error executing the msg");
        println!("the execute contract has been done {:?}",ss);
        let res:Vec<UserDetails>=app.wrap().query_wasm_smart(a.clone(), &QueryMsg::GetEmployess {}).unwrap();
        println!("the result of the query is {:#?}",res);
    }
   
}

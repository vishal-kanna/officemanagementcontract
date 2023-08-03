use std::ops::Add;

use crate::error::ContractError;

use crate::msg::{ExecuteMsg, InstatiateMsg, QueryMsg};
use crate::state::{Role, UserDetails, ENTRY_SEQ, HR, USERS};
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
    ENTRY_SEQ.save(deps.storage, &0u64)?;
    let uid = ENTRY_SEQ.update::<_, cosmwasm_std::StdError>(deps.storage, |uid| Ok(uid.add(1)))?;
    let id = uid.clone();
    let supad = UserDetails {
        uid,
        username: msg.hr.username,
        age: msg.hr.age,
        address: msg.hr.address,
        role: Role::HR,
    };
    HR.save(deps.storage, &supad)
        .expect("Error in instantiating the admin");
    USERS.save(deps.storage, id, &supad).expect("Error while saving the super admin into the userdb");
    Ok(Response::new()
        .add_attribute("action", "HR instantiated")
        .add_attribute("user id ", id.to_string()))
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
    //checking if the sender matches the instatiated super admin
    if a.address == info.sender {
        let uid =
            ENTRY_SEQ.update::<_, cosmwasm_std::StdError>(dep.storage, |uid| Ok(uid.add(1)))?;
        id = uid.clone();
        let emp = UserDetails {
            uid,
            username: name,
            age,
            address,
            role,
        };
        USERS
            .save(dep.storage, id, &emp)
            .expect("Error while adding the employee");
    } else {
        return Err(ContractError::InstateError {});
    }
    Ok(Response::new()
        .add_attribute("Action", "employee added")
        .add_attribute("userid ", id.to_string()))
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
pub fn admin(dep: Deps) -> StdResult<UserDetails> {
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
            username: String::from("superadmin"),
            age: 25,
            address: String::from("cosmosabc"),
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
        let a = addr.clone();
        let _queriedsuperadmin: UserDetails = app
            .wrap()
            .query_wasm_smart(addr, &QueryMsg::GetSuperAdmin {})
            .unwrap();

        let _ss = app
            .execute_contract(
                Addr::unchecked("cosmosabc"),
                a.clone(),
                &ExecuteMsg::AddEmployee {
                    name: String::from("Vishal"),
                    age: 22,
                    address: String::from("cosmos"),
                    role: Role::Employee,
                },
                &[],
            )
            .expect("Error executing the msg");
        let _ss = app
            .execute_contract(
                Addr::unchecked("cosmosabc"),
                a.clone(),
                &ExecuteMsg::AddEmployee {
                    name: String::from("Hemanth"),
                    age: 21,
                    address: String::from("cosmosr"),
                    role: Role::Employee,
                },
                &[],
            )
            .expect("Error executing the msg");

        let r: Vec<UserDetails> = app
            .wrap()
            .query_wasm_smart(a.clone(), &QueryMsg::GetEmployess {})
            .unwrap();
        println!("All employees are {:#?}",r);

        let singleemploye:UserDetails=app.wrap().query_wasm_smart(a.clone(), &QueryMsg::GetEmployee { uid: 1 }).unwrap();
        println!("The single employee is {:#?}",singleemploye);
    }
}

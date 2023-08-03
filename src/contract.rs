use std::ops::Add;

use crate::error::ContractError;

use crate::msg::{ExecuteMsg, InstatiateMsg, QueryMsg};
use crate::state::{LeaveReq, Role, UserDetails, ENTRY_SEQ, HR, LEAVE_LIST, LEAVE_SEQ, USERS};
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
    LEAVE_SEQ.save(deps.storage, &0u128)?;
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
    USERS
        .save(deps.storage, id, &supad)
        .expect("Error while saving the super admin into the userdb");
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
        ExecuteMsg::Applyleave {
            id,
            start_date,
            end_date,
            reason,
        } => apply_leave(dep, env, info, id, start_date, end_date, reason),
        ExecuteMsg::AcceptLeave { leaveid }=>acceptleave(dep, info, leaveid),
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

pub fn apply_leave(
    dep: DepsMut,
    _env: Env,
    info: MessageInfo,
    id: u64,
    start_date: String,
    end_date: String,
    reason: String,
) -> Result<Response, ContractError> {
    //check if the student present or not
    //if present ,he may apply leave or else he shouldnot
    let res = USERS.load(dep.storage, id.clone());
    let leave: u128;
    match res {
        Ok(student_present) => {
            if info.sender == student_present.address {
                let leaveid = LEAVE_SEQ
                    .update::<_, cosmwasm_std::StdError>(dep.storage, |leaveid: u128| {
                        Ok(leaveid.add(1))
                    })?;
                leave = leaveid.clone();
                let leavereq = LeaveReq {
                    id: id.clone(),
                    start_date: start_date.to_string(),
                    end_date: end_date.to_string(),
                    status: "Pending".to_string(),
                    reason: reason.clone(),
                };
                LEAVE_LIST.save(dep.storage, leaveid, &leavereq)?;
            } else {
                return Err(ContractError::SenderNotMatched {});
            }
        }
        Err(_studentnotfound) => {
            return Err(ContractError::UserNotFound {});
        }
    }
    let res: Response<_> = Response::new()
        .add_attribute("action", "leave applied")
        .add_attribute("leaveid", leave.to_string());
    Ok(res)
}

pub fn listallleaves(deps: Deps) -> Vec<LeaveReq> {
    // let res=LEAVE_LIST.load(deps.storage)
    let mut res: Vec<LeaveReq> = Vec::new();
    for result in LEAVE_LIST.range(deps.storage, None, None, Order::Ascending) {
        match result {
            Ok(r) => {
                res.push(r.1);
            }
            Err(_res) => {
                println!("Error exists here");
            }
        }
    }
    res
}
pub fn acceptleave(
    deps: DepsMut,
    info: MessageInfo,
    leaveid: u128,
) -> Result<Response, ContractError> {
    let adminsaddress = HR.load(deps.storage)?.address;
    let mut flag=false;
        if adminsaddress == info.sender {
            let mut leave = LEAVE_LIST.load(deps.storage, leaveid)?;
            leave.status = "approved".to_string();
            let updated_leave = leave.clone();
            println!("the updated leave req is {:?}", updated_leave);
            LEAVE_LIST.save(deps.storage, leaveid, &updated_leave)?;
            flag=true;
    }
    if flag{
        Ok(Response::new())
    }else{
        return Err(ContractError::NotSuperAdmin { });
    }
}
#[cfg_attr(not(feature = "query"), entry_point)]
pub fn query(dep: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    use QueryMsg::*;
    match msg {
        GetEmployess {} => to_binary(&getemployees(dep)),
        GetEmployee { uid } => to_binary(&getemployee(dep, uid)?),
        GetSuperAdmin {} => to_binary(&admin(dep)?),
        ListLeaves {} => to_binary(&listallleaves(dep)),
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
        println!("All employees are {:#?}", r);

        let singleemploye: UserDetails = app
            .wrap()
            .query_wasm_smart(a.clone(), &QueryMsg::GetEmployee { uid: 1 })
            .unwrap();
        println!("The single employee is {:#?}", singleemploye);

        let _ss = app
            .execute_contract(
                Addr::unchecked("cosmosr"),
                a.clone(),
                &ExecuteMsg::Applyleave {
                    id: 3,
                    start_date: String::from("3-8-2023"),
                    end_date: String::from("5-8-2023"),
                    reason: String::from("Marriage"),
                },
                &[],
            )
            .expect("Error while applying the leave");
        let listleaves: Vec<LeaveReq> = app
            .wrap()
            .query_wasm_smart(a.clone(), &QueryMsg::ListLeaves {})
            .expect("error quering the leaves");
        println!("The leaves are {:#?}", listleaves);
        let accept=app.execute_contract(Addr::unchecked("cosmosabc"), a.clone(),&ExecuteMsg::AcceptLeave { leaveid: 1 }, &[]).expect("Error while accepting the leave");
        let listleaves: Vec<LeaveReq> = app
            .wrap()
            .query_wasm_smart(a.clone(), &QueryMsg::ListLeaves {})
            .expect("error quering the leaves");
    }
}
/*  let _ss = app
.execute_contract(
    Addr::unchecked("cosmosabc"),
    a.clone(),
    &ExecuteMsg::Applyleave{
        id: 3,
        start_date: String::from("3-8-2023"),
        end_date: String::from("5-8-2023"),
        reason: String::from("Marriage")
    },
    &[],
)
.expect("Error while applying the leave");
     */

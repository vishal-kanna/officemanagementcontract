use chrono::NaiveDate;
use std::ops::Add;

use crate::error::ContractError;

use crate::msg::{ExecuteMsg, InstatiateMsg, QueryMsg};
use crate::state::{
    LeaveReq, Leavetype, Leavetype1, Role, UserDetails, UserDetails1, ENTRY_SEQ, HR, LEAVETYPE_SEQ,
    LEAVE_LIST, LEAVE_SEQ, LEAVE_TYPES, USERS, USERS_LEAVES,
};
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
    LEAVETYPE_SEQ.save(deps.storage, &0u64)?;
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
    let mut leaves: Vec<Leavetype> = Vec::new();
    let cas = Leavetype {
        types: String::from("Casual Leave"),
        count: 10,
    };
    let sick = Leavetype {
        types: String::from("Sick Leave"),
        count: 12,
    };
    leaves.push(cas);
    leaves.push(sick);
    let length = leaves.len();
    for i in 0..length {
        LEAVE_TYPES
            .save(deps.storage, (i + 1) as u64, &leaves[i])
            .expect("Error while adding the leaves to Leave types ");
    }
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
            leavetypeid,
            from,
            to,
            reason,
        } => apply_leave(dep, env, info, id, leavetypeid, from, to, reason),
        ExecuteMsg::AcceptLeave { leaveid } => acceptleave(dep, info, leaveid),
        ExecuteMsg::AddLeaveType { newleave } => addnewleave(dep, info, newleave),
    }
}
pub fn addnewleave(
    dep: DepsMut,
    info: MessageInfo,
    newleave: Leavetype,
) -> Result<Response, ContractError> {
    let mut i = 0;
    for l in LEAVE_TYPES.range(dep.storage, None, None, Order::Ascending) {
        match l {
            Ok(l) => {
                i = i + 1;
                println!("the leaves are {:?} and i value is {}", l, i);
            }
            Err(_) => {
                println!("Error");
            }
        }
    }
    let newleave1 = Leavetype1 {
        id: i + 1,
        types: newleave.types.clone(),
        count: newleave.count.clone(),
    };
    let mut lev: Vec<Leavetype1>;
    // let uid :u64;
    let mut v: Vec<UserDetails1> = Vec::new();
    for result in USERS_LEAVES.range(dep.storage, None, None, Order::Ascending) {
        match result {
            Ok(res) => {
                lev = res.1.leaves;
                lev.push(newleave1.clone());
                let newdata = UserDetails1 {
                    leaves: lev,
                    ..res.1
                };
                v.push(newdata);
            }
            Err(_res) => {}
        }
    }
    for emp in v {
        USERS_LEAVES
            .save(dep.storage, emp.uid, &emp)
            .expect("Error while adding the new leave to the employee");
    }

    LEAVE_TYPES
        .save(dep.storage, i + 1, &newleave.clone())
        .expect("Error while adding the new leave");
    Ok(Response::new().add_attribute("Action", "employee added"))
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
            username: name.clone(),
            age,
            address: address.clone(),
            role: role.clone(),
        };
        let mut leav: Vec<Leavetype1> = Vec::new();
        for k in LEAVE_TYPES.range(dep.storage, None, None, Order::Ascending) {
            let a = k.unwrap();
            let b = Leavetype1 {
                id: a.0,
                types: a.1.types,
                count: a.1.count,
            };
            leav.push(b);
        }
        let fulldetails = UserDetails1 {
            uid: uid.clone(),
            username: name.clone(),
            age: age.clone(),
            address: address.clone(),
            role: role.clone(),
            leaves: leav,
        };
        USERS_LEAVES
            .save(dep.storage, uid, &fulldetails)
            .expect("Error while adding the leaves in full details");
        USERS
            .save(dep.storage, id, &emp)
            .expect("Error while adding the employee");
    } else {
        // println!("IM here toooooo!!!!!!!");
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
    leavetypeid: u64,
    from: String,
    to: String,
    reason: String,
) -> Result<Response, ContractError> {
    //check if the student present or not
    //if present ,he may apply leave or else he shouldnot
    let res = USERS_LEAVES.load(dep.storage, id.clone());
    let leave: u128;
    match res {
        Ok(student_present) => {
            if info.sender == student_present.address {
                let leaveid = LEAVE_SEQ
                    .update::<_, cosmwasm_std::StdError>(dep.storage, |leaveid: u128| {
                        Ok(leaveid.add(1))
                    })?;
                let s_date: NaiveDate =
                    NaiveDate::parse_from_str(&from.clone(), "%d-%m-%Y").unwrap();
                let e_date = NaiveDate::parse_from_str(&to.clone(), "%d-%m-%Y").unwrap();
                // let diff=e_date-s_date;
                // let nooffays=diff.num_days();
                if s_date < e_date {
                    leave = leaveid.clone();
                    let leavereq = LeaveReq {
                        id: id.clone(),
                        leavetypeid,
                        from: from.to_string(),
                        to: to.to_string(),
                        status: "Pending".to_string(),
                        reason: reason.clone(),
                    };
                    LEAVE_LIST.save(dep.storage, leaveid, &leavereq)?;
                } else {
                    return Err(ContractError::WrongDates {});
                }
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
    let mut flag = false;
    if adminsaddress == info.sender {
        let mut leave: LeaveReq = LEAVE_LIST.load(deps.storage, leaveid)?;
        let s_date: NaiveDate = NaiveDate::parse_from_str(&leave.from.clone(), "%d-%m-%Y").unwrap();
        let e_date = NaiveDate::parse_from_str(&leave.to.clone(), "%d-%m-%Y").unwrap();
        let diff = e_date - s_date;
        let nooffays = diff.num_days();
        let userid = leave.id;
        let userspecified = USERS_LEAVES.load(deps.storage, userid)?;
        let u = userspecified.clone();
        let all_leaves = userspecified.leaves;
        let leavetypeid = leave.leavetypeid;
        let mut updatedleaves: Vec<Leavetype1> = Vec::new();

        for leaves in all_leaves {
            // updatedleaves.push(leaves);
            if leaves.id.eq(&leavetypeid) {
                println!("the leave id type is {:?}", leaves.types);
                if leaves.count >= nooffays as u64 {
                    let a = leaves.count - nooffays as u64;

                    let updatedleavetype = Leavetype1 {
                        id: leaves.id,
                        types: leaves.types,
                        count: a,
                    };
                    updatedleaves.push(updatedleavetype);
                } else {
                    return Err(ContractError::NoLeaves {});
                }
            } else {
                println!("Im running");
                updatedleaves.push(leaves);
            }
        }
        let updateduser = UserDetails1 {
            leaves: updatedleaves,
            ..u
        };
        USERS_LEAVES
            .save(deps.storage, userid, &updateduser)
            .expect("Error occured while adding the updated leave of the user");
        leave.status = "approved".to_string();
        let updated_leave: LeaveReq = leave.clone();
        println!("the updated leave req is {:?}", updated_leave);
        LEAVE_LIST.save(deps.storage, leaveid, &updated_leave)?;
        flag = true;
    }
    if flag {
        Ok(Response::new())
    } else {
        return Err(ContractError::NotSuperAdmin {});
    }
}
#[cfg_attr(not(feature = "query"), entry_point)]
pub fn query(dep: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    use QueryMsg::*;
    match msg {
        GetEmployess {} => to_binary(&getemployees1(dep)),
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
pub fn getemployees1(dep: Deps) -> Vec<UserDetails1> {
    let mut re: Vec<UserDetails1> = Vec::new();
    for result in USERS_LEAVES.range(dep.storage, None, None, Order::Ascending) {
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
            .expect("Error executing addemploee function");
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
            .expect("Error executing addemploee function");

        let r: Vec<UserDetails1> = app
            .wrap()
            .query_wasm_smart(a.clone(), &QueryMsg::GetEmployess {})
            .unwrap();
        println!("All employees are {:#?}", r);

        let singleemploye: UserDetails = app
            .wrap()
            .query_wasm_smart(a.clone(), &QueryMsg::GetEmployee { uid: 1 })
            .unwrap();
        println!("The single employee is {:#?}", singleemploye);

        let ss = app
            .execute_contract(
                Addr::unchecked("cosmosr"),
                a.clone(),
                &ExecuteMsg::Applyleave {
                    id: 3,
                    leavetypeid: 1,
                    from: String::from("3-8-2023"),
                    to: String::from("6-8-2023"),
                    reason: String::from("Marriage"),
                },
                &[],
            )
            .expect("Error while applying the leave");
        println!("the events after the applying leaves are {:#?}", ss);
        let listleaves: Vec<LeaveReq> = app
            .wrap()
            .query_wasm_smart(a.clone(), &QueryMsg::ListLeaves {})
            .expect("error quering the leaves");
        println!("The leaves are {:#?}", listleaves);
        let _accept = app
            .execute_contract(
                Addr::unchecked("cosmosabc"),
                a.clone(),
                &ExecuteMsg::AcceptLeave { leaveid: 1 },
                &[],
            )
            .expect("Error while accepting the leave");
        let _listleaves: Vec<LeaveReq> = app
            .wrap()
            .query_wasm_smart(a.clone(), &QueryMsg::ListLeaves {})
            .expect("error quering the leaves");

        let singleemploye: Vec<UserDetails1> = app
            .wrap()
            .query_wasm_smart(a.clone(), &QueryMsg::GetEmployess {})
            .unwrap();
        println!("The single employee is {:#?}", singleemploye);

        let _accept = app
            .execute_contract(
                Addr::unchecked("cosmosabc"),
                a.clone(),
                &ExecuteMsg::AddLeaveType {
                    newleave: Leavetype {
                        types: String::from("Holiday"),
                        count: 6,
                    },
                },
                &[],
            )
            .expect("Error while accepting the leave");

        let accept = app
            .execute_contract(
                Addr::unchecked("cosmosabc"),
                a.clone(),
                &ExecuteMsg::AddLeaveType {
                    newleave: Leavetype {
                        types: String::from("WFH"),
                        count: 4,
                    },
                },
                &[],
            )
            .expect("Error while accepting the leave");
        let singleemploye: Vec<UserDetails1> = app
            .wrap()
            .query_wasm_smart(a.clone(), &QueryMsg::GetEmployess {})
            .unwrap();
        println!("The single employee is {:#?}", singleemploye);

        //         let ss = app
        //         .execute_contract(
        //             Addr::unchecked("cosmosr"),
        //             a.clone(),
        //             &ExecuteMsg::Applyleave {
        //                 id: 3,
        //                 leavetypeid:3,
        //                 from: String::from("3-8-2023"),
        //                 to: String::from("6-8-2023"),
        //                 reason: String::from("WFH"),
        //             },
        //             &[],
        //         )
        //         .expect("Error while applying the leave");
        //     println!("the events after the applying leaves are {:#?}",ss);

        //     println!("The single employee is {:#?}", singleemploye);
        //     let _accept=app.execute_contract(Addr::unchecked("cosmosabc"), a.clone(),&ExecuteMsg::AcceptLeave { leaveid: 2 }, &[]).expect("Error while accepting the leave");
        //     let _listleaves: Vec<LeaveReq> = app
        //     .wrap()
        //     .query_wasm_smart(a.clone(), &QueryMsg::ListLeaves {})
        //     .expect("error quering the leaves");
        // let singleemploye: Vec<UserDetails1> = app
        // .wrap()
        // .query_wasm_smart(a.clone(), &QueryMsg::GetEmployess {})
        // .unwrap();
        // println!("The single employee is {:#?}", singleemploye);
        // let singleemploye: UserDetails = app
        // .wrap()
        // .query_wasm_smart(a.clone(), &QueryMsg::GetEmployee { uid: 3 })
        // .unwrap();
        // println!("The single employee is {:#?}", singleemploye);
    }
}

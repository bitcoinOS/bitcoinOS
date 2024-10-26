use crate::{context::STATE, domain::CanisterModuleInfo, error::Error};

pub(crate) fn add_canister_module(canister_module_info: CanisterModuleInfo) -> Result<bool, Error> {
    STATE.with(|s| {
        let mut state = s.borrow_mut();
        let canister_name = canister_module_info.canister_name.clone();
        let canister_module = &mut state.canister_module;
        if canister_module.contains_key(&canister_name.clone()) {
            Err(Error::CanisterAlreadyExists {
                canister_name: canister_name.clone(),
            })
        } else {
            canister_module.insert(canister_name, canister_module_info.clone());
            Ok(true)
        }
    })
}

pub(crate) fn get_canister_module(canister_name: String) -> Option<CanisterModuleInfo> {
    STATE.with(|s| {
        let state = s.borrow();
        state.canister_module.get(&canister_name)
    })
}

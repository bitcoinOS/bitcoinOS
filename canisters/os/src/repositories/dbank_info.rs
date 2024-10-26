use crate::{context::STATE, domain::DBankInfo, error::Error};

pub(crate) fn current_dbank_info() -> Option<DBankInfo> {
    STATE.with_borrow(|s| s.dbank_infos.last_key_value().map(|(_, v)| v))
}

pub(crate) fn get(key: &u64) -> Option<DBankInfo> {
    STATE.with_borrow(|s| s.dbank_infos.get(key))
}

pub(crate) fn list_dbank() -> Vec<DBankInfo> {
    STATE.with_borrow(|s| s.dbank_infos.iter().map(|(_, v)| v).collect())
}

pub(crate) fn save(info: DBankInfo) -> Result<(), Error> {
    STATE.with_borrow_mut(|s| {
        if s.dbank_infos.contains_key(&info.dbank_id) {
            Err(Error::CanisterAlreadyExists {
                canister_name: info.dbank_canister.to_string(),
            })
        } else {
            s.dbank_infos.insert(info.dbank_id, info);
            Ok(())
        }
    })
}

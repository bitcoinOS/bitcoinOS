use crate::{domain::DBankInfo, repositories};

pub fn serve() -> Vec<DBankInfo> {
    repositories::dbank_info::list_dbank()
}

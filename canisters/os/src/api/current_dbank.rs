use crate::{domain::DBankInfo, repositories};

pub fn serve() -> Option<DBankInfo> {
    repositories::dbank_info::current_dbank_info()
}

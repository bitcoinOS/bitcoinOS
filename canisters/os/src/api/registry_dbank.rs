use crate::{domain::DBankInfo, error::Error, repositories};

pub(crate) fn serve(info: DBankInfo) -> Result<(), Error> {
    repositories::dbank_info::save(info)
}

use ic_stable_structures::cell::ValueError;

use crate::{context::STATE, domain::ConfigSetting};
pub fn get_config_setting() -> ConfigSetting {
    STATE.with_borrow(|s| s.config_setting.get().to_owned())
}

pub fn save_config(conf: ConfigSetting) -> Result<ConfigSetting, ValueError> {
    STATE.with_borrow_mut(|s| s.config_setting.set(conf))
}

use crate::{domain::ConfigSetting, repositories};

pub fn serve() -> ConfigSetting {
    repositories::config_setting::get_config_setting()
}

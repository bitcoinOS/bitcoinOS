use crate::{domain::ConfigSetting, repositories};

pub fn serve(config: ConfigSetting) {
    let _ = repositories::config_setting::save_config(config);
}

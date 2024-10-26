use crate::{
    domain::{request::TimerSettingsRequest, TimerSettings},
    repositories,
};

pub(super) fn serve(req: TimerSettingsRequest) -> Result<TimerSettings, String> {
    let current_settings = repositories::timer_settings::get();

    let new_settings = TimerSettings {
        save_point_in_secs: req
            .save_point_in_secs
            .unwrap_or(current_settings.save_point_in_secs),
        create_staking_record_in_secs: req
            .create_staking_record_in_secs
            .unwrap_or(current_settings.create_staking_record_in_secs),
        update_btc_price_in_secs: req
            .update_btc_price_in_secs
            .unwrap_or(current_settings.update_btc_price_in_secs),
        update_leaderboard_in_secs: req
            .update_leaderboard_in_secs
            .unwrap_or(current_settings.update_leaderboard_in_secs),
    };

    repositories::timer_settings::save(new_settings)
        .map_err(|e| format!("save timer settings error: {e:?}"))
}

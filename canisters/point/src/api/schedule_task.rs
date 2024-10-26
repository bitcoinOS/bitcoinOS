use wallet::utils::{add_now_n_day, get_time_for_day, ic_time};

use crate::{context::STATE, repositories};
use std::time::Duration;

use super::{create_stake_record, update_btc_price, update_leader_board, update_user_stat};
pub fn serve() {
    repositories::task_record::stop_task();

    // let timer_config = STATE.with_borrow(|s| s.timer_settings.get().clone());

    // let point_task_id = ic_cdk_timers::set_timer_interval(
    //     Duration::from_secs(timer_config.save_point_in_secs),
    //     || {
    //         // COUNTER.fetch_add(1, Ordering::Relaxed);
    //         ic_cdk::spawn(save_points_bak::serve());
    //     },
    // );

    // repositories::task_record::new_task_record(point_task_id);

    // let stake_task_id = ic_cdk_timers::set_timer_interval(
    //     Duration::from_secs(timer_config.create_staking_record_in_secs),
    //     || {
    //         // WALLET_COUNTER.fetch_add(1, Ordering::Relaxed);
    //         ic_cdk::spawn(create_stake_record::serve());
    //     },
    // );

    // repositories::task_record::new_task_record(stake_task_id);

    // let leader_board = ic_cdk_timers::set_timer_interval(
    //     Duration::from_secs(timer_config.update_leaderboard_in_secs),
    //     || {
    //         // Update leader board
    //         ic_cdk::spawn(update_leader_board::serve());
    //     },
    // );

    // repositories::task_record::new_task_record(leader_board);

    // // add user stat
    // ic_cdk_timers::set_timer(Duration::from_secs(10), || {
    //     update_user_stat_task();
    // });
}

fn update_user_stat_task() {
    let now = ic_time();
    let next_day = add_now_n_day(1);
    //delay 600s
    let duration = (next_day + 600) * 1000000000 - now;
    ic_cdk_timers::set_timer(Duration::from_nanos(duration), || {
        update_user_stat_task();
    });
    update_user_stat::serve();
}

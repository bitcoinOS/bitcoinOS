use std::cell::RefCell;

use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    DefaultMemoryImpl, RestrictedMemory,
};

pub type DefMem = DefaultMemoryImpl;
pub type RM = RestrictedMemory<DefMem>;
pub type VM = VirtualMemory<RM>;

pub type Memory = VirtualMemory<DefMem>;

// A memory for the StableBTreeMap we're using. A new memory should be created for
// every additional stable structure.
const METADATA_MEMORY_ID: MemoryId = MemoryId::new(1);
// const POINT_RECORDS_MEMORY_ID: MemoryId = MemoryId::new(2);
// const NEXT_PERIOD_MEMORY_ID: MemoryId = MemoryId::new(3);
const BTC_PRICE_MEMORY_ID: MemoryId = MemoryId::new(2);
//box counter
// const  BOX_COUNTER_MEMORY_ID: MemoryId = MemoryId::new(5);
// user all reward
const REWARD_RECORD_MEMORY_ID: MemoryId = MemoryId::new(3);
// box reward detail
// const BOX_REWARD_DETAIL_MEMORY_ID: MemoryId = MemoryId::new(4);
// box record
const BOX_RECORD_MEMORY_ID: MemoryId = MemoryId::new(4);

const BOX_REWARD_MEMORY_ID: MemoryId = MemoryId::new(5);
const INVITE_REWARD_MEMORY_ID: MemoryId = MemoryId::new(6);
const STAKE_REWARD_MEMORY_ID: MemoryId = MemoryId::new(7);

const CONFIG_MEMORY_ID: MemoryId = MemoryId::new(8);

const LEADER_BOARD_MEMORY_ID: MemoryId = MemoryId::new(9);

const USER_RANK_MEMORY_ID: MemoryId = MemoryId::new(10);

const TIMER_SETTINGS_MEMORY_ID: MemoryId = MemoryId::new(11);

const USER_STAT_MEMORY_ID: MemoryId = MemoryId::new(12);

const USER_STAKE_RANK_MEMORY_ID: MemoryId = MemoryId::new(13);

const POINT_ADMIN_MEMORY_ID: MemoryId = MemoryId::new(14);

const STAKED_NFT_MEMORY_ID: MemoryId = MemoryId::new(15);

//invite record
// const USER_STATUS_RECORD_MEMORY_ID: MemoryId = MemoryId::new(9);

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );
}

pub(super) fn get_metadata_memory() -> Memory {
    MEMORY_MANAGER.with(|m| m.borrow().get(METADATA_MEMORY_ID))
}

// pub(super) fn get_total_point_memory() -> Memory {
//     MEMORY_MANAGER.with(|m| m.borrow().get(TOTAL_POINT_MEMORY_ID))
// }

// pub(super) fn get_point_records_memory() -> Memory {
//     MEMORY_MANAGER.with(|m| m.borrow().get(POINT_RECORDS_MEMORY_ID))
// }

// pub(super) fn get_next_period_memory() -> Memory {
//     MEMORY_MANAGER.with(|m| m.borrow().get(NEXT_PERIOD_MEMORY_ID))
// }

pub(super) fn get_btc_price_memory() -> Memory {
    MEMORY_MANAGER.with(|m| m.borrow().get(BTC_PRICE_MEMORY_ID))
}

// pub(super) fn get_box_counter_memory() -> Memory {
//     MEMORY_MANAGER.with(|m| m.borrow().get(BOX_COUNTER_MEMORY_ID))
// }

pub(super) fn get_reward_record_memory() -> Memory {
    MEMORY_MANAGER.with(|m| m.borrow().get(REWARD_RECORD_MEMORY_ID))
}

// pub(super) fn get_box_reward_detail_memory() -> Memory {
//     MEMORY_MANAGER.with(|m| m.borrow().get(BOX_REWARD_DETAIL_MEMORY_ID))
// }

pub(super) fn get_box_record_memory() -> Memory {
    MEMORY_MANAGER.with(|m| m.borrow().get(BOX_RECORD_MEMORY_ID))
}

// pub(super) fn get_user_status_record_memory() -> Memory {
//     MEMORY_MANAGER.with(|m| m.borrow().get(USER_STATUS_RECORD_MEMORY_ID))
// }

pub(super) fn get_box_reward_memory() -> Memory {
    MEMORY_MANAGER.with(|m| m.borrow().get(BOX_REWARD_MEMORY_ID))
}

pub(super) fn get_invite_reward_memory() -> Memory {
    MEMORY_MANAGER.with(|m| m.borrow().get(INVITE_REWARD_MEMORY_ID))
}

pub(super) fn get_stake_reward_memory() -> Memory {
    MEMORY_MANAGER.with(|m| m.borrow().get(STAKE_REWARD_MEMORY_ID))
}

pub(super) fn get_config_memory() -> Memory {
    MEMORY_MANAGER.with(|m| m.borrow().get(CONFIG_MEMORY_ID))
}

pub(super) fn get_learder_board_memory() -> Memory {
    MEMORY_MANAGER.with(|m| m.borrow().get(LEADER_BOARD_MEMORY_ID))
}

pub(super) fn get_user_rank_memory() -> Memory {
    MEMORY_MANAGER.with(|m| m.borrow().get(USER_RANK_MEMORY_ID))
}

pub(super) fn get_timer_settings_memory() -> Memory {
    MEMORY_MANAGER.with(|m| m.borrow().get(TIMER_SETTINGS_MEMORY_ID))
}

pub(super) fn get_user_stat_memory() -> Memory {
    MEMORY_MANAGER.with(|m| m.borrow().get(USER_STAT_MEMORY_ID))
}

pub(super) fn get_user_stake_rank_memory() -> Memory {
    MEMORY_MANAGER.with(|m| m.borrow().get(USER_STAKE_RANK_MEMORY_ID))
}

pub(super) fn get_point_admin_memory() -> Memory {
    MEMORY_MANAGER.with(|m| m.borrow().get(POINT_ADMIN_MEMORY_ID))
}

pub(super) fn get_staked_nft_memory() -> Memory {
    MEMORY_MANAGER.with(|m| m.borrow().get(STAKED_NFT_MEMORY_ID))
}

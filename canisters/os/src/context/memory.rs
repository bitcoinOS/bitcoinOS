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
const WALLET_COUNTER_MEMORY_ID: MemoryId = MemoryId::new(2);
const STAKING_POOL_COUNTER_MEMORY_ID: MemoryId = MemoryId::new(3);
const WALLET_OWNER_MEMORY_ID: MemoryId = MemoryId::new(4);
const ACTION_LOG_IDX_MEM_ID: MemoryId = MemoryId::new(5);
const ACTION_LOG_DATA_MEM_ID: MemoryId = MemoryId::new(6);
const STAKING_POOL_MEMORY_ID: MemoryId = MemoryId::new(7);
const WALLET_INFO_MEMORY_ID: MemoryId = MemoryId::new(8);
const STAKING_RECORD_MEMORY_ID: MemoryId = MemoryId::new(9);
const CANISTER_MODULE_MEMORY_ID: MemoryId = MemoryId::new(10);

const USER_INFO_MEMORY_ID: MemoryId = MemoryId::new(11);
// const LOGIN_LOG_MEMORY_ID: MemoryId = MemoryId::new(12);
const USER_COUNTER_MEMORY_ID: MemoryId = MemoryId::new(13);
const USER_INVITE_CODE_MEMORY_ID: MemoryId = MemoryId::new(14);

const DBANK_WALLET_INFO_MEMORY_ID: MemoryId = MemoryId::new(15);
const DBANK_INFO_MEMORY_ID: MemoryId = MemoryId::new(16);

const WALLET_BIND_MEMORY_ID: MemoryId = MemoryId::new(17);

const WALLET_MAP_BIND_MEMORY_ID: MemoryId = MemoryId::new(18);

const USER_BIND_COUNT_MEMORY_ID: MemoryId = MemoryId::new(19);

const USER_IMAGE_LINK_MEMORY_ID: MemoryId = MemoryId::new(20);

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );
}

pub(super) fn get_metadata_memory() -> Memory {
    MEMORY_MANAGER.with(|m| m.borrow().get(METADATA_MEMORY_ID))
}

pub(super) fn get_wallet_counter_memory() -> Memory {
    MEMORY_MANAGER.with(|m| m.borrow().get(WALLET_COUNTER_MEMORY_ID))
}

pub(super) fn get_staking_pool_counter_memory() -> Memory {
    MEMORY_MANAGER.with(|m| m.borrow().get(STAKING_POOL_COUNTER_MEMORY_ID))
}

pub(super) fn get_wallet_owner_memory() -> Memory {
    MEMORY_MANAGER.with(|m| m.borrow().get(WALLET_OWNER_MEMORY_ID))
}

pub(super) fn get_action_log_index_memory() -> Memory {
    MEMORY_MANAGER.with(|m| m.borrow().get(ACTION_LOG_IDX_MEM_ID))
}

pub(super) fn get_action_log_data_memory() -> Memory {
    MEMORY_MANAGER.with(|m| m.borrow().get(ACTION_LOG_DATA_MEM_ID))
}

pub(super) fn get_staking_pool_memory() -> Memory {
    MEMORY_MANAGER.with(|m| m.borrow().get(STAKING_POOL_MEMORY_ID))
}

pub(super) fn get_wallet_info_memory() -> Memory {
    MEMORY_MANAGER.with(|m| m.borrow().get(WALLET_INFO_MEMORY_ID))
}

pub(super) fn get_dbank_wallet_info_memory() -> Memory {
    MEMORY_MANAGER.with(|m| m.borrow().get(DBANK_WALLET_INFO_MEMORY_ID))
}

pub(super) fn get_dbank_info_memory() -> Memory {
    MEMORY_MANAGER.with(|m| m.borrow().get(DBANK_INFO_MEMORY_ID))
}

pub(super) fn get_staking_record_memory() -> Memory {
    MEMORY_MANAGER.with(|m| m.borrow().get(STAKING_RECORD_MEMORY_ID))
}

pub(super) fn get_canister_module_memory() -> Memory {
    MEMORY_MANAGER.with(|m| m.borrow().get(CANISTER_MODULE_MEMORY_ID))
}

pub(super) fn get_user_info_memory() -> Memory {
    MEMORY_MANAGER.with(|m| m.borrow().get(USER_INFO_MEMORY_ID))
}

// pub(super) fn get_login_log_memory() -> Memory {
//     MEMORY_MANAGER.with(|m| m.borrow().get(LOGIN_LOG_MEMORY_ID))
// }

pub(super) fn get_user_counter_memory() -> Memory {
    MEMORY_MANAGER.with(|m| m.borrow().get(USER_COUNTER_MEMORY_ID))
}

pub(super) fn get_user_invite_code_memory() -> Memory {
    MEMORY_MANAGER.with(|m| m.borrow().get(USER_INVITE_CODE_MEMORY_ID))
}

pub(super) fn get_wallet_bind_memory() -> Memory {
    MEMORY_MANAGER.with(|m| m.borrow().get(WALLET_BIND_MEMORY_ID))
}

pub(super) fn get_wallet_map_bind_memory() -> Memory {
    MEMORY_MANAGER.with(|m| m.borrow().get(WALLET_MAP_BIND_MEMORY_ID))
}
pub(super) fn get_user_bind_count_memory() -> Memory {
    MEMORY_MANAGER.with(|m| m.borrow().get(USER_BIND_COUNT_MEMORY_ID))
}

pub(super) fn get_user_image_link_memory() -> Memory {
    MEMORY_MANAGER.with(|m| m.borrow().get(USER_IMAGE_LINK_MEMORY_ID))
}

use crate::{context::STATE, domain::PriceRecord};

pub(crate) fn get_btc_price() -> PriceRecord {
    STATE.with_borrow(|s| s.btc_price.get().clone())
}

pub(crate) fn save_btc_price(p: PriceRecord) {
    STATE.with_borrow_mut(|s| {
        let _ = s.btc_price.set(p);
    })
}

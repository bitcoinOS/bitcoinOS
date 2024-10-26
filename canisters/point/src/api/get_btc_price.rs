use crate::domain::PriceRecord;
use crate::repositories;
pub fn serve() -> PriceRecord {
    repositories::btc_price::get_btc_price()
}

use crate::{domain::PriceRecord, repositories};
use ic_cdk::api::management_canister::http_request::HttpHeader;
use serde_json::{self, Value};
use wallet::utils::http_get;

async fn get_btc_price() -> Option<PriceRecord> {
    let url =
        "https://pro-api.coinmarketcap.com/v2/tools/price-conversion?id=1&amount=1".to_string();
    let request_headers = vec![
        HttpHeader {
            name: "X-CMC_PRO_API_KEY".to_string(),
            value: "47cf75a2-0caa-4f2b-8b62-35ae73379a05".to_string(),
        },
        HttpHeader {
            name: "Accept".to_string(),
            value: "47cf75a2-0caa-4f2b-8b62-35ae73379a05".to_string(),
        },
    ];
    let res = http_get(url, request_headers, 20_850_013_600).await;
    match res {
        Ok(d) => {
            let r: Value = serde_json::from_str(d.as_str()).unwrap();
            let data = r.get("data").unwrap();
            let price = data
                .get("quote")
                .unwrap()
                .get("USD")
                .unwrap()
                .get("price")
                .unwrap()
                .as_f64()
                .unwrap();
            Some(PriceRecord {
                message: "".to_string(),
                price: (price * 100.0) as u64,
                updated_time: ic_cdk::api::time(),
            })
        }
        Err(e) => Some(PriceRecord {
            message: e.to_string(),
            price: 0,
            updated_time: ic_cdk::api::time(),
        }),
    }
}

pub async fn serve() {
    let price = get_btc_price().await;
    if let Some(p) = price {
        repositories::btc_price::save_btc_price(p);
    }
}

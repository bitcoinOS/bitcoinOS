// use std::collections::HashMap;

// // use crate::domain::StakingPoolInfo;
// use crate::repositories::metadata::get_metadata;

// use ic_cdk::api::management_canister::bitcoin::BitcoinNetwork;

// use wallet::constants::{BOOST_RATE, MINIMUM_STAKE_AMOUNT};
// use wallet::domain::request::RegisterStakingRecordRequest;
// use wallet::domain::staking::StakingRecord;
// use wallet::domain::staking::{StakingPoolInfo, StakingType};
// use wallet::error::StakingError;
// use wallet::utils::{get_siwb_principal, http_get, ic_time, is_anonymous};

// // use wallet::macros;
// use serde_json::{self, Value};

// async fn get_tx_input_address(txid: String, network: BitcoinNetwork) -> Option<String> {
//     let host = match network {
//         BitcoinNetwork::Mainnet => "mempool.space",
//         BitcoinNetwork::Regtest => "",
//         BitcoinNetwork::Testnet => "mempool.space/testnet",
//     };
//     let url = format!("https://{}/api/tx/{}", host, txid.clone());
//     let res = http_get(url, vec![], 20_859_000_000).await;

//     match res {
//         Ok(d) => {
//             let r: Value = serde_json::from_str(d.as_str()).unwrap();
//             let vin = r.get("vin").unwrap();
//             let vins = vin.as_array().unwrap();
//             let wallet_address = vins[0]
//                 .get("prevout")
//                 .unwrap()
//                 .get("scriptpubkey_address")
//                 .unwrap()
//                 .as_str()
//                 .unwrap()
//                 .to_string();
//             log!(format!(
//                 "get tx wallet address,txid:{},input wallet address:{}",
//                 txid,
//                 wallet_address.clone()
//             ));
//             // Some("tb1qmh32acg9advekrrek59gypmgzrtm8w6hd5mplh".to_string())
//             Some(wallet_address.to_string())
//         }
//         Err(e) => {
//             log!(format!("err:{}", e.to_string()));
//             // Some("tb1qmh32acg9advekrrek59gypmgzrtm8w6hd5mplh".to_string())
//             None
//         }
//     }
// }

// pub async fn serve1() {
//     let metadata = get_metadata();
//     let resp: Result<(Vec<StakingPoolInfo>,), _> =
//         ic_cdk::call(metadata.os_canister, "list_staking_pool", ((),)).await;
//     let stake_pools = resp.map(|b| b.0).expect("get stake pool error");
//     for p in stake_pools {
//         log!(format!(
//             "utxo info,pool address:{}",
//             p.bitcoin_address.clone()
//         ));
//         let utxos = wallet::bitcoins::get_utxos(p.bitcoin_address.clone(), metadata.network, None)
//             .await
//             // .map_err(|e| e)
//             .unwrap()
//             .utxos;
//         log!(format!(
//             "utxo info,pool address:{},utxos count:{}",
//             p.bitcoin_address.clone(),
//             utxos.len()
//         ));
//         let stake_record_resp: Result<(Result<Vec<StakingRecord>, StakingError>,), _> =
//             ic_cdk::call(p.staking_pool_canister, "list_staking", ((),)).await;

//         let stake_records = match stake_record_resp {
//             Ok((Ok(s),)) => s,
//             _ => panic!("get stake pool error"),
//         };
//         // ic_cdk::print("in create 3 \n");
//         let mut record_map = HashMap::new();
//         for r in stake_records {
//             record_map.insert(r.txid.clone(), r.actual_amount);
//         }
//         let mut tx_utxo_map = HashMap::new();

//         for utxo in utxos.iter() {
//             let txid = utxo.outpoint.txid.clone();
//             let actual_amount = utxo.value;
//             let old_utxo = tx_utxo_map.get(txid.as_str());
//             if let Some(v) = old_utxo {
//                 tx_utxo_map.insert(txid.clone(), v + actual_amount);
//             } else {
//                 tx_utxo_map.insert(txid.clone(), actual_amount);
//             }
//         }

//         let mini_ammount = if let Some(m) = p.minimum_stake_amount {
//             m
//         } else {
//             MINIMUM_STAKE_AMOUNT
//         };

//         // // ic_cdk::print("in create 5 \n");
//         // TODO: 建议 在stakingpool canister 增加一个接受批量数据的接口
//         // 不要在循环里调用接口
//         for (tx, utxo) in tx_utxo_map.into_iter() {
//             if !record_map.contains_key(tx.as_str()) {
//                 // ic_cdk::print(format!("in create 6 \n{}",tx.clone()));
//                 let wallet_address = get_tx_input_address(tx.clone(), metadata.network).await;
//                 // let test_tx = "87f56e14c4c295fec1b79a0d2d5c120795393e105b98b2f204fd72fa3c05070f".to_string();
//                 // let wallet_address = get_tx_input_address(test_tx, BitcoinNetwork::Testnet).await;
//                 if utxo < mini_ammount {
//                     continue;
//                 }
//                 // ic_cdk::print(format!("in create 7 {}\n",wallet_address.clone().unwrap()));
//                 match metadata.siwb_canister {
//                     Some(c) => {
//                         if let Some(w) = wallet_address.clone() {
//                             let sender_principal = get_siwb_principal(c, w.clone()).await;
//                             log!(format!("ic swib:{}", sender_principal.clone().to_string()));
//                             if !is_anonymous(sender_principal) {
//                                 let req = RegisterStakingRecordRequest {
//                                     txid: tx.clone(),
//                                     sender: sender_principal,
//                                     sender_address: w,
//                                     sent_amount: utxo,
//                                     sent_time: ic_time(),
//                                     network: metadata.network,
//                                     stake_type: StakingType::BTCWallet,
//                                     memo: None,
//                                     fund_management: Default::default(),
//                                     staking_canister: p.staking_pool_canister,
//                                 };
//                                 // ic_cdk::print(format!("in create 8 {}\n", utxo));
//                                 let _: Result<(StakingRecord,), _> = ic_cdk::call(
//                                     p.staking_pool_canister,
//                                     "register_staking_record",
//                                     ((req),),
//                                 )
//                                 .await;
//                                 log!(format!(
//                                     "register stake record,stake pool:{},tx:{}",
//                                     p.staking_pool_canister,
//                                     tx.clone()
//                                 ));
//                             }
//                         }
//                     }
//                     None => {}
//                 };
//             }
//         }
//     }
// }

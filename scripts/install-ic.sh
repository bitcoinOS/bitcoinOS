# Watchout, this script install the canisters on the same application subnet
# See readme.md for more information

dfx canister create os --ic

dfx build os --ic

dfx canister install os --ic

just deploy_os_ic

dfx canister install frontend --ic

# os t7hzl-fqaaa-aaaah-qdc7a-cai
# wallet ygqnb-taaaa-aaaah-qddaa-cai failed
# wallet ybrlv-6yaaa-aaaah-qddaq-cai success

dfx canister deposit-cycles --ic 1_000_000_000_000 tyg77-iiaaa-aaaah-qdc7q-cai
dfx canister deposit-cycles --ic 1_000_000_000_000 7hwzm-laaaa-aaaah-qddqa-cai
-- mint cycle
dfx  canister   deposit-cycles  888888888888  be2us-64aaa-aaaaa-qaabq-cai
./bitcoind     --port=18444
./bitcoin-cli -regtest sendtoaddress mgAsuWQs3adXusER7qejfi27trcQoMbkz8 5
--create wallet
./bitcoin-cli -regtest getwalletinfo
./bitcoin-cli createwallet  XXX
./bitcoin-cli -generate 100
./bitcoin-cli -generate 1

-- from wallet staking to pool
dfx canister call asrmz-lmaaa-aaaaa-qaaeq-cai staking_to_pool '(record { staking_canister = principal "avqkn-guaaa-aaaaa-qaaea-cai"; staking_address = "mk8tN4B63wE8Td8H4UEu9FG2bHhtNuydcm"; amount = 1200000000:nat64 })'

-- staking to 
dfx canister call --ic ybrlv-6yaaa-aaaah-qddaq-cai staking_to_pool '(record { staking_canister = principal "tyg77-iiaaa-aaaah-qdc7q-cai"; staking_address = "mpYWkVyc5wZnj5aPZGBeqQWSeB3vfBwm7Q"; amount = 10000:nat64 })'

-- smartwallet transfer p2wpkh
dfx canister call br5f7-7uaaa-aaaaa-qaaca-cai --candid canisters/smartwallet/smartwallet.did transfer_from_p2wpkh '(record { txs = vec { record { recipient = "mtWKxNnewZHYQCEQvjbbVpCfs2bxm87onq"; amount = 20000000:nat64 }}})'

-- upgrade smartwallet
dfx canister call os upgrade_wallet_wasm '(principal "br5f7-7uaaa-aaaaa-qaaca-cai")'
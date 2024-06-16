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

dfx canister call a3shf-5eaaa-aaaaa-qaafa-cai staking_to_pool_from_p2wsh_multisig22 '(record { staking_canister = principal "asrmz-lmaaa-aaaaa-qaaeq-cai"; staking_address = "bcrt1qyvgtqfdlu97280nxgk58cl8rwz8r3g4fs5zsutcwpd5gt7a2s5rswcymr6"; amount = 30_000_000:nat64 })'

-- staking to pool on ic
dfx canister call --ic ybrlv-6yaaa-aaaah-qddaq-cai staking_to_pool '(record { staking_canister = principal "tyg77-iiaaa-aaaah-qdc7q-cai"; staking_address = "mpYWkVyc5wZnj5aPZGBeqQWSeB3vfBwm7Q"; amount = 10000:nat64 })'

-- transfer btc
dfx canister call br5f7-7uaaa-aaaaa-qaaca-cai transfer_from_p2wsh_multisig22 '( record { txs = vec { record { recipient = "bcrt1quv6ymxsr8hjzktfgqh6aeeg4nxuj7uzqpyc5ug"; amount = 200000:nat64 }}})'

dfx canister call a3shf-5eaaa-aaaaa-qaafa-cai transfer_from_p2wsh_multisig22 '( record { txs = vec { record { recipient = "bcrt1qyvgtqfdlu97280nxgk58cl8rwz8r3g4fs5zsutcwpd5gt7a2s5rswcymr6"; amount = 200000:nat64 }}})'


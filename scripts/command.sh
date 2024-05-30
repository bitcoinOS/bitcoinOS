-- mint cycle
dfx  canister   deposit-cycles  888888888888  be2us-64aaa-aaaaa-qaabq-cai
./bitcoind     --port=18444
./bitcoin-cli -regtest sendtoaddress mkjCdp5iELe2Mk7a8Z9TcwmcuwFKtUNTqs 5
--create wallet
./bitcoin-cli -regtest getwalletinfo
 ./bitcoin-cli createwallet  XXX
 ./bitcoin-cli -generate 100
 ./bitcoin-cli -generate 1

-- from wallet staking to pool
dfx canister call b77ix-eeaaa-aaaaa-qaada-cai staking_to_pool '(record { staking_canister = principal "by6od-j4aaa-aaaaa-qaadq-cai"; staking_address = "mhg7uoQBgvKWy3PzjUgQvBwVFzhnTaAu32"; amount = 1200000000:nat64 })'

-- staking to 
dfx canister call --ic ybrlv-6yaaa-aaaah-qddaq-cai staking_to_pool '(record { staking_canister = principal "tyg77-iiaaa-aaaah-qdc7q-cai"; staking_address = "mpYWkVyc5wZnj5aPZGBeqQWSeB3vfBwm7Q"; amount = 10000:nat64 })'
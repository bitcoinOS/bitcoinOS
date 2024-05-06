-- mint cycle
dfx  canister   deposit-cycles  888888888888  be2us-64aaa-aaaaa-qaabq-cai
./bitcoind     --port=18444
./bitcoin-cli -regtest sendtoaddress mkjCdp5iELe2Mk7a8Z9TcwmcuwFKtUNTqs 5
--create wallet
./bitcoin-cli -regtest getwalletinfo
 bitcoin-cli createwallet  XXX
 bitcoin-cli -generate 100
 bitcoin-cli -generate 1
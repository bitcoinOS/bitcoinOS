-- mint cycle
dfx  canister   deposit-cycles  888888888888  be2us-64aaa-aaaaa-qaabq-cai
./bitcoind     --port=18444
./bitcoin-cli -regtest sendtoaddress bcrt1qu98kqg4f8r05pl2w993f5tenfjz8jrc3d7sd4w 5
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

dfx canister call --ic ybrlv-6yaaa-aaaah-qddaq-cai staking_to_pool '(record { staking_canister = principal "tyg77-iiaaa-aaaah-qdc7q-cai"; staking_address = "tb1q82l3kvlvu5ph4hkv53kuwawd2sguk97cymha2gdclcngn6vflyhqfq4rvd"; amount = 100000:nat64 })'

-- transfer btc
dfx canister call br5f7-7uaaa-aaaaa-qaaca-cai transfer_from_p2wsh_multisig22 '( record { txs = vec { record { recipient = "bcrt1quv6ymxsr8hjzktfgqh6aeeg4nxuj7uzqpyc5ug"; amount = 200000:nat64 }}})'

dfx canister call a3shf-5eaaa-aaaaa-qaafa-cai transfer_from_p2wsh_multisig22 '( record { txs = vec { record { recipient = "bcrt1qyvgtqfdlu97280nxgk58cl8rwz8r3g4fs5zsutcwpd5gt7a2s5rswcymr6"; amount = 200000:nat64 }}})'

dfx canister call --ic ybrlv-6yaaa-aaaah-qddaq-cai transfer_from_p2wsh_multisig22 '( record { txs = vec { record { recipient = "tb1qj7nztwqcr4alddxla26hrz7ld2w34px4vph8g4rshmn739w2mddqmmuhtz"; amount = 2000:nat64 }}})'

dfx canister call ajuq4-ruaaa-aaaaa-qaaga-cai transfer_from_p2wsh_multisig22 '( record { txs = vec { record { recipient = "bcrt1qy3anc7x93k8z72lfvjep6fcykh7wnzvqngwzj7f0akp0cl2mqsnsa6tfje"; amount = 200_000:nat64 }}})'

-- register wallet 
dfx canister call --ic os register_wallet '(record { bitcoin_address = "mzLs49NazRUhj2PeNpFbX1gQu9codA4CX3"; wallet_canister = principal "ybrlv-6yaaa-aaaah-qddaq-cai"; owner = principal "v4r3s-nn353-xms6p-37w4r-okcn5-xxp6v-cnod7-4xqfl-sw5to-gcgue-bqe"; name = "Phili smart wallet"})'
dfx canister call --ic os register_wallet '(record { bitcoin_address = "tb1q4230hrh80tmq9dlpt472372f33pmwu5qvp0gc7sylfs5ndc9kkpqf6zc2u"; wallet_canister = principal "ybrlv-6yaaa-aaaah-qddaq-cai"; owner = principal "v4r3s-nn353-xms6p-37w4r-okcn5-xxp6v-cnod7-4xqfl-sw5to-gcgue-bqe"; name = "Phili smart wallet"})'

dfx canister call --ic os register_wallet '(record { bitcoin_address = "tb1qj7nztwqcr4alddxla26hrz7ld2w34px4vph8g4rshmn739w2mddqmmuhtz"; wallet_canister = principal "37am3-ziaaa-aaaah-qddjq-cai"; owner = principal "4qmdg-fjltn-m2ys2-xb26v-7e43a-gj6ul-culqc-jv5w7-6waaq-kst7z-vae"; name = "phili002"})'
dfx canister call --ic os register_wallet '(record { bitcoin_address = "tb1qffsp4z8lf5skttjmugwmgn8tzx8qsx0xp7d0plt64732aeydlrxq0hmxrg"; wallet_canister = principal "2vpob-3yaaa-aaaah-qddoq-cai"; owner = principal "sflvn-nyqsu-lshoj-26ppn-rvghf-7yhk6-4qhgh-rnkna-zboit-y3qvk-pqe"; name = "Wallet 99"})'

dfx canister call --ic os register_wallet '(record { bitcoin_address = "tb1qucktt4ytahxr2tzyjnzs9d9h89slhkgtne2an9jthw5vzwm0r0ys6wqczk"; wallet_canister = principal "6r5bh-6qaaa-aaaah-qddva-cai"; owner = principal "53boj-zqsmx-fcyv4-jndyw-s6j4q-lbuom-towmw-prgzx-pcgya-nzbmj-sqe"; name = "cliff"})'
dfx canister call --ic os register_wallet '(record { bitcoin_address = "tb1qwrel73ht5qnww94gewx6qewcnwwr6sut4f4drawr2cf0lnzda4ds34e3e6"; wallet_canister = principal "3rcbt-cyaaa-aaaah-qddiq-cai"; owner = principal "7pqhz-6a46q-ahxlq-debuo-s2www-cofh3-ivext-3kjhn-my6od-3tftf-3qe"; name = "liam"})'
dfx canister call --ic os register_wallet '(record { bitcoin_address = "tb1q47m5kynrctrps7fjp84n4g8wks0przl2w7msaukveame2jq5vz2qun3c2a"; wallet_canister = principal "3kh5w-yaaaa-aaaah-qddka-cai"; owner = principal "7pqhz-6a46q-ahxlq-debuo-s2www-cofh3-ivext-3kjhn-my6od-3tftf-3qe"; name = "liam2"})'
dfx canister call --ic os register_wallet '(record { bitcoin_address = "tb1q0ygg9crpwx55jra5njtsg9jv9zd2m6cz069q896f6lc9kf9hltysgsw2se"; wallet_canister = principal "73sd5-4aaaa-aaaah-qddsa-cai"; owner = principal "rg27n-3ah2q-2hlrd-hxy2y-rxxrh-wgfle-muhui-ik3dq-wpmb7-re7hb-rae"; name = "wellet-1"})'
dfx canister call --ic os register_wallet '(record { bitcoin_address = "tb1qef3nyu6hy6nqply5unkdnnc3e80cesf2wpsztnjyfny0veuggjqsldt80u"; wallet_canister = principal "74tfj-ryaaa-aaaah-qddsq-cai"; owner = principal "rg27n-3ah2q-2hlrd-hxy2y-rxxrh-wgfle-muhui-ik3dq-wpmb7-re7hb-rae"; name = "wellet22"})'
dfx canister call --ic os register_wallet '(record { bitcoin_address = "tb1qef0vas7khlehhca4flskmvhqt0gwuhdurl29g4t7lku0l554legscx6q6d"; wallet_canister = principal "7vqov-hqaaa-aaaah-qddta-cai"; owner = principal "rg27n-3ah2q-2hlrd-hxy2y-rxxrh-wgfle-muhui-ik3dq-wpmb7-re7hb-rae"; name = "wellet-3"})'
dfx canister call --ic os register_wallet '(record { bitcoin_address = "tb1qjre269ydtk6ph69jvhksfv4cwz66r82gg3lhmam83u7swu08at7sv3jfy7"; wallet_canister = principal "46gyw-biaaa-aaaah-qddzq-cai"; owner = principal "rg27n-3ah2q-2hlrd-hxy2y-rxxrh-wgfle-muhui-ik3dq-wpmb7-re7hb-rae"; name = "002"})'

dfx canister call --ic os register_wallet '(record { bitcoin_address = "tb1qxf7e8c26mzpgadvhhdtx0en26j4dhn4tk2xvpd9lkc7dqunewpgqq33kr0"; wallet_canister = principal "yptg5-fiaaa-aaaah-qddbq-cai"; owner = principal "3epuc-tlc3v-vlifc-25qc5-l7ndn-t7bpm-2tm2q-khjly-4xwd6-ibxu3-dae"; name = "phili001"})'
dfx canister call --ic os register_wallet '(record { bitcoin_address = "tb1q62s9x2y7ep90hlw6und0a20fzrnwucjuqd0ghhzp2mtvvxz539zqryvrk9"; wallet_canister = principal "6w4ht-tiaaa-aaaah-qddvq-cai"; owner = principal "ir5ms-o4oyk-2isrw-fve4f-ka65u-sqzbc-k6zy5-2cgbs-cxn3w-p4jdn-nae"; name = "钱包1"})'

dfx canister call --ic os register_wallet '(record { bitcoin_address = "tb1qkppc4g6x3na7vd5pt8dajj75sruu9ff6aj68ytu6xpmsgkjk2aqs7jrg2d"; wallet_canister = principal "6nz3w-jqaaa-aaaah-qddxa-cai"; owner = principal "ir5ms-o4oyk-2isrw-fve4f-ka65u-sqzbc-k6zy5-2cgbs-cxn3w-p4jdn-nae"; name = "2.5"})'
dfx canister call --ic os register_wallet '(record { bitcoin_address = "tb1qn28hd08xqq9qju3exmllj380ftwesqrfa59ydcs62h5surdzh52s0x0jcw"; wallet_canister = principal "4qev6-2yaaa-aaaah-qddyq-cai"; owner = principal "ir5ms-o4oyk-2isrw-fve4f-ka65u-sqzbc-k6zy5-2cgbs-cxn3w-p4jdn-nae"; name = "002"})'
dfx canister call --ic os register_wallet '(record { bitcoin_address = "tb1qzk60e5u29wynht5pakq5lctpqt263c8a86df75rqnfzs79ewurkq9xsmd7"; wallet_canister = principal "4zh6c-mqaaa-aaaah-qddza-cai"; owner = principal "ir5ms-o4oyk-2isrw-fve4f-ka65u-sqzbc-k6zy5-2cgbs-cxn3w-p4jdn-nae"; name = "002"})'
dfx canister call --ic os register_wallet '(record { bitcoin_address = "tb1qfex0pc8s9hyepdl7qs73v5cyva80cju5rud628kxl2n87m876f4szqdlan"; wallet_canister = principal "4lbj3-aaaaa-aaaah-qdd2a-cai"; owner = principal "jwi3a-25ayu-ryrib-ouqgh-7pbai-z6jcb-zg5k4-ppg2o-6bsf7-3onfj-5qe"; name = "liam23"})'

dfx canister call --ic os register_wallet '(record { bitcoin_address = "tb1qrssfxcwnpxeyw8he87zmt99qa0nth5hs8nud859lmqqaj6u34wrsfxmna2"; wallet_canister = principal "y2uxq-eaaaa-aaaah-qddca-cai"; owner = principal "cddre-d5lrb-hv2ak-tfdir-ba2su-kuu5b-32zgg-jru6s-bq7io-jataj-gqe"; name = "1"})'
dfx canister call --ic os register_wallet '(record { bitcoin_address = "tb1qdc2lnhnw8w4e435zs50ydqw94dryy4ch5dvcmyhs4gtz6dn5u0rs36l6lc"; wallet_canister = principal "2hjzy-xiaaa-aaaah-qddnq-cai"; owner = principal "yoron-jvpo4-oafgo-lgtuw-emva3-muu6h-g6k7q-vkpcx-pwqxh-k76lb-oqe"; name = "woo"})'
dfx canister call --ic os register_wallet '(record { bitcoin_address = "tb1q7y2x0fuwhp6zpv5t0ejhayp6z43n4svlydm6kwll646tghpguahqdcpez4"; wallet_canister = principal "2soiv-waaaa-aaaah-qddoa-cai"; owner = principal "yoron-jvpo4-oafgo-lgtuw-emva3-muu6h-g6k7q-vkpcx-pwqxh-k76lb-oqe"; name = "yy"})'
dfx canister call --ic os register_wallet '(record { bitcoin_address = "tb1q3wu55eqq7pr7pr9r0sfs20ef2uuwwxpvghznr2ntvhvag83s7fpq988ess"; wallet_canister = principal "6d3w6-saaaa-aaaah-qddwa-cai"; owner = principal "yoron-jvpo4-oafgo-lgtuw-emva3-muu6h-g6k7q-vkpcx-pwqxh-k76lb-oqe"; name = "woo"})'
dfx canister call --ic os register_wallet '(record { bitcoin_address = "tb1q0rc3jk9z3gnf49dtz3m4zezcgua3yqn5utf7akqzshpxt5s549mq3fz2xc"; wallet_canister = principal "6e2qk-7yaaa-aaaah-qddwq-cai"; owner = principal "yoron-jvpo4-oafgo-lgtuw-emva3-muu6h-g6k7q-vkpcx-pwqxh-k76lb-oqe"; name = "2222333"})'

dfx canister call --ic os register_wallet '(record { bitcoin_address = "tb1q32fz4nfaqpzmx5pd99efskp6yzd926mw2zzmn0lejytjah3f7xcs8k06mx"; wallet_canister = principal "3efq6-dqaaa-aaaah-qddla-cai"; owner = principal "f3cyt-4gbcv-bcnym-xu6nk-mv4cv-klmmc-wtzxh-4cv5l-idobf-xn3k7-5ae"; name = "1"})'
dfx canister call --ic os register_wallet '(record { bitcoin_address = "tb1qz86ejy9lfhhu8e2vcvfla5gy2kvfvzw5e9lq9dyh87yeks3f0d7st2dmvp"; wallet_canister = principal "4mapp-nyaaa-aaaah-qdd2q-cai"; owner = principal "f3cyt-4gbcv-bcnym-xu6nk-mv4cv-klmmc-wtzxh-4cv5l-idobf-xn3k7-5ae"; name = "1"})'

dfx canister call --ic os register_wallet '(record { bitcoin_address = "tb1q9h2akus9uuf8lyja5m7yykt6rj49yrn6663cn3hfkwh8vc5auueq4ezzft"; wallet_canister = principal "7srib-kiaaa-aaaah-qddtq-cai"; owner = principal "k3ro6-3ovqi-vsov4-uij22-2k446-ftwdu-xhcmc-3qkpg-vfnn5-wsywk-7ae"; name = "222221"})'
dfx canister call --ic os register_wallet '(record { bitcoin_address = "tb1qmy0yylpyc3cddy3nsg460tweygc2xzf4vckdh587ks4ccwlcx85sln5gk8"; wallet_canister = principal "6ky5c-eiaaa-aaaah-qddxq-cai"; owner = principal "h25rk-mg3g7-dj5wj-3kwji-xyxp3-ujlyi-3ie3v-5m5di-rshhd-whyla-iae"; name = "testing"})'
dfx canister call --ic os register_wallet '(record { bitcoin_address = "tb1qkp9p7yyn5pks5920es7k6d3r4ctqzlnk4f6gppl5le5jsxndujsq5q4afx"; wallet_canister = principal "4xftk-xaaaa-aaaah-qddya-cai"; owner = principal "h25rk-mg3g7-dj5wj-3kwji-xyxp3-ujlyi-3ie3v-5m5di-rshhd-whyla-iae"; name = "testing"})'

-- register staking pool
dfx canister call --ic os register_staking_pool '(record { duration_in_day = 30:nat64; network = variant { testnet }; name = "Babylon Staking"; description = "A financial product based on Babylon staking, with an annualized rate of return of 10%"; annual_interest_rate = 10:nat16; staking_pool_canister = principal "tyg77-iiaaa-aaaah-qdc7q-cai"; bitcoin_address = "tb1q82l3kvlvu5ph4hkv53kuwawd2sguk97cymha2gdclcngn6vflyhqfq4rvd"; status = "inactive"; start_time = 1718938706200000000:nat64; end_time = 1750474572000000000:nat64; fund_management = "transfer" })'

-- login or create wallet
 dfx canister call os login_or_create '(record { user_id = principal "v4r3s-nn353-xms6p-37w4r-okcn5-xxp6v-cnod7-4xqfl-sw5to-gcgue-bqe"; user_type = variant { II }})'

-- get address
dfx canister call --candid canisters/dbank/dbank.did asrmz-lmaaa-aaaaa-qaaeq-cai p2wpkh_address

-- update staking pool info
dfx canister call --ic os update_staking_pool_info '(record { staking_pool_canister = principal "tyg77-iiaaa-aaaah-qdc7q-cai"; name = "Babylon Staking"; description = "A financial product based on Babylon staking, with an annualized rate of return of 10%"; annual_interest_rate = 10:nat16; bitcoin_address = "tb1q82l3kvlvu5ph4hkv53kuwawd2sguk97cymha2gdclcngn6vflyhqfq4rvd"; status = "active"; start_time = 1_721_664_000_000_000_000:nat64; end_time = 1767196799000000000:nat64; fund_management = variant { Transfer }; duration_in_day = 400:nat64 })'
dfx canister call --ic os update_staking_pool_info '(record { staking_pool_canister = principal "4ttm4-caaaa-aaaah-qdefq-cai"; name = "Babylon Staking for everyone"; description = "A financial product based on Babylon staking, with an annualized rate of return of 10%"; annual_interest_rate = 10:nat16; bitcoin_address = "tb1qpc9prgt7d20jqr0e0gtd39lng824mmg0zrc64ydapz4ctw5ljnss3rn2y0"; status = "activing"; start_time = 1_721_664_000_000_000_000:nat64; end_time = 1767196799000000000:nat64; fund_management = variant { Transfer }; duration_in_day = 400:nat64 })'

-- update staking pool status
dfx canister call --ic os update_staking_pool_status '(record { staking_pool_canister = principal "tyg77-iiaaa-aaaah-qdc7q-cai"; status = "active" })'


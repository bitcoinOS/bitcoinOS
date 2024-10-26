
create_all_canisters:
    dfx canister create --all

deploy_steward:
    dfx deploy steward --argument '("regtest")'

deploy_steward_ic:
    dfx deploy steward --ic --argument '("testnet")'

deploy_os:
    STEWARD_CANISTER=$(dfx canister id steward) && dfx deploy os    --argument "(record { network = variant { regtest }; steward_canister =  principal \"$STEWARD_CANISTER\"; })" 

deploy_os_ic:
     STEWARD_CANISTER=$(dfx canister id steward --ic) && dfx deploy os --ic --argument "(record { network = variant { testnet }; steward_canister =  principal \"$STEWARD_CANISTER\"; })"

add_point:
    #!/usr/bin/env bash
    # set -euxo pipefail
    POINT_CANISTER=$(dfx canister id point)
    dfx canister call os register_canister_module  "(\"point\", principal \"$POINT_CANISTER\")"

add_point_ic:
    #!/usr/bin/env bash
    # set -euxo pipefail
    POINT_CANISTER=$(dfx canister id --ic point)
    dfx canister call --ic os register_canister_module  "(\"point\", principal \"$POINT_CANISTER\")"

generate_did:
    ./scripts/did.sh
    
build_staking:
    cargo build -p stakingpool --release --target wasm32-unknown-unknown

build_wallet:   
    cargo build -p smartwallet --release --target wasm32-unknown-unknown 

build_point:   
    cargo build -p point --release --target wasm32-unknown-unknown 


build_bank:   
    cargo build -p dbank --release --target wasm32-unknown-unknown 

translate_wasm:
    wasi2ic ./target/wasm32-unknown-unknown/release/smartwallet.wasm smartwallet.wasm

install_wallet:
    #!/usr/bin/env bash
    # set -euxo pipefail
    STEWARD_CANISTER=$(dfx canister id steward)
    dfx canister install --wasm smartwallet.wasm smartwallet --mode reinstall --argument "(record { name = \"smartwallet\";  network = variant { regtest }; steward_canister = principal\"$STEWARD_CANISTER\" })"

deploy_wallet:
    #!/usr/bin/env bash
    # set -euxo pipefail
    STEWARD_CANISTER=$(dfx canister id steward)
    dfx deploy smartwallet --argument "(record { name = \"smartwallet\"; network = variant { regtest }; steward_canister = principal \"$STEWARD_CANISTER\" })"

deploy_dbank:
    #!/usr/bin/env bash
    # set -euxo pipefail
    STEWARD_CANISTER=$(dfx canister id steward)
    dfx deploy dbank --argument "(record { name = \"dbank0\"; network = variant { regtest }; steward_canister = principal \"$STEWARD_CANISTER\" })"

deploy_staking:
    #!/usr/bin/env bash
    # set -euxo pipefail
    OS_CANISTER=$(dfx canister id os)
    dfx deploy stakingpool --argument "(record { name = \"stakingpool\"; network = variant { regtest }; os_canister = principal \"$OS_CANISTER\"; description = \"a staking pool with 10 annual interest rate for a year\"; annual_interest_rate = 10; duration_in_millisecond = 86400; })"


deploy_ii:
    dfx deploy internet_identity

deploy_point:
    #!/usr/bin/env bash
    # set -euxo pipefail
    OS_CANISTER=$(dfx canister id os)
    echo "${OS_CANISTER}"
    dfx deploy point --argument "(record { network = variant { regtest }; os_canister = principal \"${OS_CANISTER}\"; steward_canister = principal \"$OS_CANISTER\"; task_period=60 })"

deploy_point_ic:
    #!/usr/bin/env bash
    # set -euxo pipefail
    OS_CANISTER=$(dfx canister id os --ic)
    STEWARD_CANISTER=$(dfx canister id steward --ic)
    echo "${OS_CANISTER}"
    SIWB_CANISTER=$(dfx canister id ic_siwb_provider --ic)

    dfx deploy point --ic --argument "(record { network = variant { testnet }; os_canister = principal \"${OS_CANISTER}\"; steward_canister = principal  \"$STEWARD_CANISTER\"; siwb_canister = opt principal \"${SIWB_CANISTER}\";  task_period=60 })"


deploy_stake:
    #!/usr/bin/env bash
    # set -euxo pipefail
    OS_CANISTER=$(dfx canister id os)
    USER_CANISTER=$(dfx canister id os)
    echo "${OS_CANISTER}"
    dfx deploy stake --argument "(record {  os_canister = principal \"${OS_CANISTER}\"; user_canister = principal \"${USER_CANISTER}\"; })"

deploy_stake_ic:
    #!/usr/bin/env bash
    # set -euxo pipefail
    OS_CANISTER=$(dfx canister id os --ic)
    USER_CANISTER=$(dfx canister id os --ic)
    dfx deploy stake --ic --argument "(record { network = variant { testnet }; os_canister = principal \"${OS_CANISTER}\"; user_canister = principal  \"$USER_CANISTER\";  })"

add_stake:
    #!/usr/bin/env bash
    # set -euxo pipefail
    STAKE_CANISTER=$(dfx canister id stake)
    dfx canister call os register_canister_module  "(\"stake\", principal \"$STAKE_CANISTER\")"

add_stake_ic:
    #!/usr/bin/env bash
    # set -euxo pipefail
    STAKE_CANISTER=$(dfx canister id --ic stake)
    dfx canister call --ic os register_canister_module  "(\"stake\", principal \"$STAKE_CANISTER\")"


deploy_frontend:
    dfx deploy frontend 

deploy_frontend_ic:
    dfx deploy frontend --ic

deploy_provider:
    dfx deploy ic_siwb_provider --argument  '(record{domain= "localhost:5173";uri="http://localhost:5173";salt="2344";network=opt "testnet";scheme=opt "http"})'

deploy_provider_ic:
    dfx deploy ic_siwb_provider --ic --argument  '(record{domain= "testnet.bifipal.com";uri="https://testnet.bifipal.com";salt="2344";network=opt "testnet";scheme=opt "https";sign_in_expires_in = opt 1500000000000; session_expires_in = opt 604800000000000})'

add_provider:
    #!/usr/bin/env bash
    # set -euxo pipefail
    SIWB_CANISTER=$(dfx canister id ic_siwb_provider)
    dfx canister call os register_canister_module  "(\"siwb\", principal \"$SIWB_CANISTER\")"

add_provider_ic:
    #!/usr/bin/env bash
    # set -euxo pipefail
    SIWB_CANISTER=$(dfx canister id --ic ic_siwb_provider)
    dfx canister call --ic os register_canister_module  "(\"siwb\", principal \"$SIWB_CANISTER\")"

create_staking_pool:
    dfx canister call os create_staking_pool_canister '(record { duration_in_day = 1:nat64; name = "Babylon Staking"; description = "A financial product based on Babylon staking, with an annualized rate of return of 10%"; annual_interest_rate = 10:nat16 ; status = "Activing"; start_time = 1718938706200000000:nat64; end_time = 1750474572000000000:nat64; fund_management = "transfer"})'

create_staking_pool_ic:
    dfx canister call --ic os create_staking_pool_canister '(record { duration_in_day = 1:nat64; name = "Babylon Staking"; description = "A financial product based on Babylon staking, with an annualized rate of return of 10%"; annual_interest_rate = 10:nat16; status = "Activing"; start_time = 1718938706200000000:nat64; end_time = 1750474572000000000:nat64; fund_management = "transfer" })'

create_staking_pool_ic_1_y:
    dfx canister call --ic os create_staking_pool_canister '(record { duration_in_day = 365:nat64; name = "Babylon Staking for 1 year"; description = "A financial product based on Babylon staking, with an annualized rate of return of 10%"; annual_interest_rate = 10:nat16; status = "Activing"; start_time = 1718938706200000000:nat64; end_time = 1750474572000000000:nat64; fund_management = "transfer" })'

register_staking_pool_ic:
    dfx canister call --ic os register_staking_pool '(record { duration_in_day = 30:nat64; network = variant { testnet }; name = "Babylon Staking"; description = "A financial product based on Babylon staking, with an annualized rate of return of 10%"; annual_interest_rate = 10:nat16; staking_pool_canister = principal "git"; bitcoin_address = "tb1q82l3kvlvu5ph4hkv53kuwawd2sguk97cymha2gdclcngn6vflyhqfq4rvd"; status = "Activing"; start_time = 1718938706200000000:nat64; end_time = 1750474572000000000:nat64; fund_management = "transfer" })'

create_wallet:
    dfx canister call os create_wallet_canister '("smartwallet")'

create_dbank_p2pkh_wallet:
    dfx canister call dbank create_p2pkh_wallet '(record { seq_in_os = 0; name = "james1"; wallet_owner = principal "l42kn-itg55-kntvr-gwvyo-jx7ak-m3brp-p3tef-c4ire-2i772-dysqm-dqe" })'

create_dbank_p2pkh_wallet_default:
    dfx canister call dbank create_p2pkh_wallet '(record { seq_in_os = 0; name = "james1"; wallet_owner = principal "v4r3s-nn353-xms6p-37w4r-okcn5-xxp6v-cnod7-4xqfl-sw5to-gcgue-bqe" })'

create_dbank_p2wpkh_wallet:
    dfx canister call dbank create_p2wpkh_wallet '(record { seq_in_os = 0; name = "james1"; wallet_owner = principal "l42kn-itg55-kntvr-gwvyo-jx7ak-m3brp-p3tef-c4ire-2i772-dysqm-dqe" })'

create_dbank_p2wpkh_wallet_default:
    dfx canister call dbank create_p2wpkh_wallet '(record { seq_in_os = 0; name = "james1"; wallet_owner = principal "v4r3s-nn353-xms6p-37w4r-okcn5-xxp6v-cnod7-4xqfl-sw5to-gcgue-bqe" })'

mint_cycles:
    #!/usr/bin/env bash
    # set -euxo pipefail
    wallet=$(dfx identity get-wallet)
    dfx ledger fabricate-cycles --t 2000 --canister $wallet
    dfx wallet balance

deposit_os:
    dfx canister deposit-cycles 20_000_000_000_000 os

deposit_point_1t_ic:
    dfx canister deposit-cycles 1_000_000_000_000 point --ic
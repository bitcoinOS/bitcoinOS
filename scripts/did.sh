#!/usr/bin/env bash

source ./scripts/did.utils.sh

CANISTERS=os,smartwallet,stakingpool

for canister in $(echo $CANISTERS | sed "s/,/ /g")
do
    generate_did "$canister" "canisters/$canister"
done


#!/usr/bin/env bash

source ./scripts/did.utils.sh

CANISTERS=smartwallet,stakingpool,dbank,os,point,steward,stake

for canister in $(echo $CANISTERS | sed "s/,/ /g")
do
    generate_did "$canister" "canisters/$canister"
done


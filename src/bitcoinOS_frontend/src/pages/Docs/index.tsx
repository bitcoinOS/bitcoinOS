import React, { useEffect, useState } from 'react'
import { Box, Flex, Text, Table, Thead, Tbody, Tr, Th, Td, useToast, Button } from '@chakra-ui/react';

import useGetMarathon from '../../utils/marathonActor';
import useOsActor from '../../utils/osActor';
import { useOsBackend } from '../../ic/OsActors';
import { usePointBackend } from '../../ic/PointActors';
import { useInternetIdentity } from 'ic-use-internet-identity';

import { useSiwbIdentity } from 'ic-use-siwb-identity';
import useWalletConnector from "../../hooks/useWalletConnector";
import { useConnectStore } from '../../store/useConnectStore';
import { userStore, boxStore } from "../../store/useMarathonStore";
import useGetStakePool from '../../utils/poolActor';
import { useStakeListStore } from '../../store';
export default function Docs() {
    const { actor: osBackend } = useOsBackend();
    const { actor: pointBackend } = usePointBackend();

    const { identity, isLoggingIn, login, clear, isLoginSuccess } = useInternetIdentity();
    const { identity: siwb_identity, identityAddress, clear: siwb_clear } = useSiwbIdentity();
    const { get_reward, get_info, get_invite_info, get_user_stakebtc, get_boxes, get_boxes_record, open_box, update_login } = useGetMarathon();
    const { os_login } = useOsActor();
    const [currentAccount, setCurrentAccount] = useConnectStore((state) => [state.currentAccount, state.setCurrentAccount])
    const { boxNum, boxRecord, remainingTimes, boxReward, setBoxNum, setBoxReward, setBoxRecord, setRemainingTimes } = boxStore();
    const { currentPool, setCurrentPool, poolDetail, setPoolDetail } = useStakeListStore();
    const remainingTime = boxStore(state => state.remainingTimes[0] || 0);
    const { get_pointrank, get_pointrank_status } = useGetStakePool()
    const {
        connect,
        connectingWallet,
        isInstalled
    } = useWalletConnector()
    useEffect(() => {
        if (osBackend) {
            console.log('-')
        }
    }, [osBackend])

    useEffect(() => {
        if (pointBackend) {
            console.log('-')
        }
    }, [pointBackend])

    const test = async () => {
        console.log('test1')
        //console.log(boxReward)
        //console.log(currentAccount)
        //console.log(remainingTimes)
        //console.log(currentAccount)
        // console.log(identityAddress)
        //console.log(siwb_identity)
        //get_pointrank(pointBackend)
        //console.log(globalThis.wizz.sendBitcoin())
        //await os_login(osBackend)
        //await get_pointrank_status(pointBackend)
        await get_boxes(pointBackend)
        //await get_reward(pointBackend)
        //await get_info(osBackend)
        //await get_boxes_record(pointBackend)
        //await open_box(pointBackend)
        //await update_login(osBackend)
        //await get_invite_info(pointBackend)
        //await get_box_record(pointBackend)
        //await get_user_stakebtc(pointBackend)
    }

    return (
        <>
            <Flex width='100%' justifyContent='center'>
                <Text>Coming Soon</Text>
                {/*
                <Button onClick={test}>test</Button>
                */}
            </Flex>
        </>
    );
}
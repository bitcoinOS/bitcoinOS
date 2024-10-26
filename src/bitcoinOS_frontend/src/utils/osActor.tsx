import React from 'react'

import { WalletStore } from "../store/useWalletStore";
import { createActor as createActor_os, canisterId as canisterId_os } from "../../../declarations/os/index";

import { UserRequest } from "../ic/OsActors";

import { CurrentStatus } from '../store';
import { useConnectStore } from '../store/useConnectStore';
import { usePoolrecordStore } from "../store/useStakePool"
import { userStore, boxStore } from '../store/useMarathonStore';
import { IcpWalletStore } from "../store/useWalletStore";


import { useInternetIdentity } from 'ic-use-internet-identity';
import { useSiwbIdentity } from 'ic-use-siwb-identity';
import useGetWalletPool from './walletActor';

const useOsActor = () => {

    const { identity, isLoggingIn, login, clear, isLoginSuccess } = useInternetIdentity();
    const { identity: siwb_identity, identityAddress, clear: siwb_clear } = useSiwbIdentity();

    const [currentAccount, setCurrentAccount] = useConnectStore((state) => [state.currentAccount, state.setCurrentAccount])

    const { setWallet, setWalletList, setWalletSelect, setCurrentWallet, setBalance, setTotalBalance } = WalletStore()
    const { setIcpWallet, setBindIcpWallet } = IcpWalletStore()

    const {
        setUserInfo,
        setUserReward,
        setUserNftReward,
        setInviteInfo,
    } = userStore();
    const {
        setBoxNum,
        setBoxRecord,
        setRemainingTimes,
        setBoxReward
    } = boxStore();

    const { setStakeRecords } = usePoolrecordStore()

    const { get_wallets } = useGetWalletPool()

    const identity_select = React.useMemo(() => {
        if (!currentAccount) {
            return
        }
        switch (currentAccount.type) {
            case 'UNISAT':
                return siwb_identity;
            case 'WIZZ':
                return siwb_identity;
            case 'INTERNET_IDENTITY':
                return identity;
        }
    }, [currentAccount, siwb_identity, identity]);

    // get point ranks info
    const os_login = async (osBackend) => {
        //const osBackend_c = createActor_os(canisterId_os)
        const osBackend_c = await createActor_os(canisterId_os, {
            agentOptions: {
                identity: identity_select
            }
        });

        let userType;
        let walletAddress: [] | [string] = [];

        if (currentAccount) {
            switch (currentAccount.type) {
                case 'UNISAT':
                    userType = { Wallet: null };
                    if (!currentAccount.address) {
                        console.log('UNISAT account address is empty. Stopping function execution.');
                        return;
                    }
                    walletAddress = [currentAccount.address];
                    break;
                case 'WIZZ':
                    userType = { Wallet: null };
                    if (!currentAccount.address) {
                        console.log('WIZZ account address is empty. Stopping function execution.');
                        return;
                    }
                    walletAddress = [currentAccount.address];
                    break;
                case 'INTERNET_IDENTITY':
                    userType = { II: null };
                    break;
                default:
                    userType = { II: null };
            }
        } else {
            userType = { II: null };
        }

        const userRequest: UserRequest = {
            user_desc: [],
            user_type: userType,
            user_img: [],
            sign_message: [],
            invited_code: [],
            name: [],
            wallet_address: walletAddress,
            user_id: identity_select.getPrincipal(),
        };
        //debugger
        try {
            const value = await osBackend_c.login_or_create(userRequest);
            if ('Ok' in value) {
                if (currentAccount && currentAccount.type === 'INTERNET_IDENTITY') {
                    get_wallets(osBackend_c)
                }
                const result = 'ok'
                return result
            }

        } catch (error) {
            /*
            toast({
                title: 'Info',
                description: "get stake error",
                status: 'error',
                position: 'bottom-right',
                duration: 9000,
                isClosable: true,
            });
            */
            console.error("Error fetching staking pool:", error);
        } finally {
            console.log('end');
        }
    }

    const os_logout = async () => {
        setWallet("")
        setWalletList([])
        setWalletSelect(null)
        setCurrentWallet("")
        setBalance(0)
        setTotalBalance(0)
        setStakeRecords([])

        //marathon store
        setUserInfo(null)
        setUserReward(null)
        setUserNftReward(null)
        setInviteInfo(null)
        setBoxNum(0)
        setBoxRecord([])
        setRemainingTimes([])
        setBoxReward(null)

        //icp store
        setIcpWallet("")
        setBindIcpWallet("")
    }

    return {
        os_login,
        os_logout
    };
};

export default useOsActor;
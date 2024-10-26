import React from 'react'
import { useToast } from '@chakra-ui/react';
import { Box, Text } from '@chakra-ui/react'
import { createActor as createActor_wallet, canisterId } from "../../../declarations/smartwallet/index";
import { icphost, backend_ip, nftCanisterId, checkIdentityExpiration, computeExtTokenIdentifier } from './utils';
import { StakeCanisterId } from '../ic/StakeActors';

import { useInternetIdentity } from "ic-use-internet-identity";
import { useSiwbIdentity } from 'ic-use-siwb-identity';

import { WalletInfo, UserProfileRequest } from '../ic/OsActors';
import { TransferRequest } from '../ic/WalletActors'
import { BindRequest } from '../ic/OsActors';
import { StakeRequest } from '../ic/StakeActors';
import { WalletStore, IcpWalletStore, UserInfo, StakeNftInfo } from '../store/useWalletStore';
import { CurrentStatus } from '../store';
import { useConnectStore } from '../store/useConnectStore';
import useGetStakePool from './poolActor';


import { createActor as createActor_os, canisterId as canisterId_os } from "../../../declarations/os/index";

import { createActor as createActor_stake, canisterId as canisterId_stake } from "../../../declarations/stake/index";
import axios from 'axios';

import { idlFactory } from './nft.did.js';
//import { idlFactory } from '../components/ConnectModal/nft.did.js';

import { Principal } from '@dfinity/principal';
import { Observable } from '@dfinity/agent/lib/cjs/observable';

const useGetWalletPool = () => {
    const toast = useToast()

    const btcunity = 100000000;

    const { identity, clear } = useInternetIdentity();
    const { identity: siwb_identity, identityAddress, clear: siwb_clear } = useSiwbIdentity();

    const [currentAccount, setCurrentAccount] = useConnectStore((state) => [state.currentAccount, state.setCurrentAccount])
    const { isLoading, setIsLoading } = CurrentStatus()
    const { wallet, walletList, walletSelect, setWallet, setBalance, setWalletList, setWalletSelect, setCurrentWallet } = WalletStore()
    const { updateWalletData } = useGetStakePool()

    const { IcpWallet, setIcpWallet, BindIcpWallet, setBindIcpWallet, setIcpNft } = IcpWalletStore()
    const { userImageList, setUserImageList } = UserInfo()
    const { stakedNFTs, setStakedNFTs } = StakeNftInfo()

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

    const get_wallets = async (osBackend) => {
        if (!osBackend) return;
        if (!checkIdentityExpiration(identity_select)) {
            clear();
            return;
        }

        const maxRetries = 3; // Maximum number of retries
        const retryDelay = 3000; // Delay between retries in milliseconds (3 seconds)
        let attempt = 0;
        let value = null;

        const delay = (ms) => new Promise(resolve => setTimeout(resolve, ms));

        while (attempt < maxRetries) {
            try {
                value = await osBackend.my_wallets();
                if (value && value.length > 0) {
                    if (!wallet) {
                        setWallet(value[0].bitcoin_address);
                        setWalletList(value);
                        setWalletSelect(value[0]);
                        setCurrentWallet(value[0].wallet_canister.toText());
                        // updateWalletData(value[0].bitcoin_address);
                    }
                } else {
                    if (!wallet) {
                        //await createWallet(osBackend)
                    }
                }
                return; // Exit the function if successful
            } catch (error) {
                attempt++;
                if (attempt === maxRetries) {
                    console.error("Error fetching walletList:", error);
                    // Your toast notification code here if needed
                } else {
                    console.log(`Attempt ${attempt} failed, retrying in ${retryDelay / 1000} seconds...`);
                    await delay(retryDelay);
                }
            }
        }

        console.log('end1');
        // setIsLoading(false);
    }

    const get_icp_bind = async (osBackend) => {
        console.log('-0--')
        if (!osBackend) return;
        const maxRetries = 3; // Maximum number of retries
        const retryDelay = 3000; // Delay between retries in milliseconds (3 seconds)
        let attempt = 0;
        let value = null;

        const delay = (ms) => new Promise(resolve => setTimeout(resolve, ms));

        while (attempt < maxRetries) {
            try {
                value = await osBackend.get_bind_wallets_by_user(identity_select.getPrincipal());
                if (value && value.length > 0 && value[0].length > 0) {
                    console.log('get');
                    console.log(value)
                    setBindIcpWallet(value[0][0].wallet_address);
                } else {
                    console.log('No bound wallets found');
                    // Handle the case where no wallets are bound
                    setBindIcpWallet(null); // or some default value
                }
                return; // Exit the function if successful
            } catch (error) {
                attempt++;
                if (attempt === maxRetries) {
                    console.error("Error fetching walletList:", error);
                    // Your toast notification code here if needed
                } else {
                    console.log(`Attempt ${attempt} failed, retrying in ${retryDelay / 1000} seconds...`);
                    await delay(retryDelay);
                }
            }
        }

        console.log('end1');
    }

    const bind_icpWallet = async (osBackend) => {
        if (!osBackend) return;
        if (!globalThis.ic.plug.agent.getPrincipal()) return;

        const IcpprincipalId = await globalThis.ic.plug.agent.getPrincipal();
        const maxRetries = 3; // Maximum number of retries
        const retryDelay = 3000; // Delay between retries in milliseconds (3 seconds)
        let attempt = 0;
        let value = null;

        const delay = (ms) => new Promise(resolve => setTimeout(resolve, ms));

        let walletType;
        if (identity_select === siwb_identity) {
            walletType = { BTC: null };
        } else if (identity_select === identity) {
            walletType = { ICP: null };
        } else {
            // Default case or error handling
            console.error('Unknown identity type');
            return;
        }

        const BindReq = {
            wallet_address: IcpprincipalId,
            sig_message: [],
            account: [],
            wallet_type: walletType
        };

        while (attempt < maxRetries) {
            try {
                value = await osBackend.bind_wallet(BindReq);
                if ('Ok' in value) {
                    setBindIcpWallet(IcpprincipalId.toText())
                    toast({
                        //title: 'Success',
                        description: 'Wallet Bind successfully',
                        status: 'success',
                        position: 'bottom-right',
                        duration: 9000,
                        isClosable: true,
                    });
                }
                return value; // Exit the function if successful
            } catch (error) {
                attempt++;
                if (attempt === maxRetries) {
                    console.error("Error fetching walletList:", error);
                    // Your toast notification code here if needed
                } else {
                    console.log(`Attempt ${attempt} failed, retrying in ${retryDelay / 1000} seconds...`);
                    await delay(retryDelay);
                }
            }
        }

        console.log('end1');
    }

    const get_icp_nft = async () => {
        //const publicKey = await globalThis.ic.plug.requestConnect();
        console.log('-----as')
        const nftActor = await globalThis.ic.plug.createActor({
            canisterId: nftCanisterId,
            interfaceFactory: idlFactory
        });

        // await globalThis.ic.plug.requestConnect({
        //     whitelist: [nftCanisterId],
        // });
        const principalId = await globalThis.ic.plug.agent.getPrincipal();
        const accountIdentifier = principalId.toText();

        const registry = await nftActor.getRegistry();
        const userAccountId = await globalThis.ic.plug.accountId;
        // Filter the registry to get only the user's NFTs
        const userNFTs = registry.filter(([_, accountId]) => accountId === userAccountId);

        // Extract just the token indices (NFT IDs)
        const userNFTIds = userNFTs.map(([tokenIndex, _]) => tokenIndex);
        setIcpNft(userNFTIds)

    }

    const stake_in_nft = async (stakeogBackend, nft_num_id) => {
        try {
            //const stakeActor = createActor_stake(canisterId_stake);
            const principalId = await globalThis.ic.plug.agent.getPrincipal();

            //const stakeCanisterId = 'b77ix-eeaaa-aaaaa-qaada-cai'

            const nftCanisterPrincipal = Principal.fromText(nftCanisterId);

            const connected = await globalThis.ic.plug.isConnected();
            if (!connected) {
                await globalThis.ic.plug.requestConnect({
                    whitelist: [nftCanisterId],
                    host: icphost
                    //host: 'http://127.0.0.1:4943'
                });
            }
            const nftActor = await globalThis.ic.plug.createActor({
                canisterId: nftCanisterId,
                interfaceFactory: idlFactory
            });

            const hash = await computeExtTokenIdentifier(nftCanisterPrincipal, nft_num_id);

            let approveRequest = {
                token: hash,
                allowance: 1n,
                spender: Principal.fromText(StakeCanisterId),
                subaccount: []
            };

            console.log('Sending approve request...');
            const stake = await nftActor.approve(approveRequest);
            console.log('Approval result:', stake);

            const stakeReq = {
                nft_id: nft_num_id,
                nft_owner: principalId,
                nft_canister: nftCanisterPrincipal
            }
            const stake_result = await stakeogBackend.stake_nft(stakeReq)
            get_icp_nft()
            return stake_result

        } catch (error) {
            console.error('Error in get_stake_info:', error);
            if (error instanceof Error) {
                console.error('Error message:', error.message);
                console.error('Error stack:', error.stack);
            }
        }
    };

    // unstake nft

    const unstake_in_nft = async (stakeogBackend, nft_num_id) => {
        try {
            //const stakeActor = createActor_stake(canisterId_stake);
            const principalId = await globalThis.ic.plug.agent.getPrincipal();

            //const stakeCanisterId = 'asrmz-lmaaa-aaaaa-qaaeq-cai'

            const nftCanisterPrincipal = Principal.fromText(nftCanisterId);

            const connected = await globalThis.ic.plug.isConnected();
            if (!connected) {
                await globalThis.ic.plug.requestConnect({
                    whitelist: [nftCanisterId],
                    host: icphost
                    //host: 'http://127.0.0.1:4943'
                });
            }

            const unstakeReq = {
                nft_id: nft_num_id,
                nft_canister: nftCanisterPrincipal
            }
            const unstake_result = await stakeogBackend.unstake_nft(unstakeReq)
            get_icp_nft()
            return unstake_result

        } catch (error) {
            console.error('Error in unstake:', error);
            if (error instanceof Error) {
                console.error('Error message:', error.message);
                console.error('Error unstack:', error.stack);
            }
        }
    };

    const get_user_stakednft = async (stakeogBackend) => {
        try {
            //const stakeActor = createActor_stake(canisterId_stake);
            //const principalId = await globalThis.ic.plug.agent.getPrincipal();
            const principalId = Principal.fromText(BindIcpWallet)
            const stake_result = await stakeogBackend.get_user_stake_nft(principalId)
            //const stake_result = await stakeogBackend.get_user_stake_nft_reward(principalId)
            const stakeInfo = stake_result.map(item => ({
                nft_id: item.nft_id,
                amount: item.amount,
                stake_at: item.stake_at
            }));
            setStakedNFTs(stakeInfo)
            console.log(stakedNFTs)
            return stake_result

        } catch (error) {
            console.error('Error in get_stake_info:', error);
            if (error instanceof Error) {
                console.error('Error message:', error.message);
                console.error('Error stack:', error.stack);
            }
        }
    }

    const get_user_stakednft_point = async (pointBackend) => {
        try {
            //const stakeActor = createActor_stake(canisterId_stake);
            //const principalId = await globalThis.ic.plug.agent.getPrincipal();
            const principalId = Principal.fromText(BindIcpWallet)
            const stake_result = await pointBackend.get_all_user_stake_nft_reward()
            //const stake_result = await stakeogBackend.get_user_stake_nft_reward(principalId)
            console.log('------------test')
            console.log(stake_result)
            return stake_result

        } catch (error) {
            console.error('Error in get_stake_info:', error);
            if (error instanceof Error) {
                console.error('Error message:', error.message);
                console.error('Error stack:', error.stack);
            }
        }
    }

    const get_user_image = async () => {
        const url = backend_ip + '/link/get_all_image'
        const response = await axios.get(url);
        if (response.data) {
            setUserImageList(response.data.data)
            return response.data
        }
        // const image = await osBackend.get_image_link();
        // console.log('-----test')
        // console.log(image)
    }

    const update_user_info = async (osBackend, userName, userImg) => {
        const userInfo: UserProfileRequest = {
            user_name: userName,
            user_id: identity_select.getPrincipal(),
            image_link: userImg
        }
        const res = await osBackend.update_user_profile(userInfo)
        if ('Ok' in res) {
            return true
            console.log('------testupdate')
            console.log(res)
        } else {
            return false
        }

    }

    const createWallet = async (osBackend) => {
        if (!osBackend || !identity) {
            return
        }

        try {
            const result = await osBackend.create_wallet_canister('Wallet01')

            if ('Err' in result && result.Err) {
                toast({
                    title: 'Error',
                    description: 'Create Error',
                    status: 'error',
                    position: 'bottom-right',
                    duration: 9000,
                    isClosable: true,
                });
            } else {
                toast({
                    //title: 'Success',
                    description: 'Wallet create successfully',
                    status: 'success',
                    position: 'bottom-right',
                    duration: 9000,
                    isClosable: true,
                });
            }

            await get_wallets(osBackend)
        } catch (error) {
            console.log(error)
        } finally {
            console.log('end')
        }
    }

    const get_balance = async (walletBackend, addr) => {
        if (!walletBackend) return;
        if (!identity_select) return;
        if (identity_select && !checkIdentityExpiration(identity_select)) {
            await clear();
            return;
        }

        const maxRetries = 3;
        const retryDelay = 3000; // 3 seconds

        for (let attempt = 1; attempt <= maxRetries; attempt++) {
            try {
                const balance = await walletBackend.balance(addr);

                if ('Err' in balance) {
                    throw new Error("get balance error");
                } else {
                    const b: bigint = balance.Ok;
                    setBalance(Number(b) / btcunity);
                    return Number(b);
                }
            } catch (error) {
                console.error(`Error updating wallet balance (attempt ${attempt}/${maxRetries}):`, error);

                if (attempt === maxRetries) {
                    // If it's the last attempt, show the error toast
                    // toast({
                    //     title: 'Error',
                    //     description: "Error updating wallet balance after multiple attempts",
                    //     status: 'error',
                    //     position: 'bottom-right',
                    //     duration: 9000,
                    //     isClosable: true,
                    //     variant: 'left-accent right-accent',
                    //     containerStyle: {
                    //         border: '1px solid #FF8800',
                    //         background: 'white',
                    //         color: 'red',
                    //         borderRadius: 'xl',
                    //         boxShadow: '0 0 10px rgba(255, 136, 0, 0.2)',
                    //     },
                    // });
                } else {
                    // If it's not the last attempt, wait before retrying
                    await new Promise(resolve => setTimeout(resolve, retryDelay));
                }
            }
        }

        console.log('end');
    }

    const onRefresh_balance = async () => {
        if (currentAccount && currentAccount.type === 'UNISAT') {
            const { type, address } = currentAccount
            const balance = await globalThis.unisat.getBalance()
            setCurrentAccount({
                address: address,
                type: type,
                balance: balance.total
            })
        }
        if (currentAccount && currentAccount.type === 'WIZZ') {
            const { type, address } = currentAccount
            const balance = await globalThis.wizz.getBalance()
            setCurrentAccount({
                address: address,
                type: type,
                balance: balance.total
            })
        }
    }

    const transfer_balance = async (walletBackend, transferAddress, transferBalance, btc) => {

        if (!walletBackend) return;
        setIsLoading(true)
        const amountInSatoshis = Math.round(transferBalance * btc); // Make sure it's an integer.

        const TransferInfo: TransferRequest = {
            txs: [
                {
                    recipient: transferAddress,
                    amount: BigInt(amountInSatoshis) // Make sure it's an integer.
                }
            ]
        };

        try {
            const result = await walletBackend.transfer_from_p2wsh_multisig22(TransferInfo);

            if ('Err' in result) {
                toast({
                    title: 'Transfer',
                    description: 'Transfer balance error',
                    status: 'error',
                    position: "bottom-right",
                    duration: 9000,
                    isClosable: true,
                    variant: 'left-accent right-accent',
                    containerStyle: {
                        border: '1px solid #FF8800',
                        background: 'white',
                        color: 'red',
                        borderRadius: 'xl',
                        boxShadow: '0 0 10px rgba(255, 136, 0, 0.2)',
                    },
                });
            } else {
                toast({
                    title: 'Transfer',
                    status: 'success',
                    position: 'bottom-right',
                    duration: 9000,
                    isClosable: true,
                    render: () => (
                        <Box color='white' p={3} bg='green.500'>
                            <Text>Transfer balance success</Text>
                            <Text>{`txid: ${result.Ok}`}</Text>
                        </Box>
                    ),
                });
            }

            setIsLoading(false);
            return true;
        } catch (error) {
            console.error('Error in transfer_balance:', error);
            setIsLoading(false);
            throw error; // Rethrow the error to be caught by the caller
        }

    }

    return {
        get_balance,
        get_icp_bind,
        bind_icpWallet,
        get_icp_nft,
        stake_in_nft,
        unstake_in_nft,
        get_user_stakednft,
        get_user_stakednft_point,
        onRefresh_balance,
        get_wallets,
        get_user_image,
        update_user_info,
        transfer_balance
    }
}

export default useGetWalletPool;
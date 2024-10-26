import React from 'react'
import { Box, Text, useToast } from '@chakra-ui/react'
import { createActor as createActor_os, canisterId as canisterId_os } from "../../../declarations/os/index";

import { WalletStore } from "../store/useWalletStore";
import { userStore, boxStore } from "../store/useMarathonStore";
import { useConnectStore } from '../store/useConnectStore';
import { CurrentStatus } from '../store';
import { IcpWalletStore } from '../store/useWalletStore';

import { useInternetIdentity } from 'ic-use-internet-identity';
import { useSiwbIdentity } from 'ic-use-siwb-identity';
import { formatDateminute } from './utils';
import useOsActor from "./osActor"

import { Principal } from '@dfinity/principal';

const useGetMarathon = () => {
    const toast = useToast()

    const { walletSelect } = WalletStore();
    const { userInfo, userReward, setUserInfo, setUserReward, setUserNftReward, setInviteInfo, setUserStake } = userStore();
    const { boxNum, boxRecord, remainingTimes, setBoxNum, setBoxReward, setBoxRecord, setRemainingTimes } = boxStore();

    const { BindIcpWallet } = IcpWalletStore()

    const { identity, isLoggingIn, login, clear, isLoginSuccess } = useInternetIdentity();
    const { identity: siwb_identity, identityAddress, clear: siwb_clear } = useSiwbIdentity();

    const [currentAccount, setCurrentAccount] = useConnectStore((state) => [state.currentAccount, state.setCurrentAccount])
    const { isLoading, setIsLoading } = CurrentStatus()

    const { os_login, os_logout } = useOsActor()

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
    const get_reward = async (pointBackend) => {
        if (!pointBackend) return;

        try {
            const value = await pointBackend.get_user_reward(identity_select.getPrincipal());
            setUserReward(value)
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
            console.error("Error fetching user reward:", error);
        } finally {
            console.log('end');
        }
    }

    // get point of nft module
    const get_nft_reward = async (pointBackend) => {
        if (!pointBackend) return;
        const IcpWalletPrincipal = Principal.fromText(BindIcpWallet);
        try {
            const value = await pointBackend.get_user_stake_nft_reward(IcpWalletPrincipal);
            console.log('-test')
            console.log(value[0].stake_nft_point)
            if (Array.isArray(value) && value.length > 0 && 'stake_nft_point' in value[0]) {
                console.log(value[0].stake_nft_point);
                setUserNftReward(value[0].stake_nft_point);
            } else {
                console.log('No valid stake_nft_point found');
                setUserNftReward(null);
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
            console.error("Error fetching user reward:", error);
        } finally {
            console.log('end');
        }
    }

    // get boxes info
    const get_boxes = async (pointBackend) => {
        if (!pointBackend) return;

        try {
            const value = await pointBackend.get_user_box_reward(identity_select.getPrincipal());
            if (value && value[0].unopen_box_count) {
                setBoxNum(value[0].unopen_box_count)
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

    // get open boxes records info
    const get_user_stakebtc = async (pointBackend) => {
        if (!pointBackend) return;

        try {
            const value = await pointBackend.get_user_stake_reward(identity_select.getPrincipal());
            setUserStake(value[0])
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

    // get open boxes records info
    const get_boxes_record = async (pointBackend) => {
        if (!pointBackend) return;

        try {
            const value = await pointBackend.get_user_open_boxes(identity_select.getPrincipal());
            const sortedValue = value[0].sort((a, b) => {
                return Number(b.create_time) - Number(a.create_time);
            });
            setBoxRecord(sortedValue);
            console.log(value)
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


    // get user info
    const get_info = async (osBackend) => {

        try {
            const value = await osBackend.get_user_info(identity_select.getPrincipal());

            if (value.length > 0) {
                setUserInfo(value[0])
                if (value[0].last_reward_at === 0n) {
                    setRemainingTimes([0])
                    return
                }

                const lastRewardAt = BigInt(value[0].last_reward_at);
                const currentTimeUTC = BigInt(Date.now()) * BigInt(1000000);
                // Calculation of time difference (in seconds)
                const timeDiffSeconds = Number(currentTimeUTC - lastRewardAt) / 1000000000;

                if (timeDiffSeconds >= 24 * 3600) {
                    setRemainingTimes([0])
                    return
                } else {
                    const remainingSeconds = 24 * 3600 - timeDiffSeconds;
                    setRemainingTimes([remainingSeconds])
                }
            } else {
                const result = await os_login(osBackend)
                if (result === 'ok') {
                    get_info(osBackend)
                }
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

    // get user of invite info
    const get_invite_info = async (pointBackend) => {

        try {
            const value = await pointBackend.get_user_invite_reward(identity_select.getPrincipal());
            setInviteInfo(value[0])
            console.log(value)
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

    //user login can get a box
    const update_login = async (osBackend, pointBackend) => {
        if (!osBackend) return
        if (isLoading) return
        setIsLoading(true)
        try {
            const value = await osBackend.update_login_reward(identity_select.getPrincipal());
            const currentBoxNum = boxStore.getState().boxNum;
            setBoxNum(Number(currentBoxNum) + 1);
            get_boxes(pointBackend)
            const remainingSeconds = 24 * 60 * 60
            setRemainingTimes([Math.round(remainingSeconds)])
            setIsLoading(false)
            toast({
                title: 'Check In',
                status: 'success',
                position: 'bottom-right',
                duration: 9000,
                isClosable: true,
                render: () => (
                    <Box color='white' p={3} bg='green.500'>
                        <Text>Check in success</Text>
                    </Box>
                ),
            });
            console.log(value)
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
            setIsLoading(false)
            console.error("Error fetching staking pool:", error);
        } finally {
            setIsLoading(false)
            console.log('end');
        }
    }

    //update invite code of user
    const update_code = async (osBackend, code) => {
        setIsLoading(true)
        try {
            const value = await osBackend.update_invited_code(identity_select.getPrincipal(), code);
            console.log(value)
            if ('Ok' in value) {
                toast({
                    title: 'Transfer',
                    status: 'success',
                    position: 'bottom-right',
                    duration: 9000,
                    isClosable: true,
                    render: () => (
                        <Box color='white' p={3} bg='green.500'>
                            <Text>code success</Text>
                        </Box>
                    ),
                });
            }
            if ('Err' in value && 'InviteCodeError' in value.Err) {
                toast({
                    title: "InviteCode Error",
                    description: "Please check your invitation code and re-enter it.",
                    status: "error",
                    position: 'bottom-right',
                    duration: 3000,
                    isClosable: true,
                });
            } else if ('Err' in value) {
                toast({
                    title: "Error",
                    description: "Please try again later.",
                    status: "error",
                    position: 'bottom-right',
                    duration: 3000,
                    isClosable: true,
                });
            }
            get_info(osBackend)
            setIsLoading(false)
            console.log(value)
        } catch (error) {
            setIsLoading(false)
            toast({
                title: 'Info',
                description: "code error",
                status: 'error',
                position: 'bottom-right',
                duration: 9000,
                isClosable: true,
            });
            console.error("Error add code:", error);
        } finally {
            console.log('end');
            setIsLoading(false)
        }
    }

    const open_box = async (pointBackend) => {
        setIsLoading(true)
        try {
            const value = await pointBackend.open_all_box(identity_select.getPrincipal());
            if ('Ok' in value) {
                setBoxReward(value.Ok)
                setBoxNum(0)
                await get_boxes(pointBackend)
                await get_reward(pointBackend)
                await get_boxes_record(pointBackend)
            }
            setIsLoading(false)
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
            setIsLoading(false)
        } finally {
            console.log('end');
            setIsLoading(false)
        }
    }


    return {
        get_reward,
        get_nft_reward,
        get_boxes,
        get_user_stakebtc,
        get_boxes_record,
        update_login,
        update_code,
        get_info,
        get_invite_info,
        open_box,
    };
};

export default useGetMarathon;
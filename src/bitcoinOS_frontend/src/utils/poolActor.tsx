import React from 'react'
import { useToast } from '@chakra-ui/react';
import { StakingRequest, StakingRecords } from '../ic/WalletActors';
import StakePoolActors, { StakingRecords as allStakingRecords, StakingRecord as allStakingRecord } from '../ic/StakePoolActors';
import { useStakeListStore, useAllInfo, StakepoolStore } from "../store/index";
import { backend_ip, checkIdentityExpiration } from './utils';
import { useInternetIdentity } from "ic-use-internet-identity";
import { useSiwbIdentity } from 'ic-use-siwb-identity';
import { Box, Text } from '@chakra-ui/react';
import { createActor as createActor_os, canisterId as canisterId_os } from "../../../declarations/os/index";
import { createActor as createActor_staking, canisterId as canisterId_staking } from "../../../declarations/stakingpool/index";
import { createActor as createActor_wallet, canisterId as canisterId_wallet } from "../../../declarations/smartwallet/index";
import { createActor as createActor_point, canisterId as canisterId_point } from "../../../declarations/point/index";


import { StakingRecord } from '../ic/WalletActors';
import { RewardRecord, LeaderBoardStatus } from '../ic/PointActors';
import { RedeemRequest, RedeemResponse } from '../ic/StakePoolActors';
import { useConnectStore } from '../store/useConnectStore';
import { usePointStore, usePoolrecordStore } from '../store/useStakePool';
import { WalletStore } from '../store/useWalletStore';
import { CurrentStatus, getNetworkInfo } from '../store';
import { userStore } from '../store/useMarathonStore';

import { PriceRecord } from '../ic/PointActors';

import axios from 'axios';

const useGetStakePool = () => {
    const { isLoading, setIsLoading } = CurrentStatus()

    const [currentAccount, setCurrentAccount] = useConnectStore((state) => [state.currentAccount, state.setCurrentAccount])
    const { btcprice, setBtcprice, network, setNetwork } = getNetworkInfo()
    const { totalBalance, setTotalBalance } = WalletStore()
    const { setTvl, setUsers, userslist, setUserslist, tvllist, setTvllist } = useAllInfo();
    const setStakeList = useStakeListStore((state) => state.setStakeList);

    const { userReward, setUserReward, setUserCredit } = userStore();
    const { pointRank, rankStatus, setPointRank, setCreditRank, setRankStatus } = usePointStore()
    const { stakeRecords, setStakeRecords, allStakeRecords, setAllStakeRecords } = usePoolrecordStore();

    const { stakepoolCanister, setStakepoolCanister } = StakepoolStore()

    const toast = useToast();
    const { identity, clear } = useInternetIdentity();
    const { identity: siwb_identity, identityAddress, clear: siwb_clear } = useSiwbIdentity();

    const btcunity = 100000000;

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

    const get_btc_price = async () => {
        if (btcprice !== 0) return;

        let attempts = 0;
        const maxAttempts = 3;

        while (attempts < maxAttempts) {
            try {
                //const result = await createActor_point(canisterId_point).get_btc_price();
                //setBtcprice(Number(result.price) / 100);
                setBtcprice(1)
                return; // Exit the function if the request is successful
            } catch (error) {
                attempts++;
                console.error(`Attempt ${attempts} failed:`, error);
                if (attempts >= maxAttempts) {
                    toast({
                        title: 'Error',
                        description: "Failed to fetch BTC price after multiple attempts.",
                        status: 'error',
                        position: 'bottom-right',
                        duration: 9000,
                        isClosable: true,
                        variant: 'left-accent right-accent',
                        containerStyle: {
                            border: '1px solid #FF8800',
                            background: 'white',
                            color: 'red',
                            borderRadius: 'xl', // 可选：添加圆角
                            boxShadow: '0 0 10px rgba(255, 136, 0, 0.2)', // 可选：添加阴影效果
                        },
                    });
                }
            }
        }
    };

    const get_btc_fee = async (walletBackend) => {
        if (!walletBackend) return;
        if (identity_select && !checkIdentityExpiration(identity_select)) {
            await clear();
            setCurrentAccount(undefined);
            return;
        }

        const maxRetries = 3; // Maximum number of retries
        let attempt = 0;
        let response;

        while (attempt < maxRetries) {
            try {
                response = await walletBackend.current_fee_percentiles();
                if (response.Ok) {
                    const feeArray = Array.from(response.Ok);
                    feeArray.sort((a, b) => (a < b ? -1 : a > b ? 1 : 0)); // Sort the array using bigint comparison

                    const midIndex = Math.floor(feeArray.length / 2);

                    let median;
                    if (feeArray.length % 2 === 0) {
                        // If even number of elements, average the two middle elements
                        median = (Number(feeArray[midIndex - 1]) + Number(feeArray[midIndex])) / 2;
                    } else {
                        // If odd number of elements, take the middle element
                        median = Number(feeArray[midIndex]);
                    }

                    const medianFee = median / btcunity;
                    return medianFee;
                } else {
                    console.error("Response does not contain 'Ok' key:", response);
                    return null;
                }
            } catch (error) {
                attempt++;
                if (attempt === maxRetries) {
                    console.error("Error fetching fee data after 3 attempts:", error);
                    // Display toast or handle error here
                    toast({
                        title: 'Info',
                        description: "get BTC fee error after multiple attempts",
                        status: 'error',
                        position: 'bottom-right',
                        duration: 9000,
                        isClosable: true,
                        variant: 'left-accent right-accent',
                        containerStyle: {
                            border: '1px solid #FF8800',
                            background: 'white',
                            color: 'red',
                            borderRadius: 'xl', // 可选：添加圆角
                            boxShadow: '0 0 10px rgba(255, 136, 0, 0.2)', // 可选：添加阴影效果
                        },
                    });
                } else {
                    console.log(`Attempt ${attempt} failed, retrying...`);
                }
            }
        }
    }

    const get_stake_pool = async (osBackend: any) => {
        if (!osBackend) return;

        const osBackend_c = createActor_os(canisterId_os)
        //await get_btc_price()

        async function delay(ms: number): Promise<void> {
            return new Promise(resolve => setTimeout(resolve, ms));
        }
        async function fetchTvlWithRetry(staking_pool_canister: string, retries = 3, delayMs = 2000): Promise<any> {
            for (let attempt = 1; attempt <= retries; attempt++) {
                try {
                    const result = await osBackend.tvl_of(staking_pool_canister);
                    return result;
                } catch (error) {
                    if (attempt < retries) {
                        console.warn(`Attempt ${attempt} failed, retrying after ${delayMs}ms...`);
                        await delay(delayMs);
                        // Optional: increase the delay time, e.g. double the delay time for each retry
                        // delayMs *= 2;
                    } else {
                        throw error;
                    }
                }
            }
        }

        try {
            const value = await osBackend.list_staking_pool();
            setStakepoolCanister(value[0].staking_pool_canister)
            const results = await Promise.all(value.map(async (pool) => {
                const { staking_pool_canister } = pool;
                const result = await fetchTvlWithRetry(staking_pool_canister);
                //const tvl_u = Number(result) * btcprice
                return {
                    ...pool,
                    tvl: Number(result),
                };
            }));

            setStakeList(results);
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
    };

    const updateWalletData = async (walletBackend: any) => {
        if (!walletBackend) return;
        if (!identity) return
        if (identity_select && !checkIdentityExpiration(identity_select)) {
            await clear()
            setCurrentAccount(undefined)
            return
        }

        try {
            const [stakeRecordsResult] = await Promise.all([
                walletBackend.list_staking()
            ]);

            if ('Ok' in stakeRecordsResult) {
                const records: StakingRecord[] = stakeRecordsResult.Ok;
                setStakeRecords(records);
                let r: bigint = 0n;
                records.forEach((record) => {
                    r += record.sent_amount;
                });
                setTotalBalance(Number(r) / btcunity);
            }

        } catch (error) {
            console.error('Error updating wallet data:', error);
            {/*
            toast({
                title: 'Error',
                description: "Error updating wallet data",
                status: 'error',
                position: 'bottom-right',
                duration: 9000,
                isClosable: true,
            });
            */}
        } finally {
            console.log('end');
        }
    };

    //stake
    const stake_balance = async (walletBackend: any, stakeRequest) => {
        if (!walletBackend) return;
        if (identity_select && !checkIdentityExpiration(identity_select)) {
            await clear()
            setCurrentAccount(undefined)
            return
        }

        try {
            const result = await walletBackend.staking_to_pool_from_p2wsh_multisig22(stakeRequest);
            if ('Err' in result) {
                toast({
                    title: 'Stake',
                    description: "stake balance error",
                    status: 'error',
                    position: "bottom-right",
                    duration: 9000,
                    isClosable: true,
                    variant: 'left-accent right-accent',
                    containerStyle: {
                        border: '1px solid #FF8800',
                        background: 'white',
                        color: 'red',
                        borderRadius: 'xl', // 可选：添加圆角
                        boxShadow: '0 0 10px rgba(255, 136, 0, 0.2)', // 可选：添加阴影效果
                    },
                });
            } else {
                toast({
                    title: 'Stake',
                    status: 'success',
                    position: "bottom-right",
                    duration: 9000,
                    isClosable: true,
                    render: () => (
                        <Box color='white' p={3} bg='green.500'>
                            <Text>stake balance success</Text>
                            <Text>{"txid:" + result.Ok}</Text>
                        </Box>
                    )
                });
            }
        } catch (error) {
            toast({
                title: 'Stake',
                description: "stake balance error",
                status: 'error',
                position: "bottom-right",
                duration: 9000,
                isClosable: true,
                variant: 'left-accent right-accent',
                containerStyle: {
                    border: '1px solid #FF8800',
                    background: 'white',
                    color: 'red',
                    borderRadius: 'xl', // 可选：添加圆角
                    boxShadow: '0 0 10px rgba(255, 136, 0, 0.2)', // 可选：添加阴影效果
                },
            });
            console.error("Error staking balance:", error);
        } finally {
            console.log('end')
        }
    };

    //unstake balance
    const unstake_balance = async (stakeBackend, txid, addr, network_in) => {
        //if (!walletBackend) return
        if (!stakeBackend) return
        if (isLoading) return
        //if (!stakeCanister) return
        //setIsLoading(true);
        const unstakeRequest: RedeemRequest = {
            'txid': txid,
            'recipient': addr,
            'network': network_in,
        }

        try {
            setIsLoading(true)
            const result: RedeemResponse = await stakeBackend.redeem_from_p2wsh_multisig22(unstakeRequest)
            if ('Ok' in result) {
                setIsLoading(false)
                toast({
                    title: 'Unstake',
                    status: 'success',
                    position: "bottom-right",
                    duration: 9000,
                    isClosable: true,
                    render: () => (
                        <Box color='white' p={3} bg='green.500'>
                            <Text>unstake balance success</Text>
                            <Text>{"txid:" + result.Ok}</Text>
                        </Box>
                    )
                })
            }
            if ('Err' in result) {
                console.log("stake err", result)
                // Getting Keys and Values in an Err Object
                const errorEntries = Object.entries(result.Err);
                let errorMessage = '';

                // Generate an error message string
                if (errorEntries.length > 0) {
                    const [key, value] = errorEntries[0];
                    errorMessage = `${key}: ${value}`;
                }

                toast({
                    title: 'Unstake',
                    description: errorMessage || "unstake balance error",
                    status: 'error',
                    position: "bottom-right",
                    duration: 9000,
                    isClosable: true,
                    variant: 'left-accent right-accent',
                    containerStyle: {
                        border: '1px solid #FF8800',
                        background: 'white',
                        color: 'red',
                        borderRadius: 'xl', // 可选：添加圆角
                        boxShadow: '0 0 10px rgba(255, 136, 0, 0.2)', // 可选：添加阴影效果
                    },
                })
            }

        } catch (error) {
            setIsLoading(false)
            toast({
                title: 'Unstake',
                description: "unstake balance error",
                status: 'error',
                position: "bottom-right",
                duration: 9000,
                isClosable: true,
                variant: 'left-accent right-accent',
                containerStyle: {
                    border: '1px solid #FF8800',
                    background: 'white',
                    color: 'red',
                    borderRadius: 'xl', // 可选：添加圆角
                    boxShadow: '0 0 10px rgba(255, 136, 0, 0.2)', // 可选：添加阴影效果
                },
            })
        } finally {
            setIsLoading(false)
        }
    }

    //Get Current Network
    const get_network = async () => {
        if (network !== '') return;
        if (identity_select && !checkIdentityExpiration(identity_select)) {
            await clear()
            setCurrentAccount(undefined)
            return
        }
        const walletBackend_c = createActor_os(canisterId_os)
        const maxRetries = 3; // Maximum number of retries
        let attempt = 0;
        let result = null;

        while (attempt < maxRetries) {
            try {
                result = await walletBackend_c.metadata();
                if (result && result.network) {
                    const networkKeys = Object.keys(result.network);
                    console.log("Network keys:", networkKeys);

                    // If only the first key value is needed
                    const firstKey = networkKeys.length > 0 ? networkKeys[0] : null;
                    setNetwork(firstKey)
                }
                return result;
            } catch (error) {
                attempt++;
                if (attempt === maxRetries) {
                    toast({
                        title: 'Network',
                        description: "get network error",
                        status: 'error',
                        position: "bottom-right",
                        duration: 9000,
                        isClosable: true,
                        variant: 'left-accent right-accent',
                        containerStyle: {
                            border: '1px solid #FF8800',
                            background: 'white',
                            color: 'red',
                            borderRadius: 'xl', // 可选：添加圆角
                            boxShadow: '0 0 10px rgba(255, 136, 0, 0.2)', // 可选：添加阴影效果
                        },
                    });
                    console.error("Error staking balance:", error);
                }
            } finally {
                if (result) {
                    break; // Exit loop if successful
                }
                console.log(`Attempt ${attempt} failed, retrying...`);
            }
        }

        console.log('end');
    }

    //Get Tvl、User
    const get_tvl_user = async (osBackend) => {
        if (!osBackend) return;
        await get_btc_price()
        try {
            const result = await osBackend.count_wallet()
            const tvl = await osBackend.tvl()
            setTvl(Number(tvl) / btcunity)
            setUsers(Number(result))
            //setUsers(Number(value));

        } catch (error) {
            /*
            toast({
                title: 'Info',
                description: "get walletCount error",
                status: 'error',
                position: 'bottom-right',
                duration: 9000,
                isClosable: true,
            });
            */
            console.error("Error fetching tvl and user count:", error);
        } finally {
            console.log('end');
        };
    }
    const get_user_list = async () => {
        const MAX_RETRIES = 3;
        let retryCount = 0;
        try {
            const user_url = backend_ip + '/stat/get_user_stat'
            const user_response = await axios.get(user_url);

            if (user_response.data && user_response.data.data) {
                if (user_response.data.data) {
                    const stats = user_response.data.data.slice(0, 5).map((item, index) => ({
                        period: index === 0 ? 'Today' :
                            index === 1 ? 'Yesterday' :
                                `${index} days ago`,
                        count: Number(item.user_count)
                    }));

                    const counts = stats.map(stat => stat.count);

                    const reversedCounts = counts.slice().reverse();
                    console.log(reversedCounts)
                    setUserslist(reversedCounts);
                    return stats;
                } else {
                    console.error('Expected an array for point data, but got:', typeof user_response.data.data)
                    setUserslist(null)
                }
            } else {
                setUserslist(null)
            }


            // if (!osBackend) return;

            // const result = await osBackend.list_wallet();
            // // Get current time and target time
            // const now = new Date();
            // const todayMidnight = new Date(now);
            // todayMidnight.setHours(23, 59, 59, 999);

            // // Calculate the date of yesterday, the day before yesterday, the day before yesterday, the day before that, and the day before that.
            // const yesterday = new Date(todayMidnight);
            // yesterday.setDate(yesterday.getDate() - 1);
            // yesterday.setHours(23, 59, 59, 999);

            // const twoDaysAgo = new Date(todayMidnight);
            // twoDaysAgo.setDate(twoDaysAgo.getDate() - 2);
            // twoDaysAgo.setHours(23, 59, 59, 999);

            // const threeDaysAgo = new Date(todayMidnight);
            // threeDaysAgo.setDate(threeDaysAgo.getDate() - 3);
            // threeDaysAgo.setHours(23, 59, 59, 999);

            // const fourDaysAgo = new Date(todayMidnight);
            // fourDaysAgo.setDate(fourDaysAgo.getDate() - 4);
            // fourDaysAgo.setHours(23, 59, 59, 999);

            // // Constructing the result array
            // const stats = [
            //     {
            //         period: 'Today before 12:00 PM',
            //         count: 0,
            //     },
            //     {
            //         period: 'Yesterday',
            //         count: 0,
            //     },
            //     {
            //         period: 'Two days ago',
            //         count: 0,
            //     },
            //     {
            //         period: 'Three days ago',
            //         count: 0,
            //     },
            //     {
            //         period: 'Four days ago',
            //         count: 0,
            //     },
            // ];

            // // Counting the number of eligible data in an array
            // result.forEach(item => {
            //     const a = Number(item.created_at) / 1000000;
            //     const createdAt = new Date(a);
            //     if (createdAt < todayMidnight) {
            //         stats[0].count++;
            //     }

            //     if (createdAt < yesterday) {
            //         stats[1].count++;
            //     }

            //     if (createdAt < twoDaysAgo) {
            //         stats[2].count++;
            //     }

            //     if (createdAt < threeDaysAgo) {
            //         stats[3].count++;
            //     }

            //     if (createdAt < fourDaysAgo) {
            //         stats[4].count++;
            //     }
            // });

            // const counts = stats.map(stat => stat.count);
            // const reversedCounts = counts.slice().reverse();
            // setUserslist(reversedCounts);
            // return stats;

        } catch (error) {
            console.error('Error fetching data:', error);
            if (retryCount < MAX_RETRIES) {
                retryCount++;
                console.log(`Retrying... Attempt ${retryCount}`);
                return await get_user_list();
            } else {
                console.error('Max retries exceeded. Exiting retry loop.');
            }
        }
    };
    const get_tvl_list = async () => {


        const MAX_RETRIES = 3;
        let retryCount = 0;

        try {
            const stake_url = backend_ip + '/stat/get_stake_stat'
            const stake_response = await axios.get(stake_url);

            if (stake_response.data && stake_response.data.data) {
                if (stake_response.data.data) {
                    const stats = stake_response.data.data.slice(0, 5).map((item, index) => ({
                        period: index === 0 ? 'Today' :
                            index === 1 ? 'Yesterday' :
                                `${index} days ago`,
                        count: Number(item.stake_ammount) / btcunity
                    }));

                    const counts = stats.map(stat => stat.count);

                    const reversedCounts = counts.slice().reverse();
                    setTvllist(reversedCounts);
                    return stats;
                } else {
                    console.error('Expected an array for point data, but got:', typeof stake_response.data.data)
                    setTvllist(null)
                }
            } else {
                setTvllist(null)
            }

            // if (!osBackend) return;
            // const result = await osBackend.list_staking_record();

            // // Get current time and target time (today midnight)
            // const now = new Date();
            // const todayMidnight = new Date(now);
            // todayMidnight.setHours(23, 59, 59, 999);

            // // Calculate the date of yesterday, the day before yesterday, the day before yesterday, the day before that, and the day before that.
            // const yesterday = new Date(todayMidnight);
            // yesterday.setDate(yesterday.getDate() - 1);
            // yesterday.setHours(23, 59, 59, 999);

            // const twoDaysAgo = new Date(todayMidnight);
            // twoDaysAgo.setDate(twoDaysAgo.getDate() - 2);
            // twoDaysAgo.setHours(23, 59, 59, 999);

            // const threeDaysAgo = new Date(todayMidnight);
            // threeDaysAgo.setDate(threeDaysAgo.getDate() - 3);
            // threeDaysAgo.setHours(23, 59, 59, 999);

            // const fourDaysAgo = new Date(todayMidnight);
            // fourDaysAgo.setDate(fourDaysAgo.getDate() - 4);
            // fourDaysAgo.setHours(23, 59, 59, 999);

            // // Constructing the result array
            // const stats = [
            //     {
            //         period: 'Today before 12:00 PM',
            //         count: 0,
            //     },
            //     {
            //         period: 'Yesterday',
            //         count: 0,
            //     },
            //     {
            //         period: 'Two days ago',
            //         count: 0,
            //     },
            //     {
            //         period: 'Three days ago',
            //         count: 0,
            //     },
            //     {
            //         period: 'Four days ago',
            //         count: 0,
            //     },
            // ];

            // // Counting the number of eligible data in the array
            // result.forEach(item => {
            //     const createdAt = new Date(Number(item.sent_time) / 1000000); // Convert sent_time to Date object
            //     const sentAmount = Number(item.sent_amount) / btcunity; // Ensure sent_amount is a number
            //     if (createdAt < todayMidnight) {
            //         stats[0].count += sentAmount;
            //     }

            //     if (createdAt < yesterday) {
            //         stats[1].count += sentAmount;
            //     }

            //     if (createdAt < twoDaysAgo) {
            //         stats[2].count += sentAmount;
            //     }

            //     if (createdAt < threeDaysAgo) {
            //         stats[3].count += sentAmount;
            //     }

            //     if (createdAt < fourDaysAgo) {
            //         stats[4].count += sentAmount;
            //     }
            // });

            // console.log('Statistics:', stats);

        } catch (error) {
            console.error('Error fetching data:', error);
            if (retryCount < MAX_RETRIES) {
                retryCount++;
                console.log(`Retrying... Attempt ${retryCount}`);
                return await get_tvl_list();
            } else {
                console.error('Max retries exceeded. Exiting retry loop.');
                // Handle max retries exceeded error
            }
        }
    };

    //Get Pool Pledge Records
    const get_stake_records = async (pool_canister: string, addr: string) => {
        if (identity_select && !checkIdentityExpiration(identity_select)) {
            await clear();
            setCurrentAccount(undefined);
            return;
        }
        if (!addr || addr.length <= 1) {
            return;
        }
        const stakeBackend_c = createActor_staking(pool_canister);

        let attempts = 0;
        const maxAttempts = 3;
        const retryDelay = 3000; // 3 seconds delay between attempts

        while (attempts < maxAttempts) {
            try {
                const v = await stakeBackend_c.list_staking_by_wallet(addr);
                if ('Ok' in v) {
                    const records = v.Ok;

                    // Sort the records based on sent_time in descending order (newest first)
                    const sortedRecords = records.sort((a, b) => {
                        return Number(b.sent_time - a.sent_time);
                    });

                    setStakeRecords(sortedRecords);
                    let r: bigint = 0n;
                    v.Ok.forEach((record) => {
                        r += record.sent_amount;
                    });
                    setTotalBalance(Number(r) / btcunity);
                    return; // Success, exit the function
                }
            } catch (error) {
                attempts++;
                console.error(`Error getting stake records (attempt ${attempts}):`, error);
                if (attempts >= maxAttempts) {
                    console.error('Max attempts reached. Could not get stake records.');
                } else {
                    // Wait for the specified delay before the next attempt
                    await new Promise(resolve => setTimeout(resolve, retryDelay));
                }
            } finally {
                console.log('end');
            }
        }
    };

    // get pools all records
    const get_allstake_records = async (pool_canister: string) => {
        const stakeBackend_c = createActor_staking(pool_canister);

        let attempts = 0;
        const maxAttempts = 3;

        while (attempts < maxAttempts) {
            try {
                const v: allStakingRecords = await stakeBackend_c.list_staking();
                if ('Ok' in v) {
                    const records: allStakingRecord[] = v.Ok;

                    // Sort the records based on sent_time in descending order (newest first)
                    const sortedRecords = records.sort((a, b) => {
                        return Number(b.sent_time - a.sent_time);
                    });

                    setAllStakeRecords(sortedRecords);
                    return;
                }
            } catch (error) {
                attempts++;
                console.error(`Error getting stake records (attempt ${attempts}):`, error);
                if (attempts >= maxAttempts) {
                    console.error('Max attempts reached. Could not get stake records.');
                }
            } finally {
                console.log('end');
            }
        }
    };

    // get point ranks info
    const get_pointrank = async () => {
        try {
            const point_url = backend_ip + '/leader/get_leader_board/1'
            const user_point = backend_ip + '/leader/' + identity_select.getPrincipal().toText() + '/1'

            const point_response = await axios.get(point_url);
            const userpoint_response = await axios.get(user_point);

            // Processing user data
            if (userpoint_response.data && userpoint_response.data.data) {
                if (userpoint_response.data.data) {
                    setUserCredit(userpoint_response.data.data)
                } else {
                    console.error('Expected an array for point data, but got:', typeof userpoint_response.data.data)
                    setUserCredit(null)
                }
            } else {
                setUserCredit(null)
            }

            // Processing point data
            if (point_response.data && point_response.data.data) {
                if (Array.isArray(point_response.data.data)) {
                    const sortedPointData = [...point_response.data.data].sort((a, b) => a.point_rank - b.point_rank);
                    setPointRank(sortedPointData)
                } else {
                    console.error('Expected an array for point data, but got:', typeof point_response.data.data)
                    setPointRank([])
                }
            }


            return {
                pointData: point_response.data,
                creditData: userpoint_response.data
            }
        } catch (error) {
            console.error('Error fetching rank data:', error)
            setPointRank([])
            setCreditRank([])
        }

        // if (!pointBackend) return;

        // try {
        //     const value: RewardRecord[] = await pointBackend.get_leader_board();
        //     const sortedValue = value.sort((a, b) => Number(b.total_point) - Number(a.total_point));

        //     setPointRank(sortedValue);
        // } catch (error) {
        //     /*
        //     toast({
        //         title: 'Info',
        //         description: "get stake error",
        //         status: 'error',
        //         position: 'bottom-right',
        //         duration: 9000,
        //         isClosable: true,
        //     });
        //     */
        //     console.error("Error fetching staking pool:", error);
        // } finally {
        //     console.log('end');
        // }
    }

    // get point ranks info
    const get_pointrank_status = async (pointBackend) => {
        if (!pointBackend) return;

        try {
            const value: LeaderBoardStatus = await pointBackend.get_leader_board_status();
            setRankStatus(value);
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

    return {
        get_btc_price,
        get_btc_fee,
        get_stake_pool,
        stake_balance,
        unstake_balance,
        get_network,
        get_tvl_user,
        get_user_list,
        get_tvl_list,
        updateWalletData,
        get_stake_records,
        get_allstake_records,
        get_pointrank,
        get_pointrank_status
    };
};

export default useGetStakePool;
import {
    Box,
    Flex,
    Spacer,
    Heading,
    Text,
    Image,
    Input,
    Button,
    HStack,
    VStack,
    InputGroup,
    InputRightElement,
    InputLeftElement,
    useDisclosure,
    Modal,
    ModalOverlay,
    ModalContent,
    ModalHeader,
    ModalFooter,
    ModalCloseButton,
    FormControl,
    ModalBody,
    FormLabel,
    Spinner,
    Checkbox,
    Tag,
    TagLabel,
    Grid,
    IconButton,
    GridItem,
    CheckboxGroup,
    StatHelpText
} from '@chakra-ui/react'
import {
    Table,
    Thead,
    Tbody,
    Tfoot,
    Tr,
    Th,
    Td,
} from '@chakra-ui/react'
import { useToast } from '@chakra-ui/react'
import { BsCopy, BsCheckCircleFill, BsBoxArrowUpRight, BsArrowClockwise } from "react-icons/bs";
import { Select, Card, CardHeader, CardBody, CardFooter } from '@chakra-ui/react'
import { checkIdentityExpiration, truncateMiddle, formatDate, formatDateminute, getLastLoggedInWallet, isThirdPartyWallet, satoshisToBTC } from '../../utils/utils';
import useGetStakePool from '../../utils/poolActor'
import useGetWalletPool from '../../utils/walletActor';
//Components of Line Charts
import CurvedLineChart from '../../components/chart/CurvedLineChart';
import AssetChart from '../../components/chart/AssetChart';
import ConnectModal from "../../components/ConnectModal/ConnectModal";
import WalletList from "../../components/ConnectModal/WalletList";

import React, { useEffect, useState, useRef } from 'react';
import { StakepoolStore, useStakeListStore, useCurrentPoolStore } from "../../store/index"
import { WalletStore } from '../../store/useWalletStore'
import { usePoolrecordStore } from '../../store/useStakePool';
import { Metadata, useWalletBackend, Result_1 as BalanceResult, StakingRequest, StakingType, Result_3 as StakeResult, StakingRecords, StakingRecord, MetadataRecords, TransferRequest } from "../../ic/WalletActors";
import { TotalStakingRequest, utxosRecords, UtxosRequest, UtxosResponse } from "../../ic/WalletActors";
import { RedeemRequest } from "../../ic/StakePoolActors"
import { useOsBackend, WalletInfo, Result as StakingPoolResult, StakingPoolInfo, CreateStakingPoolRequest } from "../../ic/OsActors";
import { useSatkePoolBackend, StakingRecords as allStakingRecords, StakingRecord as allStakingRecord } from "../../ic/StakePoolActors";
import { usePointBackend } from '../../ic/PointActors';
import { useInternetIdentity } from "ic-use-internet-identity";
import { useSiwbIdentity } from 'ic-use-siwb-identity';
import { Principal } from "@dfinity/principal"
import { stakingpool } from '../../../../declarations/stakingpool'
import { RedeemResponse } from '../../ic/StakePoolActors'
import { createActor, canisterId as test123 } from "../../../../declarations/os/index";
import { createActor as createActor_staking } from "../../../../declarations/stakingpool/index";
import { ConnectWalletType, useConnectStore } from '../../store/useConnectStore';
import { userStore } from '../../store/useMarathonStore';

import { walletAdapterMap } from '../../utils/walletAdapter';
import useGetMarathon from '../../utils/marathonActor';

//import CreateWalletModal from '../../components/CreateModal';
interface ExtendedStakingPoolInfo extends StakingPoolInfo {
    tvl?: any;
}
export default function Market() {
    const toast = useToast();
    const { allStakeRecords } = usePoolrecordStore()
    const { get_stake_pool, get_allstake_records, stake_balance, get_btc_fee } = useGetStakePool();
    const { get_balance, get_wallets, onRefresh_balance } = useGetWalletPool();

    const { actor: walletBackend } = useWalletBackend();
    const { actor: osBackend } = useOsBackend();
    const { actor: pointBackend } = usePointBackend();
    const { actor: stakeBackend } = useSatkePoolBackend();
    const { identity, login } = useInternetIdentity();
    const { identity: siwb_identity, identityAddress, clear: siwb_clear } = useSiwbIdentity();


    const [isLogin, setIslogin] = useState<boolean>(false)

    const [isLoading, setIsLoading] = useState<boolean>(false)
    const [isOsInited, setIsOsInited] = useState<boolean>(false)
    const [isPointInited, setIsPointInited] = useState<boolean>(false)
    const [isWalletInited, setIsWalletInited] = useState<boolean>(false)

    // wallet Info
    const [firstGetbalance, setFirstGetbalance] = useState<boolean>(false)
    const [balanceError, setBalanceError] = useState<string>("");
    const [walletName, setWalletName] = useState<string>("");
    const { walletList, walletSelect, wallet, setWallet, balance, totalBalance, currentWallet, setBalance, setWalletList, setWalletSelect, setCurrentWallet } = WalletStore(state => ({
        walletList: state.walletList,
        walletSelect: state.walletSelect,
        balance: state.balance,
        totalBalance: state.totalBalance,
        currentWallet: state.currentWallet,
        wallet: state.wallet,
        setWallet: state.setWallet,
        setBalance: state.setBalance,
        setWalletList: state.setWalletList,
        setWalletSelect: state.setWalletSelect,
        setCurrentWallet: state.setCurrentWallet
    }));
    const { userStake } = userStore();

    const [initialLoadDoneWallet, setInitialLoadDoneWallet] = useState(false);

    const { isOpen: isOpneWalletList, onOpen: onOpenWalletList, onClose: onCloseWalletList } = useDisclosure()

    //create wallet
    const { isOpen: isCreateOpen, onOpen: onCreateOpen, onClose: onCreateClose } = useDisclosure();


    // stake balance Info
    const [btcUnit, setBtcUnit] = useState('btc');
    const [stakeBalance, setStakeBalance] = useState<number>(0)
    const btcunity = 100000000;
    const [btc, setBtc] = useState(100000000);
    const [btcFee, setBtcFee] = useState<number>(0)
    const { get_user_stakebtc } = useGetMarathon();


    // stakepool Info
    const { stakepoolCanister, setStakepoolCanister } = StakepoolStore()
    const stakeList = useStakeListStore((state) => state.stakeList);
    const setStakeList = useStakeListStore((state) => state.setStakeList);
    const [isFirstCall, setIsFirstCall] = useState(true);
    const [isfirstInit, setIsfirstInit] = useState(true);
    const [initialLoadDoneOs, setInitialLoadDoneOs] = useState(false);
    //const [poolDetail, setPoolDetail] = useState<boolean>(false) //view pool Detail
    //const [currentPool, setCurrentPool] = useState<ExtendedStakingPoolInfo>()
    const { currentPool, setCurrentPool, poolDetail, setPoolDetail } = useStakeListStore();

    const [currentAccount] = useConnectStore((state) => [state.currentAccount])
    const [thirdPartyWalletBalance, setThirdPartyWalletBalance] = useState<number>(0)
    //fisrt stake record
    const [firstGetpool, setFirstGetpool] = useState<boolean>(false)
    const [originalStakeList, setOriginalStakeList] = useState([]);
    useEffect(() => {
        if (currentAccount && currentAccount.type !== ConnectWalletType.INTERNET_IDENTITY) {
            setThirdPartyWalletBalance(satoshisToBTC(currentAccount.balance || 0))
        }
    }, [currentAccount])

    useEffect(() => {
        //if (stakeList.length === 0) {
        //    get_stake_pool(osBackend)
        //get_wallet_count()
        //}
        setFirstGetbalance(false)
        if (identity) {
            setIslogin(true)
        }

        if (!osBackend) {
            setIsOsInited(false)
        } else {
            setIsOsInited(true)
        }
    }, [])

    useEffect(() => {
        if (pointBackend) {
            setIsPointInited(true)
        }

    }, [pointBackend])

    useEffect(() => {
        if (osBackend && stakeList.length === 0) {
            //get_wallets()
            get_stake_pool(osBackend)

            //get_wallet_count()
            //setInitialLoadDoneOs(true)

        }
    }, [osBackend])

    useEffect(() => {
        if (identity !== null && identity !== undefined || siwb_identity !== null && siwb_identity !== undefined) {
            get_user_stakebtc(pointBackend)
        }

    }, [pointBackend, identity, siwb_identity])

    useEffect(() => {
        if (!firstGetpool && stakeList.length !== 0) {
            setOriginalStakeList(stakeList)
            setFirstGetpool(true)
        }
    }, [stakeList])

    useEffect(() => {
        // debugger
        if (osBackend) {
            setIsOsInited(true)
        }
        if (identity && osBackend && !wallet) {
            get_wallets(osBackend)
        }
        // if (identity) {
        //     if (walletList && walletList.length === 0 && currentAccount.type === 'INTERNET_IDENTITY') {
        //         onCreateOpen()
        //     } else {
        //         onCreateClose()
        //     }
        // }
    }, [osBackend, identity]);

    useEffect(() => {
        if (walletBackend) {
            setIsWalletInited(true);
            get_fee()
        }
    }, [walletBackend]);

    useEffect(() => {
        if (walletBackend && walletSelect && walletSelect.bitcoin_address && !firstGetbalance) {
            get_balance(walletBackend, walletSelect.bitcoin_address)
            setFirstGetbalance(true)
        }
    }, [walletBackend, walletSelect]);

    /*
    useEffect(() => {
        if (!initialLoadDoneWallet && walletList.length > 0) {
            setWallet(walletList[0].bitcoin_address)
            // Trigger onChangeWallet with the first wallet's value
            // Find selected wallet item
            if (walletList[0].bitcoin_address) {
                const walletselectedItem = walletList.find(item => item.bitcoin_address === walletList[0].bitcoin_address);
                // If the selected item is found and it is not in the walletSelect array, it is added to the array
                if (walletselectedItem) {
                    setWalletSelect(walletselectedItem);
                }

                updateWalletData(walletList[0].bitcoin_address);
            }
            setInitialLoadDoneWallet(true);
        }
    }, [walletList, initialLoadDoneWallet]);
    */
    //Initialisation of all states
    async function get_fee() {
        const fee = await get_btc_fee(walletBackend)
        setBtcFee(fee)
    }
    function resetallInit() {
        console.log('reset')
    }

    //How to switch ii Wallet
    async function onChangeWallet(event: React.ChangeEvent<HTMLSelectElement>) {
        setWallet(event.target.value)
        setStakeBalance(0)

        // Find selected wallet item
        const selectedItem = walletList.find(item => item.bitcoin_address === event.target.value);
        // If the selected item is found and it is not in the walletSelect array, it is added to the array
        if (selectedItem) {
            setWalletSelect(selectedItem);
        }

        const selectOption = event.target.selectedOptions[0]
        if (selectOption.dataset.id) {
            setCurrentWallet(selectOption.dataset.id)
        }
        await updateWalletData(event.target.value);
    }
    //How to get ii wallet balance
    async function updateWalletData(addr: string) {
        if (!walletBackend) { return };
        if (!checkIdentityExpiration(identity)) {
            resetallInit()
            return
        }
        // Initialising the loading state
        setIsLoading(true);
        // Initialising state values
        if (!addr || addr.length < 1) {
            setBalance(0);
            setIsLoading(false);
            return;
        }
        try {

            // Parallel calls to get_balance and get_stake_records
            const [balanceResult] = await Promise.all([
                walletBackend.balance(addr)
            ]);

            // Processing of balance results
            if ('Err' in balanceResult) {
                toast({
                    title: 'Balance',
                    description: "get balance error",
                    status: 'error',
                    position: 'bottom-right',
                    duration: 9000,
                    isClosable: true,
                });
            } else {
                const b: bigint = balanceResult.Ok;
                setBalance(Number(b) / btcunity);
            }

        } catch (error) {
            console.error('Error updating wallet data:', error);
            toast({
                title: 'Error',
                description: "Error updating wallet data",
                status: 'error',
                position: 'bottom-right',
                duration: 9000,
                isClosable: true,
            });
        } finally {
            setIsLoading(false);
        }
    }
    //Creating a new wallet method
    function onCreateWallet() {
        onCreateClose()
        if (!osBackend || !identity) {
            return
        }
        if (!checkIdentityExpiration(identity)) {
            resetallInit()
            return
        }
        setIsLoading(true)
        osBackend.create_wallet_canister(walletName).then(
            (v) => {
                get_wallets(osBackend)
                //get_wallet_count()
                setIsLoading(false)
            }, (e) => {
                setIsLoading(false)
            }
        )

    }

    //Click to enter pool details
    const onCardClick = async (stake) => {
        setStakepoolCanister(stake.staking_pool_canister.toText())
        setCurrentPool(stake)
        await get_allstake_records(stake.staking_pool_canister.toText())
        setPoolDetail(true)
        // const currentTimeStamp = BigInt(Date.now()) * 1000000n; // Converts the current timestamp from milliseconds to nanoseconds
        // const startTimeStamp = BigInt(stake.start_time);
        // const endTimeStamp = BigInt(stake.end_time);
        // if (currentTimeStamp >= startTimeStamp && currentTimeStamp <= endTimeStamp) {
        //     setStakepoolCanister(stake.staking_pool_canister.toText())
        //     setCurrentPool(stake)
        //     await get_allstake_records(stake.staking_pool_canister.toText())
        //     setPoolDetail(true)
        // } else {
        //     // If it is not within the time range, you can choose to display an error message or do nothing
        //     console.log('The current time is not within the specified time range and the operation cannot be performed.');
        //     // Or you can add appropriate error handling logic
        // }
    }

    /*--- change btc unit ---*/
    function onChangebtcunit(event: React.ChangeEvent<HTMLSelectElement>) {
        const newUnit = event.target.value;
        setBtcUnit(newUnit);
        if (event.target.value === 'btc') {
            setBtc(100000000)
        }
        else if (event.target.value === 'satoshi') {
            setBtc(1)
        }
    }
    //Judgement of the number of pledged tokens
    function handleChangeStake(event: React.ChangeEvent<HTMLInputElement>) {
        const value = parseFloat(event.target.value)
        const currentBalance = isThirdPartyWallet(currentAccount) ? thirdPartyWalletBalance : balance;
        if (value >= currentBalance) {
            setBalanceError("*BTC balance is insufficient ")
        } else {
            setStakeBalance(parseFloat(event.target.value))
            setBalanceError("")
        }
    }
    //Selection of maximum number of pledges
    function onMaxClick() {
        setStakeBalance(balance)
    }
    //stake
    function onStake() {
        if (currentAccount.type === ConnectWalletType.INTERNET_IDENTITY) {
            onIiStake()
        } else {
            // third party wallet
            onThirdPartyWalletStake()
        }
    }

    const onThirdPartyWalletStake = async () => {
        const amountInSatoshis = Math.round(stakeBalance * btc);
        const tx = await walletAdapterMap[currentAccount.type].sendBitcoin(currentPool.bitcoin_address, amountInSatoshis)

        toast({
            title: 'Stake',
            status: 'success',
            position: "bottom-right",
            duration: 9000,
            isClosable: true,
            render: () => (
                <Flex
                    direction='row'
                    color='black'
                    p={3}
                    bg='#FFFFFF'
                    border='2px solid #FF8800'
                    borderRadius='xl'

                >
                    <Flex pt='2' pl='2' mr='2'>
                        <BsCheckCircleFill color='#30C06C' />
                    </Flex>
                    <Flex direction='column'>
                        <Text>Successfully staked</Text>
                        <Text>
                            {"TXID:" + truncateMiddle(tx, 5, 5)}
                            <Button
                                variant='ghost'
                                onClick={() => window.open(`https://mempool.space/testnet/tx/${tx}`, '_blank')}
                            >
                                <BsBoxArrowUpRight size='12px' />
                            </Button>
                        </Text>
                    </Flex>
                </Flex>

            )
        });

        // toast({
        //     title: '🚧 This feature is under construction...',
        //     status: 'warning',
        //     position: 'top',
        //     duration: 3000,
        //     isClosable: true,
        // });
    }

    const onIiStake = () => {
        if (!walletBackend) return;
        const amountInSatoshis = Math.round(stakeBalance * btc);
        const stakeRequest: StakingRequest = {
            staking_address: currentPool.bitcoin_address,
            memo: [],
            stake_type: [{ OSWallet: null }], // Optional attribute, explicitly supplied undefined
            staking_canister: currentPool.staking_pool_canister,
            fund_management: [], // Optional attribute, explicitly supplied undefined
            amount: BigInt(amountInSatoshis), // Make sure it's an integer.
        };
        stake_balance_ago()
        //stake_balance(walletBackend, stakeRequest)
    }
    function stake_balance_ago() {
        if (!walletBackend) return;
        setIsLoading(true);

        const amountInSatoshis = Math.round(stakeBalance * btc);
        const stakeRequest: StakingRequest = {
            staking_address: currentPool.bitcoin_address,
            memo: [],
            stake_type: [{ OSWallet: null }], // Optional attribute, explicitly supplied undefined
            staking_canister: currentPool.staking_pool_canister,
            fund_management: [], // Optional attribute, explicitly supplied undefined
            amount: BigInt(amountInSatoshis), // Make sure it's an integer.
        };

        walletBackend.staking_to_pool_from_p2wsh_multisig22(stakeRequest).then((result) => {
            if ('Err' in result) {
                toast({
                    title: 'Stake',
                    description: "stake balance error",
                    status: 'error',
                    position: "bottom-right",
                    duration: 9000,
                    isClosable: true,
                });
            } else {
                toast({
                    title: 'Stake',
                    status: 'success',
                    position: "bottom-right",
                    duration: 9000,
                    isClosable: true,
                    render: () => (
                        <Flex
                            direction='row'
                            color='black'
                            p={3}
                            bg='#FFFFFF'
                            border='2px solid #FF8800'
                            borderRadius='xl'

                        >
                            <Flex pt='2' pl='2' mr='2'>
                                <BsCheckCircleFill color='#30C06C' />
                            </Flex>
                            <Flex direction='column'>
                                <Text>Successfully staked</Text>
                                <Text>
                                    {"TXID:" + truncateMiddle(result.Ok, 5, 5)}
                                    <Button
                                        variant='ghost'
                                        onClick={() => window.open(`https://mempool.space/testnet/tx/${result.Ok}`, '_blank')}
                                    >
                                        <BsBoxArrowUpRight size='12px' />
                                    </Button>
                                </Text>
                            </Flex>
                        </Flex>

                    )
                });
            }

            setStakeBalance(0);
            setIsLoading(false);
        });
    }


    //--------------------------------
    //Graph data (test data)
    //--------------------------------
    const data = {
        labels: ['January', 'February', 'March', 'April', 'May', 'June', 'July'],
        datasets: [
            {
                label: 'My First Dataset',
                data: [10, 10, 10, 10, 10, 10, 10],
                fill: false,
                backgroundColor: 'rgba(75,192,192,0.2)',
                borderColor: 'rgba(75,192,192,1)',
                tension: 0.4, // This value makes the line curved
            },
        ]
    };

    const options = {
        responsive: true,
        plugins: {
            legend: {
                display: false,
            },
            title: {
                display: true,
            },
        },
        scales: {
            x: {
                grid: {
                    display: false, // Disable x-axis grid lines
                },
            },
            y: {
                grid: {
                    display: true, // Enable y-axis grid lines
                },
                beginAtZero: true, // Start y-axis at zero
                ticks: {
                    stepSize: 5, // Define the step size between ticks
                    callback: function (value, index, values) {
                        return value + '%'; // Add '%' to the tick labels
                    },
                },
                title: {
                    display: false,
                    text: 'Percentage', // Add title to y-axis
                },
            },
        },
    };


    const data2 = {
        labels: ['btc'],
        datasets: [
            {
                label: '# of Votes',
                data: [100],
                backgroundColor: [
                    '#F7931A',
                    'rgba(54, 162, 235, 0.8)',
                    'rgba(255, 206, 86, 0.8)',
                    'rgba(75, 192, 192, 0.8)',
                    'rgba(153, 102, 255, 0.8)',
                    'rgba(255, 159, 64, 0.8)',
                ],
                borderColor: '#F7931A', // White border as a spacer
            },
        ],
    };

    const options2 = {
        responsive: true,
        cutout: '60%', // Adjust this value to change the thickness of the ring
        plugins: {
            legend: {
                position: 'right',
            },
            title: {
                display: true,
            },
        },
    };
    const handleCopy = (text) => {
        // copy address
        navigator.clipboard.writeText(text).then(() => {
            toast({
                title: "Address copied",
                status: "success",
                position: "bottom-right",
                duration: 2000,
                isClosable: true,
            })
        })
    }

    const [sortOption, setSortOption] = useState('default');
    const handleSort = (option) => {
        let sortedList;
        switch (option) {
            case 'option1':
                // Sort by annual_interest_rate descending
                sortedList = [...stakeList].sort((a, b) => b.annual_interest_rate - a.annual_interest_rate);
                break;
            case 'option2':
                // Sort by annual_interest_rate ascending
                sortedList = [...stakeList].sort((a, b) => a.annual_interest_rate - b.annual_interest_rate);
                break;
            default:
                // Default option, show original order
                sortedList = originalStakeList;
        }
        setStakeList(sortedList);
    };

    const test = () => {

        console.log(BigInt(Date.now()) * 1000000n)
        console.log(currentPool.end_time)
        console.log(currentPool.start_time)
    }

    return (
        <>
            <Flex
                direction='column'
                justifyContent='center'
                alignItems="center"
                bgGradient="linear(to-b, #F5F7FF 0%, #FFFFFF 100%)" // Adding gradient background
                position="relative"
            >
                <ConnectModal isOpen={isOpneWalletList} onClose={onCloseWalletList}>
                    <WalletList />
                </ConnectModal>
                <Box
                    width="100%"
                    height="300px" // Set the desired height for the background image
                    backgroundImage="./home/background-top.svg" // Replace with your image path
                    backgroundSize="cover"
                    backgroundPosition="center"
                    position="absolute" // Absolute positioning
                    top="0"
                    left="0"
                    zIndex="0" // Ensure the background image is behind other content
                />
                {isLoading &&
                    <Flex
                        zIndex={999999}
                        height="100%"
                        bg="rgba(0, 0, 0, 0.6)" // 使用 RGBA 调整背景透明度
                        width="100%"
                        position="fixed"
                        align="center"
                        justify="center"
                        top={0}
                        left={0}
                        backdropFilter="blur(5px)" // 添加背景模糊效果
                    >
                        <Spinner
                            color="purple.500"
                            size="xl"
                            speed="0.65s"
                            thickness="4px" // 调整厚度
                        />
                    </Flex>
                }

                {/*
                <Flex mt={6} direction='row'>
                    <Button onClick={test}>test</Button>
                </Flex>
                */}

                {!poolDetail ? (
                    <Flex width='100%' justifyContent='center' direction='column'>
                        <Flex justifyContent='center' pt='10'>
                            <Flex width='78%' maxWidth='1120px' direction='column'>
                                <Text
                                    fontSize={{ md: '23px', lg: '30px', xl: '40px', '2xl': '40px', '3xl': '40px' }}
                                    fontWeight='bold'
                                >Discover Pools</Text>
                                <Text fontSize={{ md: '15px', lg: '20px', xl: '20px', '2xl': '20px', '3xl': '20px' }}>Never Idle Your Assets</Text>
                            </Flex>
                        </Flex>
                        <Flex justifyContent='center'>
                            <Flex width='78%' maxWidth='1120px' justifyContent='flex-end'>
                                <Flex direction='row' alignItems='center'>
                                    {/*
                                        <Checkbox defaultChecked><Text>MyPools</Text></Checkbox>
                                    */}
                                    <Select
                                        variant='filled'
                                        placeholder='Default'
                                        ml={2}
                                        onChange={(e) => {
                                            const option = e.target.value;
                                            setSortOption(option);
                                            handleSort(option);
                                        }}
                                        value={sortOption}
                                    >
                                        <option value='option1'>APY: High to Low</option>
                                        <option value='option2'>APY: Low to High</option>
                                    </Select>
                                </Flex>
                            </Flex>
                        </Flex>
                        <Flex justifyContent='center'>
                            <Grid mt={6} width="78%" maxWidth='1120px' templateColumns="repeat(3, 1fr)" gap={6} justifyContent='center'>
                                {stakeList && stakeList.length > 0 ? (
                                    stakeList.map((stake, index) => (
                                        <Card
                                            key={index}
                                            borderWidth='1px'
                                            borderColor='gray.200'
                                            borderRadius='lg'
                                            overflow='hidden'
                                            style={{ cursor: 'pointer' }}
                                            onClick={() => onCardClick(stake)}
                                        >
                                            <Card
                                                key={index}
                                                p={{ md: '0', lg: '0', xl: '2', '2xl': '2', '3xl': '2' }}
                                                borderWidth='1px'
                                                borderColor='gray.200'
                                                borderRadius='lg'
                                                overflow='hidden'
                                                background="linear-gradient(to right, #FFFFFF 0%, #E0E6FF 100%)" // 线性渐变背景
                                            >
                                                <CardHeader>
                                                    <Flex justifyContent="space-between" alignItems="center" mb='3'>
                                                        <Flex direction="column">
                                                            <Flex fontSize={{ md: '14px', lg: '14px', xl: '14px', '2xl': '14px', '3xl': '14px' }} alignContent='center'>
                                                                <Text>Top {index + 1} </Text>
                                                                <Tag
                                                                    size={{ md: 'sm', lg: 'sm', xl: 'sm', '2xl': 'sm', '3xl': 'sm' }}
                                                                    variant='solid'
                                                                    colorScheme='teal'
                                                                    borderRadius='3xl'
                                                                    ml='3'
                                                                >{Object.keys(stake.status)[0]}</Tag></Flex>
                                                            <Text
                                                                fontSize={{ md: '14px', lg: '14px', xl: '14px', '2xl': '14px', '3xl': '14px' }}
                                                                mt='2'
                                                                fontWeight='700'
                                                            >{stake.name}</Text>
                                                            <Flex alignItems="center">
                                                                <Box
                                                                    color='#ED8936'
                                                                    as="span"
                                                                    fontWeight="bold"
                                                                    fontSize={{ md: '20px', lg: '22px', xl: '24px', '2xl': '24px', '3xl': '24px' }}
                                                                >{Number(stake.annual_interest_rate).toFixed(2)}%</Box>
                                                                <Text fontSize={{ md: '12px', lg: '12px', xl: '12px', '2xl': '12px', '3xl': '12px' }} ml='2'>APY</Text>
                                                            </Flex>
                                                        </Flex>
                                                        <Flex>
                                                            <Image width={{ md: '40px', lg: '40px', xl: '40px', '2xl': '40px', '3xl': '40px' }} src="./home/bitcoin-btc-logo.svg"></Image>
                                                        </Flex>
                                                    </Flex>
                                                    <Flex
                                                        fontSize={{ md: '12px', lg: '12px', xl: '12px', '2xl': '12px', '3xl': '12px' }}
                                                        justifyContent="space-between"
                                                        pr='2'
                                                    >
                                                        <Text>Net Asset Value</Text>
                                                        <Flex>
                                                            <Text fontWeight='bold' mr='1'>{Number(stake.tvl) * 1.0 / btcunity}</Text>btc
                                                        </Flex>
                                                    </Flex>
                                                    <Flex
                                                        fontSize={{ md: '12px', lg: '12px', xl: '12px', '2xl': '12px', '3xl': '12px' }}
                                                        justifyContent="space-between"
                                                        pr='2'
                                                    >
                                                        <Text>TVL</Text>
                                                        <Flex>
                                                            <Text fontWeight='bold' mr='1'>{Number(stake.tvl) * 1.0 / btcunity}</Text>btc
                                                        </Flex>
                                                    </Flex>
                                                </CardHeader>
                                            </Card>
                                            <Box
                                                fontSize={{ md: '12px', lg: '12px', xl: '12px', '2xl': '12px', '3xl': '12px' }}
                                                ml={{ md: '5', lg: '5', xl: '7', '2xl': '8', '3xl': '8' }}
                                            ><Text color='#636363'>No Position</Text></Box>
                                        </Card>
                                    ))
                                ) : (
                                    <Text>No staking pools available.</Text>
                                )}
                            </Grid>
                        </Flex>
                    </Flex>
                ) : (
                    <Flex
                        maxWidth='1120px'
                        direction='column'
                        width='78%'
                    >
                        <Box pt='3'>
                            <Button
                                fontSize='16px'
                                color='#58667E'
                                variant='ghost'
                                onClick={() => setPoolDetail(false)}
                            >{"<"} Back</Button></Box>
                        {currentPool ? (
                            <>
                                <Flex>
                                    <Flex direction='column' width='100%'>
                                        <Flex justifyContent="space-between" width='100%'>
                                            <Flex>
                                                <Text
                                                    fontSize={{ md: '24px', lg: '24px', xl: '24px', '2xl': '24px', '3xl': '24px' }}
                                                    fontWeight='bold'
                                                    mb={{ md: '6', lg: '6', xl: '8', '2xl': '10', '3xl': '12' }}
                                                >{currentPool.name}</Text>
                                            </Flex>
                                            <Flex alignItems="center">
                                                <Flex
                                                    direction='column'
                                                    fontSize={{ md: '12px', lg: '12px', xl: '12px', '2xl': '12px', '3xl': '12px' }}
                                                    mr='3'
                                                >
                                                    <Text>My Total Position</Text>
                                                    <Text>(btc)</Text>
                                                </Flex>
                                                <Flex
                                                    fontSize={{ md: '23px', lg: '30px', xl: '40px', '2xl': '40px', '3xl': '40px' }}
                                                    fontWeight='bold'
                                                >
                                                    {userStake?.stake_amount
                                                        ? `${(Number(userStake.stake_amount) / btcunity).toFixed(6)} btc`
                                                        : '0 btc'}
                                                </Flex>
                                            </Flex>
                                        </Flex>
                                        <Flex>
                                            <Flex direction='column' mr='16'>
                                                <Text fontSize={{ md: '12px', lg: '12px', xl: '12px', '2xl': '12px', '3xl': '12px' }}>Net Asset Value</Text>
                                                <Box
                                                    as="span"
                                                    fontWeight="bold"
                                                    color='#4C506B'
                                                    fontSize={{ md: '24px', lg: '24px', xl: '24px', '2xl': '24px', '3xl': '24px' }}
                                                >{Number(currentPool.tvl) * 1.0 / btcunity} btc</Box>
                                            </Flex>
                                            <Flex direction='column' mr='16'>
                                                <Text fontSize={{ md: '12px', lg: '12px', xl: '12px', '2xl': '12px', '3xl': '12px' }}>APY</Text>
                                                <Box
                                                    as="span"
                                                    fontWeight="bold"
                                                    color='#4C506B'
                                                    fontSize={{ md: '24px', lg: '24px', xl: '24px', '2xl': '24px', '3xl': '24px' }}
                                                >{Number(currentPool.annual_interest_rate).toFixed(2)}%</Box>
                                            </Flex>
                                            <Flex direction='column'>
                                                <Text fontSize={{ md: '12px', lg: '12px', xl: '12px', '2xl': '12px', '3xl': '12px' }}>TVL</Text>
                                                <Box
                                                    as="span"
                                                    fontWeight="bold"
                                                    color='#4C506B'
                                                    fontSize={{ md: '24px', lg: '24px', xl: '24px', '2xl': '24px', '3xl': '24px' }}
                                                >{Number(currentPool.tvl) * 1.0 / btcunity} btc</Box>
                                            </Flex>
                                        </Flex>
                                    </Flex>
                                </Flex>
                                <Flex mt={{ md: '8', lg: '10', xl: '12', '2xl': '16', '3xl': '6' }} justifyContent='space-between'>
                                    <Flex
                                        width='53%'
                                        px={10}
                                        py={6}
                                        bgColor='white'
                                        mt={2}
                                        justifyContent="center"
                                        border='1px'
                                        borderColor='gray.200'
                                        borderRadius='2xl'
                                        direction='column'
                                        zIndex={1}
                                    >
                                        <Flex alignItems='center'>
                                            <Text
                                                fontWeight='bold'
                                                fontSize={{ md: '12px', lg: '15px', xl: '24px', '2xl': '24px', '3xl': '24px' }}>Performance</Text>
                                        </Flex>
                                        <Flex justifyContent="space-between" alignItems='center'>
                                            <Flex alignItems='center' color='#4C506B'>
                                                <Text
                                                    mr='3'
                                                    fontSize={{ md: '22px', lg: '30px', xl: '32px', '2xl': '32px', '3xl': '32px' }}
                                                    fontWeight='bold'
                                                >{Number(currentPool.annual_interest_rate).toFixed(2)}%</Text>
                                                <Text fontSize={{ md: '12px', lg: '12px', xl: '12px', '2xl': '12px', '3xl': '12px' }}>since {formatDate(currentPool.start_time)}</Text>
                                            </Flex>
                                        </Flex>
                                        <Flex width='100%' display='flex' justifyContent="center">
                                            <CurvedLineChart data={data} options={options} />
                                        </Flex>
                                    </Flex>
                                    <Flex
                                        zIndex={1}
                                        px={{ md: '6', lg: '6', xl: '8', '2xl': '10', '3xl': '10' }}
                                        py={{ md: '5', lg: '5', xl: '6', '2xl': '6', '3xl': '6' }}
                                        width='45%'
                                        bgColor='white'
                                        mt={2}
                                        justifyContent="center"
                                        border='1px'
                                        borderColor='gray.200'
                                        borderRadius='2xl'
                                    >
                                        <VStack align='left' width='100%'>
                                            <HStack align='end'>
                                                <Text
                                                    fontSize={{ md: '24px', lg: '24px', xl: '24px', '2xl': '24px', '3xl': '24px' }}
                                                    fontWeight='bold'
                                                    color='#4C506B'
                                                    mb={{ md: '13', lg: '13', xl: '15', '2': '201', '3xl': '251' }}
                                                >Purchase</Text>
                                            </HStack>
                                            {/*
                                            {
                                                walletList.length > 0 && (
                                                    <Flex
                                                        justifyContent='space-between'
                                                    >
                                                        <Text
                                                            fontSize={{ md: '12px', lg: '12px', xl: '12px', '2xl': '14px', '3xl': '14px' }}
                                                            fontWeight='500'
                                                        >Wallet</Text>
                                                    </Flex>
                                                ) || false
                                            }
                                            <HStack bg="gray.200" borderRadius="lg">
                                                {
                                                    walletList.length > 0 && (
                                                        <Select onChange={onChangeWallet} width="100%">
                                                            {
                                                                walletList.map((item, index) => (
                                                                    <option key={index} value={item.bitcoin_address} data-id={item.wallet_canister.toText()}>{item.name} ({truncateMiddle(item.bitcoin_address, 5, 5)})</option>
                                                                ))
                                                            }
                                                        </Select>
                                                    )
                                                }
                                                {
                                                    walletList.length === 0 || false
                                                }
                                            </HStack>
                                            */}
                                            {wallet.length > 0 &&
                                                <Flex alignItems='center'>
                                                    <Text fontSize='sm'>
                                                        address: {truncateMiddle(walletSelect.bitcoin_address, 5, 5)}
                                                    </Text>
                                                    <Flex alignItems='center'>
                                                        <Button variant='ghost' onClick={() => handleCopy(walletSelect.bitcoin_address)}>
                                                            <BsCopy
                                                                size='15px'
                                                            />
                                                        </Button>
                                                    </Flex>
                                                </Flex>
                                            }
                                            <Flex
                                                mt='2'
                                                alignItems='center'
                                                justifyContent='space-between'
                                            >
                                                <Text
                                                    fontWeight='500'
                                                    fontSize={{ md: '12px', lg: '12px', xl: '12px', '2xl': '14px', '3xl': '14px' }}
                                                >Amount</Text>
                                                <Flex alignItems='center'>
                                                    <Text fontSize={{ md: '12px', lg: '12px', xl: '12px', '2xl': '12px', '3xl': '12px' }}>btc Balance:{isThirdPartyWallet(currentAccount) ? thirdPartyWalletBalance : balance}</Text>
                                                    <IconButton
                                                        icon={<BsArrowClockwise />}
                                                        variant="ghost"
                                                        aria-label="Refresh"
                                                        onClick={onRefresh_balance}
                                                        cursor="pointer"
                                                        _hover={{ bg: "gray.100" }}
                                                    />
                                                </Flex>
                                            </Flex>
                                            <HStack
                                                bg="gray.200"
                                                p={1}
                                                borderRadius="xl"
                                                height={{ md: '2.3rem', lg: '2.7rem', xl: '3rem', '2xl': '44px', '3xl': '44px' }}
                                                mb='2'
                                            >
                                                <InputGroup>
                                                    <InputLeftElement
                                                        pointerEvents="none"
                                                    >
                                                        <Image src='./favicon.png' boxSize="1.2rem" />
                                                    </InputLeftElement>
                                                    <Input type="number" value={stakeBalance} border="none" placeholder='0.0' isDisabled={!currentAccount} onChange={handleChangeStake}></Input >

                                                    <InputRightElement width="auto" display="flex" alignItems="center">
                                                        <Select value={btcUnit} onChange={onChangebtcunit} width="auto" mr={2}>
                                                            <option value="btc">btc</option>
                                                            <option value="satoshi">satoshi</option>
                                                        </Select>
                                                        <Button color="orange.300" isDisabled={!isLogin} p={2} fontSize="0.8rem" onClick={onMaxClick}>MAX</Button>
                                                    </InputRightElement>
                                                </InputGroup>
                                            </HStack>
                                            {/*
                                            <div>Receive</div>
                                            <HStack bg="gray.200" p={1} borderRadius="lg">
                                                <InputGroup>
                                                    <InputLeftElement
                                                        pointerEvents="none"
                                                    >
                                                        <Image src='./favicon.png' boxSize="1.2rem" />
                                                    </InputLeftElement>
                                                    <Input type="number" value={stakeBalance} border="none" placeholder='0.0' isDisabled={true} onChange={handleChangeStake}></Input >

                                                </InputGroup>
                                            </HStack>
                                            */}
                                            <Text fontSize="0.8rem" color='red'><span>{balanceError}</span></Text>
                                            <HStack
                                                borderRadius="lg"
                                                mt='2'
                                            >
                                                <Flex
                                                    justifyContent="space-between"
                                                    width="100%"
                                                    fontSize={{ md: '12px', lg: '12px', xl: '12px', '2xl': '12px', '3xl': '12px' }}
                                                >
                                                    <Text>Entrance Fee</Text>
                                                    <Text>{btcFee * btcunity} sats</Text>
                                                </Flex>
                                            </HStack>

                                            <Flex width='100%' direction='column' align="center" pt={2}>
                                                {currentAccount && (
                                                    <>

                                                        <Button
                                                            width="100%"
                                                            color="white"
                                                            bgColor="orange.400"
                                                            _hover={{ bg: "orange.200", borderColor: "orange.400" }}
                                                            height={{ md: '2.3rem', lg: '2.7rem', xl: '3rem', '2xl': '44px', '3xl': '44px' }}
                                                            fontSize={{ md: '20px', lg: '22px', xl: '25px', '2xl': '16px', '3xl': '16px' }}
                                                            //isDisabled={stakeBalance <= 0 || !isOsInited} 
                                                            isDisabled={
                                                                stakeBalance <= 0 ||
                                                                !isOsInited ||
                                                                !(BigInt(Date.now()) * 1000000n >= BigInt(currentPool.start_time) &&
                                                                    BigInt(Date.now()) * 1000000n <= BigInt(currentPool.end_time))
                                                            }
                                                            onClick={onStake}
                                                        >Stake</Button>

                                                    </>
                                                )}
                                                {!currentAccount && (
                                                    <Button
                                                        height={{ md: '2.3rem', lg: '2.7rem', xl: '3rem', '2xl': '44px', '3xl': '44px' }}
                                                        //height="2.5rem"
                                                        fontSize={{ md: '20px', lg: '22px', xl: '25px', '2xl': '16px', '3xl': '16px' }}
                                                        borderRadius='xl'
                                                        width="100%"
                                                        color="white"
                                                        bgColor="orange.400"
                                                        _hover={{ bg: "orange.200", borderColor: "orange.400" }}
                                                        onClick={onOpenWalletList}
                                                    >Login/Register</Button>
                                                )}
                                            </Flex>
                                        </VStack>
                                    </Flex>
                                </Flex>
                                <Flex
                                    mt={5}
                                    mb={5}
                                    p={10}
                                    bgColor='white'
                                    direction='column'
                                    border='1px'
                                    borderColor='gray.200'
                                    borderRadius='2xl'
                                >
                                    <Text fontSize='24px' fontWeight='bold' mb='2'>Key Statistics</Text>
                                    <Grid
                                        fontSize={{ md: '12px', lg: '12px', xl: '12px', '2xl': '12px', '3xl': '12px' }}
                                        mb={5}
                                        bgColor="white"
                                        gridTemplateColumns="repeat(6, 1fr)" // Display 6 items per line
                                        gap={5} // Setting the spacing between items
                                    >
                                        <Flex direction='column' flex='1'><Flex color='gray.400'>Fund Manager</Flex><div>BifiPal</div></Flex>
                                        <Flex direction='column' flex='1'><Flex color='gray.400'>Genesis Data</Flex><div>{formatDate(currentPool.created_at)}</div></Flex>
                                        <Flex direction='column' flex='1'><Flex color='gray.400'>Maturity Data</Flex><div>{formatDate(currentPool.end_time)}</div></Flex>
                                        <Flex direction='column' flex='1'><Flex color='gray.400'>Est APY</Flex><div>{currentPool.annual_interest_rate}%</div></Flex>
                                        <Flex direction='column' flex='1'>
                                            <Flex color='gray.400'>Deposit Address</Flex>
                                            <div>{truncateMiddle(currentPool.bitcoin_address.toString(), 5, 5)}
                                                <Button variant='ghost' onClick={() => handleCopy(currentPool.bitcoin_address.toString())}>
                                                    <BsCopy size='12px' />
                                                </Button>
                                            </div>
                                        </Flex>
                                        <Flex direction='column' flex='1'><Flex color='gray.400'>Sale Start Time(UTC)</Flex><div>{formatDate(currentPool.start_time)}</div></Flex>
                                        <Flex direction='column' flex='1'><Flex color='gray.400'>Redemption Freq</Flex><div>{Number(currentPool.duration_in_day)} days</div></Flex>
                                        <Flex direction='column' flex='1'><Flex color='gray.400'>Purchase Limit</Flex><div>No limit</div></Flex>
                                    </Grid>
                                </Flex>
                                <Flex
                                    pt='10'
                                    pl='10'
                                    mt={5}
                                    mb={5}
                                    bgColor='white'
                                    direction='row'
                                    border='1px'
                                    borderColor='gray.200'
                                    borderRadius='2xl'
                                >
                                    <Flex width='55%' direction='column' pr='8'>
                                        <Text fontSize='24px' fontWeight='bold'>Description</Text>
                                        <Text fontSize='16px'>{currentPool.description}</Text>
                                    </Flex>
                                    <Flex direction='column'>
                                        <Text fontSize='24px' fontWeight='bold'>Asset Managed</Text>
                                        <AssetChart data={data2} options={options2} />
                                    </Flex>
                                </Flex>
                                <Flex
                                    mt={5}
                                    mb={5}
                                    p={10}
                                    bgColor='white'
                                    direction='column'
                                    border='1px'
                                    borderColor='gray.200'
                                    borderRadius='2xl'
                                >
                                    <Table variant='simple' size='md'>
                                        <Thead>
                                            <Tr fontSize={{ md: '12px', lg: '12px', xl: '12px', '2xl': '12px', '3xl': '12px' }}>
                                                <Th>#</Th>
                                                <Th textTransform="none">Date</Th>
                                                <Th textTransform="none">Wallet</Th>
                                                <Th textTransform="none">Amount(btc)</Th>
                                                <Th textTransform="none">TXID</Th>
                                                <Th textTransform="none">Status</Th>
                                            </Tr>
                                        </Thead>
                                        <Tbody>
                                            {allStakeRecords && allStakeRecords.length > 0 ? (
                                                allStakeRecords.map((stake, index) => {
                                                    return (
                                                        <Tr key={index} fontSize={{ md: '14px', lg: '14px', xl: '14px', '2xl': '14px', '3xl': '14px' }}>
                                                            <Td>{index + 1}</Td>
                                                            <Td>
                                                                {formatDateminute(stake.sent_time)}
                                                            </Td>
                                                            <Td>
                                                                <div>{truncateMiddle(stake.sender_address.toString(), 5, 5)}
                                                                    <Button variant='ghost' onClick={() => handleCopy(stake.sender_address.toString())}>
                                                                        <BsCopy size='12px' />
                                                                    </Button>
                                                                </div>
                                                            </Td>
                                                            <Td>{Number(stake.sent_amount) / btcunity}</Td>
                                                            <Td>
                                                                {truncateMiddle(stake.txid, 5, 5)}
                                                                <Button
                                                                    variant='ghost'
                                                                    onClick={() => window.open(`https://mempool.space/testnet/tx/${stake.txid}`, '_blank')}
                                                                >
                                                                    <BsBoxArrowUpRight size='12px' />
                                                                </Button>
                                                                {/*
                                                                <Button
                                                                    variant='ghost'
                                                                    color='#3861FB'
                                                                    fontSize='14px'
                                                                    onClick={() => onCardClick(stake)}
                                                                >Buy</Button>
                                                                */}
                                                            </Td>
                                                            <Td>
                                                                {Object.keys(stake.status)[0]}
                                                            </Td>
                                                        </Tr>

                                                    );
                                                })
                                            ) : (
                                                <Flex width='250%' justifyContent='center' alignItems='center'>
                                                    <Text>No staking Records.</Text>
                                                </Flex>
                                            )}
                                        </Tbody>
                                    </Table>
                                </Flex>
                                {/* Add more fields here based on your requirements */}
                            </>
                        ) : (
                            <Text>No current pool selected.</Text>
                        )}
                    </Flex>
                )}
                <Modal
                    isOpen={isCreateOpen}
                    onClose={onCreateClose}
                    isCentered
                    closeOnOverlayClick={false}
                >
                    <ModalOverlay />
                    <ModalContent pt='10' pb='8' borderRadius="3xl">
                        <ModalHeader></ModalHeader>
                        {/*<ModalCloseButton />*/}
                        <Image src='./home/createWallet.svg' />
                        <ModalBody pb={6}>
                            <Flex justifyContent="center">
                                <FormControl width='70%'>
                                    <FormLabel textAlign="center">You need to create a wallet to continue using our products</FormLabel>
                                    <Input placeholder="wallet name" onChange={event => setWalletName(event.currentTarget.value)} />
                                </FormControl>
                            </Flex>
                        </ModalBody>

                        <ModalFooter>
                            <Flex justifyContent="center" width='100%'>
                                <Button
                                    width='60%'
                                    bgColor="#000000"
                                    color="white"
                                    _hover={{ bg: "purple.300", borderColor: "purple.500" }}
                                    onClick={onCreateWallet}>
                                    Create Now
                                </Button>
                            </Flex>
                        </ModalFooter>
                    </ModalContent>
                </Modal>

            </Flex>
        </>
    )
}
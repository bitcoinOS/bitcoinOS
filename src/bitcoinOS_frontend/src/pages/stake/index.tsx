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
    Divider,
    Tag,
    Center
} from '@chakra-ui/react'
import { useNavigate } from 'react-router-dom';
import {
    Table,
    Thead,
    Tbody,
    Tfoot,
    Tr,
    Th,
    Td,
    TableCaption,
    TableContainer,
} from '@chakra-ui/react'
import { Card, CardHeader, CardBody, CardFooter } from '@chakra-ui/react'
import { useToast } from '@chakra-ui/react'

import { checkIdentityExpiration } from '../../utils/utils';
import useGetStakePool from '../../utils/poolActor'
import useGetWalletPool from '../../utils/walletActor';

import React, { useEffect, useState, useRef } from 'react';
import { StakepoolStore, useStakeListStore, useAllInfo, getNetworkInfo } from "../../store/index"
import { WalletStore } from '../../store/useWalletStore'


import { Metadata, useWalletBackend, Result_1 as BalanceResult, StakingRequest, Result_3 as StakeResult, StakingRecords, StakingRecord, MetadataRecords, TransferRequest } from "../../ic/WalletActors";
import { TotalStakingRequest, utxosRecords, UtxosRequest, UtxosResponse } from "../../ic/WalletActors";
import { RedeemRequest } from "../../ic/StakePoolActors"
import { useOsBackend, WalletInfo, Result as StakingPoolResult, StakingPoolInfo, CreateStakingPoolRequest } from "../../ic/OsActors";
import { useSatkePoolBackend, StakingRecords as allStakingRecords, StakingRecord as allStakingRecord } from "../../ic/StakePoolActors";
import { useInternetIdentity } from "ic-use-internet-identity";
import { Principal } from "@dfinity/principal"
import { stakingpool } from '../../../../declarations/stakingpool'
import { RedeemResponse } from '../../ic/StakePoolActors'

//Components of Line Charts
import CurvedLineChart from '../../components/chart/CurvedLineChart';
import ChartContainer from '../../components/chart/ChartContainer';
import ChatUser from '../../components/chart/ChatUser';
import { createActor } from "../../../../declarations/smartwallet/index";
import { createActor as createActor_staking } from "../../../../declarations/stakingpool/index";
import { usePointBackend } from '../../ic/PointActors';
import { point } from '../../../../declarations/point';

import { useConnectStore } from '../../store/useConnectStore';

export default function Stake() {
    const toast = useToast();
    const navigate = useNavigate();
    const { get_stake_pool, get_allstake_records, get_tvl_user, get_btc_price, get_user_list, get_tvl_list } = useGetStakePool();
    const { btcprice } = getNetworkInfo()
    const { actor: walletBackend } = useWalletBackend();
    const { actor: osBackend } = useOsBackend();
    const { actor: stakeBackend } = useSatkePoolBackend();
    const { actor: pointBackend } = usePointBackend();
    const { identity } = useInternetIdentity();
    /*--- wallet Info ---*/
    const [walletList, setWalletList] = useState<WalletInfo[]>([])
    const [walletSelect, setWalletSelect] = useState([])
    const [wallet, setWallet] = useState<string>("")
    const [walletUtxos, setWalletUtxos] = useState<UtxosResponse[]>([])
    const [walletMetadata, setWalletMetadata] = useState([])
    const [balance, setBalance] = useState<number>(0)
    const { isOpen: isWalletOpen, onOpen: onWalletOpen, onClose: onWalletClose } = useDisclosure();

    const { get_wallets } = useGetWalletPool()

    const [totalBalance, setTotalBalance] = useState<number>(0)

    const [stakeBalance, setStakeBalance] = useState<number>(0)
    const [isstakeBalance, setIsstakeBalance] = useState<number>(0)
    const { currentWallet, setCurrentWallet } = WalletStore();
    const { stakepoolCanister, setStakepoolCanister } = StakepoolStore();
    const [balanceError, setBalanceError] = useState<string>("");
    const [isLogin, setIslogin] = useState<boolean>(false)

    //const [tvl, setTvl] = useState<number>(0)
    //const [users, setUsers] = useState<number>(0)
    const { tvl, users } = useAllInfo()

    const [btcUnit, setBtcUnit] = useState('btc');
    const [isWalletInited, setIsWalletInited] = useState<boolean>(false)
    const [isOsInited, setIsOsInited] = useState<boolean>(false)
    const [isStakePoolInited, setIsStakePoolInited] = useState<boolean>(false)
    const [isPointInited, setIsPointInited] = useState<boolean>(false)
    const { isOpen: isCreateOpen, onOpen: onCreateOpen, onClose: onCreateClose } = useDisclosure();
    const [walletName, setWalletName] = useState<string>("");
    const [isLoading, setIsLoading] = useState<boolean>(false)

    /*--- transfer Info ---*/
    const [transferBalance, setTransferBalance] = useState<number>(0)
    const [transferAddress, setTransferAddress] = useState<string>("")
    /*--- stake pool Info ---*/
    const { currentPool, setCurrentPool, poolDetail, setPoolDetail } = useStakeListStore();
    const [isFirstCall, setIsFirstCall] = useState(true);
    const stakeList = useStakeListStore((state) => state.stakeList);
    const setStakeList = useStakeListStore((state) => state.setStakeList);
    const [stakeSelect, setStakeSelect] = useState([])
    const [stakeAddress, setStakeAddress] = useState<string>("")
    const [stakeCanister, setStakeCanister] = useState<Principal>();
    const [stakeRecords, setStakeRecords] = useState<StakingRecord[]>([])
    const [allstakeRecords, setAllstakeRecords] = useState<allStakingRecord[]>([])
    const [initialLoadDoneWallet, setInitialLoadDoneWallet] = useState(false);
    const [initialLoadDoneStake, setInitialLoadDoneStake] = useState(false);
    const [initialLoadDoneOs, setInitialLoadDoneOs] = useState(false);

    const [currentAccount] = useConnectStore((state) => [state.currentAccount])

    const btcunity = 100000000;
    const [btc, setBtc] = useState(100000000); // starting value


    useEffect(() => {
        //if (stakeList.length === 0) {
        //    get_stake_pool(osBackend)
        //get_wallet_count()
        //}
        if (identity) {
            setIslogin(true)
        } else {
            setIsOsInited(false)
            setIsLoading(false)
            setIslogin(false)
            setIsStakePoolInited(false)
            setIsWalletInited(false)

        }
        if (!walletBackend) {
            setIsWalletInited(false);
        } else {
            setIsWalletInited(true);
        }
        if (!stakeBackend) {
            setIsStakePoolInited(false);
        } else {
            //get_tvl();
            setIsStakePoolInited(true);
        }
        if (!osBackend) {
            setIsOsInited(false)
        } else {
            setIsOsInited(true)
            //get_wallets()
            //get_stake_pool(osBackend)
            //get_wallet_count()
        }
    }, [])


    useEffect(() => {
        if (identity) {
            setIslogin(true)
        } else {
            if (isLogin) {
                setIsOsInited(false)
                setIsLoading(false)
                setIslogin(false)
                setIsStakePoolInited(false)
                setIsWalletInited(false)
            }
        }
    }, [identity])

    useEffect(() => {
        if (osBackend) {
            get_tvl_user(osBackend)
            get_user_list()
            get_tvl_list()
        }
        if (osBackend && stakeList.length === 0) {
            //get_wallets()
            get_stake_pool(osBackend)
            //get_wallet_count()
            //setInitialLoadDoneOs(true)

        }
    }, [osBackend])

    useEffect(() => {
        if (pointBackend) {
            setIsPointInited(true)
        }

    }, [pointBackend])

    // Get the principal from the backend when an identity is available
    useEffect(() => {
        // debugger
        if (osBackend) {
            setIsOsInited(true)
        }
        if (identity && osBackend && !wallet) {
            get_wallets(osBackend)
        }
    }, [osBackend, identity]);

    useEffect(() => {
        if (identity && stakeBackend) {
            //get_tvl()
        }
    }, [stakeBackend, identity]);

    async function onChangeWallet(event: React.ChangeEvent<HTMLSelectElement>) {
        setWallet(event.target.value)
        setTransferAddress('')
        setTransferBalance(0)
        setStakeBalance(0)

        // Find selected wallet item
        const selectedItem = walletList.find(item => item.bitcoin_address === event.target.value);
        // If the selected item is found and it is not in the walletSelect array, it is added to the array
        if (selectedItem) {
            setWalletSelect([selectedItem]);
        }

        const selectOption = event.target.selectedOptions[0]
        if (selectOption.dataset.id) {
            setCurrentWallet(selectOption.dataset.id)
        }
        await updateWalletData(event.target.value);

    }

    useEffect(() => {
        // Update balance with new btc value
        if (btc === 1) {
            console.log(btc)
        } else if (btc === 100000000) {
            console.log(btc)
        }
    }, [btc]); // Trigger update when btc changes
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
    /*--- change transfer info ---*/
    function handleChangeTransferAddress(event: React.ChangeEvent<HTMLInputElement>) {
        const value = parseFloat(event.target.value)

        setTransferAddress(event.target.value)
        setBalanceError("")

    }
    function handleChangeTransfer(event: React.ChangeEvent<HTMLInputElement>) {
        const value = parseFloat(event.target.value)
        if (value >= balance) {
            setBalanceError("*BTC balance is insufficient ")
        } else {
            setTransferBalance(parseFloat(event.target.value))
            setBalanceError("")
        }

    }
    function handleChangeStake(event: React.ChangeEvent<HTMLInputElement>) {
        const value = parseFloat(event.target.value)
        if (value >= balance) {
            setBalanceError("*BTC balance is insufficient ")
        } else {
            setStakeBalance(parseFloat(event.target.value))
            setBalanceError("")
        }

    }
    function resetallInit() {
        setIsOsInited(false)
        setIsLoading(false)
        setIslogin(false)
        setIsStakePoolInited(false)
        setIsWalletInited(false)
    }
    //Get all pool information
    async function get_stake_pool_ago() {
        if (!osBackend) return;
        if (!checkIdentityExpiration(identity)) {
            resetallInit();
            return;
        }
        setIsLoading(true);
        try {
            const value = await osBackend.list_staking_pool();
            const results = await Promise.all(value.map(async (pool) => {
                const { staking_pool_canister } = pool;
                const result = await createActor_staking(staking_pool_canister.toString()).tvl();
                console.log(result);
                return {
                    ...pool,
                    tvl: result
                };
            }));

            setStakeList(results);

            if (isFirstCall && value.length > 0) {
                const stakePool = value[0];
                //setStakeAddress(stakePool.bitcoin_address);
                //setStakeCanister(stakePool.staking_pool_canister);
                //setStakepoolCanister(stakePool.staking_pool_canister.toText());
                setIsFirstCall(false);
            }
        } catch (error) {
            toast({
                title: 'Info',
                description: "get stake error",
                status: 'error',
                position: 'bottom-right',
                duration: 9000,
                isClosable: true,
            });
            console.error("Error fetching staking pool:", error);
        } finally {
            setIsLoading(false);
        }
    }

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
            setStakeRecords([]);
            setIsLoading(false);
            return;
        }

        try {

            // Parallel calls to get_balance and get_stake_records
            const [balanceResult, stakeRecordsResult] = await Promise.all([
                walletBackend.balance(addr),
                walletBackend.list_staking()
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

            // Processing staking Recording results
            if ('Ok' in stakeRecordsResult) {
                const records: StakingRecord[] = stakeRecordsResult.Ok;
                setStakeRecords(records);
                let r: bigint = 0n;
                records.forEach((record) => {
                    r += record.sent_amount;
                });
                setTotalBalance(Number(r) * 1.0 / btcunity);
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

    async function get_wallet_utxos(addr: string) {
        if (!walletBackend) return;
        if (!checkIdentityExpiration(identity)) {
            resetallInit()
            return
        }
        // if(wallet.length <=1) return;
        setIsLoading(true);
        if (!addr || addr.length < 1) {
            setBalance(0);
            setStakeRecords([]);
            setIsLoading(false);
            return;
        }
        try {
            const stakeRequest: UtxosRequest = {
                'filter': [],
                'address': addr,
            }
            const value: utxosRecords = await walletBackend.utxos(stakeRequest);
            if ('Err' in value) {
                toast({
                    title: 'utxo',
                    description: "get balance error",
                    status: 'error',
                    position: "bottom-right",
                    duration: 9000,
                    isClosable: true,
                });
            } else {
                if ('Ok' in value) {
                    const result: UtxosResponse = value.Ok;
                    setWalletUtxos([result])
                }
            }
        } catch (error) {
            console.error('Error getting utxos:', error);
            toast({
                title: 'Balance',
                description: "get utxos error",
                status: 'error',
                position: "bottom-right",
                duration: 9000,
                isClosable: true,
            });
        } finally {
            setIsLoading(false);
        }
    }
    // function stake_pool_canister(){
    //     if(osBackend){
    //         osBackend.list_staking_pool().then((value: StakingPoolInfo[]) => {
    //             if(value.length >0){
    //                 const stakePool = value[0]
    //                 setStakepoolCanister(stakePool.staking_pool_canister.toText())
    //             }
    //         })
    //     }
    // }

    function onCreateWallet() {
        onCreateClose()
        if (!osBackend || !identity) {
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
    function refresh() {

        get_wallets(osBackend)
        //get_tvl()
        //get_wallet_count()
        get_stake_pool(osBackend)
    }

    function sub(s: string) {
        const l = s.length
        return s.substring(0, 3) + "..." + s.substring(l - 3, l);
    }
    const formatDate = (bigintTimestamp) => {
        const date = new Date(Number(bigintTimestamp / 1000000n)); // Assuming the timestamp is in nanoseconds, convert to milliseconds
        return date.toLocaleString();
    };

    // pool list scroll
    const flexRef = useRef(null);
    const scrollRight = () => {
        if (flexRef.current) {
            flexRef.current.scrollBy({ left: 300, behavior: 'smooth' });
        }
    };

    const scrollLeft = () => {
        if (flexRef.current) {
            flexRef.current.scrollBy({ left: -300, behavior: 'smooth' });
        }
    };

    const test = async () => {
        const result = await get_user_list()
        // toast({
        //     title: 'Error',
        //     description: "Failed to fetch BTC price after multiple attempts.",
        //     status: 'error',
        //     position: 'bottom-right',
        //     duration: 9000,
        //     variant: 'left-accent right-accent',
        //     isClosable: true,
        //     colorScheme: 'red',
        //     containerStyle: {
        //         border: '1px solid #FF8800',
        //         background: 'white',
        //         color: 'red',
        //         borderRadius: 'xl', // 可选：添加圆角
        //         boxShadow: '0 0 10px rgba(255, 136, 0, 0.2)', // 可选：添加阴影效果
        //     },
        // });
        /*
        stakeBackend.tvl().then((v: BigInt) => {
            setTvl(Number(v) * 1.0 / btcunity)
            setIsLoading(false);
        }).catch((error) => {
            console.error("Error fetching wallet count:", error);
        }).finally(() => {
            setIsLoading(false);
        });
        */
    }


    //Click to enter pool details
    const onCardClick = async (stake) => {
        setStakepoolCanister(stake.staking_pool_canister.toText())
        setCurrentPool(stake);
        setPoolDetail(true);
        await get_allstake_records(stake.staking_pool_canister.toText())
        navigate('/pools');
    }
    return (
        <>
            <Flex direction='column' alignItems='center'>
                {/*
                <Button onClick={test}>test</Button>
                */}
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
                <Flex
                    justifyContent='center'
                    alignItems='center'
                    fontSize='16px'
                    fontWeight='700'
                    height='40px'
                    width='100vw'
                    bgGradient='linear(to-r, #FF8800, #AA00FF, #3600FF, #33CCFF)'
                    color='white'
                >
                    btc Marathon is on!  Come and join us!
                </Flex>
                <Flex
                    direction='column'
                    width='100%'
                    //minWidth='1500px' 
                    alignItems='center'
                >
                    <Flex
                        width="100%"
                        backgroundImage="url('./home/background-top.svg')" // Add your background image path
                        backgroundSize="cover"
                        backgroundPosition="center"
                        display="flex"
                        justifyContent="center"
                        alignItems="center"
                        direction='column'
                    >
                        <Flex
                            width='78%'
                            maxWidth='1120px'
                            mt={6}
                            direction='row'
                            alignItems='center'
                            justifyContent='space-between'
                        //minWidth='1350px'
                        >
                            <Flex direction='column'>
                                <Text>
                                    <Flex alignItems='center'>
                                        <Heading fontSize={{ md: '28px', lg: '40px', xl: '40px', '2xl': '40px', '3xl': '40px' }}>BifiPal</Heading>
                                        <Text
                                            //mt={2}
                                            pl='2'
                                            py='1'
                                            borderRadius='xl'
                                            background="linear-gradient(to right, #FFA033, #FFFFFF)"
                                            fontStyle='italic'
                                            fontWeight='400'
                                            ml='2'
                                            fontSize={{ md: '20px', lg: '20px', xl: '20px', '2xl': '20px', '3xl': '20px' }}
                                        >
                                            Make Bitcoin the Pioneer in Finance
                                        </Text>
                                    </Flex>
                                    <Text
                                        mt={2}
                                        color='gray.700'
                                        fontWeight='500'
                                        fontSize={{ md: '20px', lg: '20px', xl: '20px', '2xl': '20px', '3xl': '20px' }}
                                    >
                                        Unlocking the DeFi Gaming Paradigm for Bitcoin
                                    </Text>
                                </Text>
                                <Text
                                    fontSize={{ md: '20px', lg: '20px', xl: '20px', '2xl': '20px', '3xl': '20px' }}
                                    color='gray.600'
                                    fontWeight='500'
                                    mt='10'
                                >
                                    Total {tvl} btc Staked
                                </Text>
                            </Flex>
                            <Flex
                                alignItems="center"
                                p={3}
                                direction="row"
                                border="2px"
                                borderColor="gray.200"
                                borderRadius="3xl"
                                background="linear-gradient(to right, #CFD9FF, #F2F5FF)" // Setting a linear gradient background from left to right
                            >
                                <div>
                                    <Image height='32px' src="./home/coin.svg"></Image >
                                </div>
                                <Flex ml={2} direction='column'>
                                    <Flex fontSize={{ md: '12px', lg: '12px', xl: '12px', '2xl': '12px', '3xl': '12px' }}>
                                        <Flex width='40px'><Text color='gray.500' fontWeight='400'>TVL: </Text></Flex><Text color='gray.700' fontWeight='500'>{tvl * btcprice} btc</Text>
                                    </Flex>
                                    <Flex fontSize={{ md: '12px', lg: '12px', xl: '12px', '2xl': '12px', '3xl': '12px' }}>
                                        <Flex width='40px'><Text color='gray.500' fontWeight='400'>Users: </Text></Flex> <Text color='gray.700' fontWeight='500'>{users}</Text>
                                    </Flex>
                                </Flex>
                            </Flex>
                            {/* <Flex>
                                <Image src="bitcoinos.jpg"></Image>
                            </Flex> */}
                        </Flex>
                        <Box
                            position="relative"
                            width="78%"
                            maxWidth='1120px'
                        //overflow="hidden"
                        //minWidth='1350px'
                        >
                            <Flex
                                mt={5}
                                borderColor="gray.200"
                                overflowX="scroll"
                                width="100%"
                                ref={flexRef}
                                whiteSpace="nowrap" // Keeping content on one line
                                sx={{
                                    '::-webkit-scrollbar': {
                                        display: 'none',
                                    },
                                    '-ms-overflow-style': 'none',  // IE and Edge
                                    'scrollbar-width': 'none',  // Firefox
                                }}
                            >
                                {stakeList && stakeList.length > 0 ? (
                                    stakeList.map((stake, index) => (
                                        <Card
                                            key={index}
                                            borderWidth="1px"
                                            borderColor="gray.200"
                                            borderRadius="lg"
                                            overflow="hidden"
                                            width='28%'
                                            //maxWidth="28%" // Make sure the width is consistent
                                            m={2} // Add margin for spacing between cards
                                            flexShrink={0} // Prevents card shrinkage
                                            style={{ cursor: 'pointer' }}
                                            background="linear-gradient(to right, #FFFFFF 0%, #E0E6FF 100%)" // Linear Gradient Background
                                            onClick={() => onCardClick(stake)}
                                        >
                                            <CardHeader>
                                                <Flex direction="column">
                                                    <Flex justifyContent="space-between" alignItems="center" width='100%'>
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
                                                            <Flex alignItems='center'>
                                                                <Box
                                                                    color='#ED8936'
                                                                    as="span"
                                                                    fontWeight="bold"
                                                                    fontSize={{ md: '24px', lg: '24px', xl: '24px', '2xl': '24px', '3xl': '24px' }}
                                                                >{Number(stake.annual_interest_rate).toFixed(2)}%</Box>
                                                                <Text fontSize={{ md: '12px', lg: '12px', xl: '12px', '2xl': '12px', '3xl': '12px' }} ml='2'>APY</Text>
                                                            </Flex>
                                                        </Flex>
                                                        <Flex>
                                                            <Image width={{ md: '40px', lg: '40px', xl: '40px', '2xl': '40px', '3xl': '40px' }} src="./home/bitcoin-btc-logo.svg"></Image>
                                                        </Flex>
                                                    </Flex>
                                                    <Flex direction="column" mt='3'>
                                                        <Flex justifyContent="space-between" pr='2'>
                                                            <Text fontSize={{ md: '12px', lg: '12px', xl: '12px', '2xl': '12px', '3xl': '12px' }}>Net Asset Value</Text>
                                                            <Flex fontSize={{ md: '12px', lg: '12px', xl: '12px', '2xl': '12px', '3xl': '12px' }}>
                                                                <Text fontWeight='bold' mr='1'>
                                                                    {Number(stake.tvl) * 1.0 / btcunity}
                                                                </Text>
                                                                btc
                                                            </Flex>
                                                        </Flex>
                                                        <Flex justifyContent="space-between" pr='2'>
                                                            <Text fontSize={{ md: '12px', lg: '12px', xl: '12px', '2xl': '12px', '3xl': '12px' }}>TVL</Text>
                                                            <Flex fontSize={{ md: '12px', lg: '12px', xl: '12px', '2xl': '12px', '3xl': '12px' }}>
                                                                <Text fontWeight='bold' mr='1'>
                                                                    {Number(stake.tvl) * 1.0 / btcunity}
                                                                </Text>
                                                                btc
                                                            </Flex>
                                                        </Flex>
                                                    </Flex>
                                                </Flex>
                                            </CardHeader>
                                        </Card>
                                    ))
                                ) : (
                                    <Text>No staking pools available.</Text>
                                )}
                            </Flex>
                            <Button
                                position="absolute"
                                left="-40px"
                                top="50%"
                                transform="translateY(-50%)"
                                zIndex="1"
                                onClick={scrollLeft}
                                backgroundColor="#A8B0C1"
                                color='white'
                                borderRadius="50%"
                                width="40px"
                                height="40px"
                                display="flex"
                                alignItems="center"
                                justifyContent="center"
                                p="0"
                            >
                                {'<'}
                            </Button>
                            <Button
                                position="absolute"
                                right="-40px"
                                top="50%"
                                transform="translateY(-50%)"
                                zIndex="1"
                                onClick={scrollRight}
                                backgroundColor="#A8B0C1"
                                color='white'
                                borderRadius="50%"
                                width="40px"
                                height="40px"
                                display="flex"
                                alignItems="center"
                                justifyContent="center"
                                p="0"
                            >
                                {'>'}
                            </Button>
                        </Box>
                    </Flex>
                    <Flex
                        maxWidth='1120px'
                        mt={5}
                        p={5}
                        boxShadow="lg"
                        border="1px"
                        borderColor="gray.200"
                        borderRadius="md"
                        //minWidth='1350px'
                        width="78%"
                        direction='column'
                        bgColor='white'
                    >
                        <Flex mb='10' p='6'>
                            <ChartContainer />
                            <ChatUser />
                        </Flex>
                        <Flex mt={3} mb={8}>
                            <Text fontWeight='bold' fontSize='24px'>Top Pools</Text>
                        </Flex>
                        <Divider />
                        <Table variant='simple' size='md'>
                            <Thead>
                                <Tr fontSize={{ md: '12px', lg: '12px', xl: '12px', '2xl': '12px', '3xl': '12px' }}>
                                    <Th>#</Th>
                                    <Th textTransform="none">Pool</Th>
                                    <Th textTransform="none">TVL(btc)</Th>
                                    <Th textTransform="none">Volumn(btc)</Th>
                                    <Th textTransform="none">APR</Th>
                                </Tr>
                            </Thead>
                            <Tbody>
                                {stakeList && stakeList.length > 0 ? (
                                    stakeList.map((stake, index) => {
                                        return (
                                            <Tr key={index} fontSize={{ md: '14px', lg: '14px', xl: '14px', '2xl': '14px', '3xl': '14px' }}>
                                                <Td>{index + 1}</Td>
                                                <Td>
                                                    <Flex justifyContent='flex-start' alignItems='center'>
                                                        <Flex>
                                                            <Image mr='2' height='24px' width={{ md: '24px', lg: '24px', xl: '24px', '2xl': '24px', '3xl': '24px' }} src="./home/bitcoin-btc-logo.svg">
                                                            </Image>
                                                            {stake.name}
                                                        </Flex>
                                                    </Flex>
                                                </Td>
                                                <Td>{Number(stake.tvl) * 1.0 / btcunity}</Td>
                                                <Td>{Number(stake.tvl) * 1.0 / btcunity}</Td>
                                                <Td>{stake.annual_interest_rate}%</Td>
                                                <Td>
                                                    <Button
                                                        variant='ghost'
                                                        color='#3861FB'
                                                        fontSize='14px'
                                                        onClick={() => onCardClick(stake)}
                                                    >Stake</Button>
                                                </Td>
                                            </Tr>

                                        );
                                    })
                                ) : (
                                    <Text>No staking pools available.</Text>
                                )}
                            </Tbody>
                        </Table>
                    </Flex>

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


                    {/*
                    <Box mt={2} boxShadow="lg" border="1px" borderColor="gray.200" borderRadius="md" p={3} zIndex={4}>

                        <Flex width='100%' mb={4}>
                            <Flex direction='column'>
                                <Flex>
                                    <Text mr={2}>Wallets:</Text>
                                    <Select onChange={onChangeWallet} mr={10} width="100%" maxWidth="200px">
                                        {
                                            walletList.length === 0
                                                ? <option value="" disabled>No Wallet</option>
                                                : null
                                        }
                                        {
                                            walletList.map((item, index) => (<option key={index} value={item.bitcoin_address} data-id={item.wallet_canister.toText()}>{item.name}</option>))
                                        }
                                    </Select>
                                </Flex>
                                {wallet.length > 0 && <Text fontSize='sm' mt="2" maxWidth="100%" overflow="auto" whiteSpace="nowrap">address:  {(wallet)}</Text>}
                                {wallet.length > 0 && <Text fontSize='sm' mt="2">{walletSelect.map((item, index) => (
                                    Object.keys(item.network).map((key) => (
                                        <Box key={key}>Network: {key}</Box>
                                    ))
                                ))}</Text>}
                            </Flex>
                            <Flex>
                                <Text mr={2}>Pools:</Text>
                                <Select onChange={onChangeStake} mr={10} width="100%">
                                    {
                                        stakeList.length === 0
                                            ? <option value="" disabled>No Pool</option>
                                            : null
                                    }
                                    {
                                        stakeList.map((item, index) => (<option key={index} value={item.bitcoin_address} data-id={item.os_canister.toText()}>{item.name}</option>))
                                    }
                                </Select>
                            </Flex>
                            <Button onClick={onWalletOpen}>Wallet</Button>
                            <Spacer></Spacer>
                            <Button
                                bgColor="orange.400"
                                color="white"
                                isDisabled={!isLogin || !isOsInited}
                                _hover={{ bg: "orange.200", borderColor: "orange.400" }}
                                onClick={onCreateOpen}
                                mr='10'
                            >
                                Create Wallet
                            </Button>
                            <Button
                                bgColor="orange.400"
                                color="white"
                                isDisabled={!isLogin || !isOsInited}
                                _hover={{ bg: "orange.200", borderColor: "orange.400" }}
                                onClick={refresh}>
                                <BsArrowClockwise />
                            </Button>
                            <Modal
                                isOpen={isCreateOpen}
                                onClose={onCreateClose}
                            >
                                <ModalOverlay />
                                <ModalContent>
                                    <ModalHeader>Create your wallet</ModalHeader>
                                    <ModalCloseButton />
                                    <ModalBody pb={6}>
                                        <FormControl>
                                            <FormLabel>wallet name</FormLabel>
                                            <Input placeholder="wallet name" onChange={event => setWalletName(event.currentTarget.value)} />
                                        </FormControl>
                                    </ModalBody>

                                    <ModalFooter>
                                        <Button
                                            bgColor="purple.500"
                                            color="white"
                                            _hover={{ bg: "purple.300", borderColor: "purple.500" }}
                                            mr={3} onClick={onCreateWallet}>
                                            create
                                        </Button>
                                        <Button color="white" bgColor="gray.500" onClick={onCreateClose}>Cancel</Button>
                                    </ModalFooter>
                                </ModalContent>
                            </Modal>
                        </Flex>


                        <Flex justify="space-around" alignItems="center" width="100%">
                            <Tabs minHeight="500px" width="100%" maxWidth="600px">
                                <Flex justifyContent="center" borderBottom="1px solid" borderColor="gray.200">
                                    <TabList>
                                        <Tab mr={10} _selected={{ color: 'orange.400', borderBottom: '2px solid', borderColor: 'orange.400' }}>Transfer</Tab>
                                        <Tab mr={10} _selected={{ color: 'orange.400', borderBottom: '2px solid', borderColor: 'orange.400' }}>Stake</Tab>
                                        <Tab mr={10} _selected={{ color: 'orange.400', borderBottom: '2px solid', borderColor: 'orange.400' }}>Detail</Tab>
                                    </TabList>
                                </Flex>

                                <TabPanels>
                                    <TabPanel>
                                        <Flex mt={2} justifyContent="center">
                                            <VStack align='left' width='410px'>
                                                <HStack align='end'>
                                                    <Text fontSize='sm'>BTC Balance:{balance}</Text>
                                                    <Spacer></Spacer>
                                                    <Flex>
                                                        <Text fontSize='sm'>osBTC Balance:{totalBalance}</Text>
                                                        // <Button
                                                        //    bgColor="orange.400"
                                                        //    color="white"
                                                        //    size='sm'
                                                        //    _hover={{ bg: "orange.200", borderColor: "orange.400" }}
                                                        //    onClick={refresh}>
                                                        //    <BsBoxArrowUpRight  />
                                                        //</Button> 
                                                    </Flex>
                                                </HStack>
                                                <HStack bg="gray.200" p={1} borderRadius="lg">

                                                    <InputGroup>

                                                        <Input type="string" value={transferAddress} border="none" placeholder='address' isDisabled={!isLogin} onChange={handleChangeTransferAddress}></Input >

                                                    </InputGroup>
                                                </HStack>
                                                <HStack bg="gray.200" p={1} borderRadius="lg">
                                                    <InputGroup>
                                                        <InputLeftElement pointerEvents="none">
                                                            <Image src='./favicon.png' boxSize="1.2rem" />
                                                        </InputLeftElement>

                                                        <Input
                                                            type="number"
                                                            value={transferBalance}
                                                            border="none"
                                                            placeholder='0.0'
                                                            isDisabled={!isLogin}
                                                            onChange={handleChangeTransfer}
                                                            pr="4.5rem" // Add padding to the right to make space for the InputRightElement
                                                        />

                                                        <InputRightElement width="auto" display="flex" alignItems="center">
                                                            <Select value={btcUnit} onChange={onChangebtcunit} width="auto" mr={2}>
                                                                <option value="btc">btc</option>
                                                                <option value="satoshi">satoshi</option>
                                                            </Select>
                                                            <Button color="orange.300" isDisabled={!isLogin} p={2} fontSize="0.8rem" onClick={onMaxClick}>
                                                                MAX
                                                            </Button>
                                                        </InputRightElement>
                                                    </InputGroup>
                                                </HStack>

                                                <Text fontSize="0.8rem" color='red'><span>{balanceError}</span></Text>
                                                <Text fontSize='sm'>Exchange Rate 1.00 BTC = 1.00 osBTC</Text>
                                                <Flex width='100%' direction='column' align="center" pt={4}>
                                                    {isLogin && <Button height="2.5rem" width="40%" color="white" bgColor="orange.400" _hover={{ bg: "orange.200", borderColor: "orange.400" }} isDisabled={transferBalance <= 0 || !isOsInited} onClick={transfer_balance}>Transfer</Button>}
                                                    {!isLogin && <Button height="2.5rem" width="40%" color="white" bgColor="orange.400" _hover={{ bg: "orange.200", borderColor: "orange.400" }} onClick={login}>Login/Registe</Button>}
                                                </Flex>
                                            </VStack>
                                        </Flex>
                                    </TabPanel>
                                    <TabPanel>
                                        <Flex mt={2} justifyContent="center">
                                            <VStack align='left' width='410px'>
                                                <HStack align='end'>
                                                    <Text fontSize='sm'>BTC Balance:{balance}</Text>
                                                    <Spacer></Spacer>
                                                    <Flex>
                                                        <Text fontSize='sm'>osBTC Balance:{totalBalance}</Text>
                            
                                                    </Flex>
                                                </HStack>
                                                <HStack bg="gray.200" p={1} borderRadius="lg">
                                                    <InputGroup>
                                                        <InputLeftElement
                                                            pointerEvents="none"
                                                        >
                                                            <Image src='./favicon.png' boxSize="1.2rem" />
                                                        </InputLeftElement>
                                                        <Input type="number" value={stakeBalance} border="none" placeholder='0.0' isDisabled={!isLogin} onChange={handleChangeStake}></Input >

                                                        <InputRightElement width="auto" display="flex" alignItems="center">
                                                            <Select value={btcUnit} onChange={onChangebtcunit} width="auto" mr={2}>
                                                                <option value="btc">btc</option>
                                                                <option value="satoshi">satoshi</option>
                                                            </Select>
                                                            <Button color="orange.300" isDisabled={!isLogin} p={2} fontSize="0.8rem" onClick={onMaxClick}>MAX</Button>
                                                        </InputRightElement>
                                                    </InputGroup>
                                                </HStack>
                                                <Text fontSize="0.8rem" color='red'><span>{balanceError}</span></Text>
                                                <Text fontSize='sm'>Exchange Rate 1.00 BTC = 1.00 osBTC</Text>
                                                <Flex width='100%' direction='column' align="center" pt={4}>
                                                    {isLogin && <Button height="2.5rem" width="40%" color="white" bgColor="orange.400" _hover={{ bg: "orange.200", borderColor: "orange.400" }} isDisabled={stakeBalance <= 0 || !isOsInited} onClick={onStake}>Stake</Button>}
                                                    {!isLogin && <Button height="2.5rem" width="40%" color="white" bgColor="orange.400" _hover={{ bg: "orange.200", borderColor: "orange.400" }} onClick={login}>Login/Registe</Button>}
                                                </Flex>
                                            </VStack>
                                        </Flex>
                                    </TabPanel>
                                    <TabPanel>
                                        <TableContainer>
                                            <Table variant='simple' size="sm">
                                                <Thead>
                                                    <Tr>
                                                        <Th>Date</Th>
                                                        <Th>Stake pool</Th>
                                                        <Th>Ammount(BTC) </Th>
                                                        <Th>Day</Th>
                                                    </Tr>
                                                </Thead>
                                                <Tbody>
                                                    {
                                                        stakeRecords.map((v, i) => {

                                                            return (
                                                                <Tr>
                                                                    <Td>{new Date(Number(v.sent_time) / 1000000).toLocaleDateString(undefined, { year: 'numeric', month: '2-digit', day: '2-digit' })}</Td>
                                                                    <Td>{sub(v.staking_address)}</Td>
                                                                    <Td>{Number(v.sent_amount) / btc}</Td>
                                                                    <Td>{v.duration_in_day.toString()}</Td>
                                                                    <Td>
                                                                        <Button onClick={() => unstake_balance(v.txid, wallet, v.network)}>unstake</Button>
                                                                    </Td>
                                                                </Tr>
                                                            );
                                                        })
                                                    }
                                                </Tbody>
                                            </Table>
                                        </TableContainer>
                                    </TabPanel>
                                </TabPanels>
                            </Tabs>
                            <Flex direction="column" >
                                <Tabs minHeight="500px" width="100%" maxWidth="600px">
                                    <Flex justifyContent="center" borderBottom="1px solid" borderColor="gray.200">
                                        <TabList>
                                            <Tab mr={10} _selected={{ color: 'orange.400', borderBottom: '2px solid', borderColor: 'orange.400' }}>Stake Pool Info</Tab>
                                            <Tab mr={10} _selected={{ color: 'orange.400', borderBottom: '2px solid', borderColor: 'orange.400' }}>Stake Record </Tab>
                                        </TabList>
                                    </Flex>
                                    <TabPanels>
                                        <TabPanel>
                                            {stakeSelect.length > 0 && (
                                                <Box maxWidth="600px" mx="auto">

                                                    <Table variant='simple' size='sm' maxHeight="380px">
                                                        <Tbody>
                                                            {stakeSelect.map((item, index) => (
                                                                <React.Fragment key={index}>
                                                                    <Tr>
                                                                        <Td>Name:</Td>
                                                                        <Td>{item.name}</Td>
                                                                    </Tr>
                                                                    <Tr>
                                                                        <Td>Canister ID:</Td>
                                                                        <Td>{item.staking_pool_canister.toText()}</Td>
                                                                    </Tr>
                                                                    <Tr>
                                                                        <Td>Address:</Td>
                                                                        <Td>{item.bitcoin_address}</Td>
                                                                    </Tr>
                                                                    <Tr>
                                                                        <Td>Description:</Td>
                                                                        <Td>{item.description}</Td>
                                                                    </Tr>
                                                                    <Tr>
                                                                        <Td>Duration Day:</Td>
                                                                        <Td>{item.duration_in_day.toString()}</Td>
                                                                    </Tr>
                                                                    <Tr>
                                                                        <Td>Annual Rata:</Td>
                                                                        <Td>{item.annual_interest_rate}%</Td>
                                                                    </Tr>
                                                                </React.Fragment>
                                                            ))}
                                                        </Tbody>
                                                    </Table>
                                                </Box>
                                            )}
                                        </TabPanel>
                                        <TabPanel>
                                            <TableContainer>
                                                <Table variant='simple' size="sm">
                                                    <Thead>
                                                        <Tr>
                                                            <Th>Date</Th>
                                                            <Th>Stake pool</Th>
                                                            <Th>Ammount(BTC) </Th>
                                                            <Th>Day</Th>
                                                        </Tr>
                                                    </Thead>
                                                    <Tbody>
                                                        {
                                                            allstakeRecords.map((v, i) => {

                                                                return (
                                                                    <Tr>
                                                                        <Td>{new Date(Number(v.sent_time) / 1000000).toLocaleDateString(undefined, { year: 'numeric', month: '2-digit', day: '2-digit' })}</Td>
                                                                        <Td>{sub(v.staking_address)}</Td>
                                                                        <Td>{Number(v.sent_amount) / btc}</Td>
                                                                        <Td>{v.duration_in_day.toString()}</Td>
                                                                    </Tr>
                                                                );
                                                            })
                                                        }
                                                    </Tbody>
                                                </Table>
                                            </TableContainer>
                                        </TabPanel>
                                    </TabPanels>
                                </Tabs>
                            </Flex>
                        </Flex>
                    </Box >
                     */}
                </Flex>
                <Modal isOpen={isWalletOpen} onClose={onWalletClose} isCentered={true}>
                    <ModalOverlay />
                    <ModalContent maxW="35%" minW="700px">
                        <ModalHeader>Wallet Info</ModalHeader>
                        <ModalCloseButton />
                        <ModalBody>
                            {walletSelect.length > 0 && (
                                <Table variant='simple' size='xl'>
                                    <Tbody>
                                        {walletSelect.map((item, index) => (
                                            <React.Fragment key={index}>
                                                <Tr>
                                                    <Td>Name:</Td>
                                                    <Td>{item.name}</Td>
                                                </Tr>
                                                <Tr>
                                                    <Td>Address:</Td>
                                                    <Td>{item.bitcoin_address}</Td>
                                                </Tr>
                                                <Tr>
                                                    <Td>Wallet canister:</Td>
                                                    <Td>{item.wallet_canister.toText()}</Td>
                                                </Tr>
                                                <Tr>
                                                    <Td>Create time</Td>
                                                    <Td>{formatDate(item.created_at)}</Td>
                                                </Tr>
                                                <Tr>
                                                    <Td>Balance</Td>
                                                    <Td>{balance}</Td>
                                                </Tr>
                                            </React.Fragment>
                                        ))}
                                    </Tbody>
                                </Table>
                            )}
                            <Button mt="3%" onClick={() => get_wallet_utxos(wallet)}>Utxos List</Button>
                            <Box maxWidth="600px" mx="auto">
                                <Text fontSize="2xl" textAlign="center" mb={4}>
                                    UTXO Table
                                </Text>
                                {walletUtxos && walletUtxos.length > 0 && walletUtxos.some(response => response.utxos.length > 0) && (
                                    <Table variant='simple' size='sm'>
                                        <Thead>
                                            <Tr>
                                                <Th>Height</Th>
                                                <Th>txid</Th>
                                            </Tr>
                                        </Thead>
                                        <Tbody>
                                            {walletUtxos.map((response, index) => (
                                                response.utxos.map((item, utxoIndex) => (
                                                    <React.Fragment key={`${index}-${utxoIndex}`}>
                                                        <Tr>
                                                            <Td>{item.height}</Td>
                                                            <Td>{item.outpoint.txid}</Td>
                                                        </Tr>
                                                    </React.Fragment>
                                                ))
                                            ))}
                                        </Tbody>
                                    </Table>
                                )}
                            </Box>
                        </ModalBody>
                    </ModalContent>
                </Modal>
            </Flex >
        </>
    )
}
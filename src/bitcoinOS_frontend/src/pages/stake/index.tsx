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
    Center
}
    from '@chakra-ui/react'
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
import { useToast } from '@chakra-ui/react'
import {
    BsArrowClockwise, BsBoxArrowUpRight
} from 'react-icons/bs'
import { Tabs, TabList, TabPanels, Tab, TabPanel } from "@chakra-ui/react"
import { Select } from "@chakra-ui/react"
import React, { useEffect, useState, useRef } from 'react';
import { WalletStore, StakepoolStore } from "../../store/index"
import { Metadata, useWalletBackend, Result_1 as BalanceResult, StakingRequest, Result_3 as StakeResult, StakingRecords, StakingRecord, MetadataRecords, TransferRequest } from "../../ic/WalletActors";
import { TotalStakingRequest, utxosRecords, UtxosRequest, UtxosResponse } from "../../ic/WalletActors";
import { RedeemRequest } from "../../ic/StakePoolActors"
import { useOsBackend, WalletInfo, Result as StakingPoolResult, StakingPoolInfo, CreateStakingPoolRequest } from "../../ic/OsActors";
import { useSatkePoolBackend } from "../../ic/StakePoolActors";
import { useInternetIdentity } from "ic-use-internet-identity";
import { Principal } from "@dfinity/principal"
import { stakingpool } from '../../../../declarations/stakingpool'
import { RedeemResponse } from '../../ic/StakePoolActors'

export default function Stake() {
    const toast = useToast();

    const { actor: walletBackend } = useWalletBackend();
    const { actor: osBackend } = useOsBackend();
    const { actor: stakeBackend } = useSatkePoolBackend();
    const { identity } = useInternetIdentity();
    /*--- wallet Info ---*/
    const [walletList, setWalletList] = useState<WalletInfo[]>([])
    const [walletSelect, setWalletSelect] = useState([])
    const [wallet, setWallet] = useState<string>("")
    const [walletUtxos, setWalletUtxos] = useState<UtxosResponse[]>([])
    const [walletMetadata, setWalletMetadata] = useState([])
    const [balance, setBalance] = useState<number>(0)
    const { isOpen: isWalletOpen, onOpen: onWalletOpen, onClose: onWalletClose } = useDisclosure();

    const [totalBalance, setTotalBalance] = useState<number>(0)

    const [stakeBalance, setStakeBalance] = useState<number>(0)
    const [isstakeBalance, setIsstakeBalance] = useState<number>(0) //已质押数量
    const { currentWallet, setCurrentWallet } = WalletStore();
    const { stakepoolCanister, setStakepoolCanister } = StakepoolStore();
    const [balanceError, setBalanceError] = useState<string>("");
    const [isLogin, setIslogin] = useState<boolean>(false)

    const [tvl, setTvl] = useState<number>(0)
    const [users, setUsers] = useState<number>(0)

    const [isWalletInited, setIsWalletInited] = useState<boolean>(false)
    const [isOsInited, setIsOsInited] = useState<boolean>(false)
    const [isStakePoolInited, setIsStakePoolInited] = useState<boolean>(false)
    const { isOpen: isCreateOpen, onOpen: onCreateOpen, onClose: onCreateClose } = useDisclosure();
    const [walletName, setWalletName] = useState<string>("");
    const [isLoading, setIsLoading] = useState<boolean>(false)

    /*--- transfer Info ---*/
    const [transferBalance, setTransferBalance] = useState<number>(0)
    const [transferAddress, setTransferAddress] = useState<string>("")
    /*--- stake pool Info ---*/
    const [stakeList, setStakeList] = useState<StakingPoolInfo[]>([])
    const [stakeSelect, setStakeSelect] = useState([])
    const [stakeAddress, setStakeAddress] = useState<string>("")
    const [stakeCanister, setStakeCanister] = useState<Principal>();
    const [stakeRecords, setStakeRecords] = useState<StakingRecord[]>([])
    const [initialLoadDone, setInitialLoadDone] = useState(false);
    const btc = 100000000
    useEffect(() => {
        if (identity) {
            setIslogin(true)
        }
        if (isLogin) {
            if (!walletBackend) {
                setIsWalletInited(false);
            } else {
                setIsWalletInited(true);
            }
            if (!stakeBackend) {
                setIsStakePoolInited(true);
            } else {
                get_tvl();
                setIsStakePoolInited(false);
            }
            if (!osBackend) {
                setIsOsInited(false)
            } else {
                setIsOsInited(true)
                get_wallets()
                get_wallet_count()

            }
        }
    }, [])

    useEffect(() => {
        if (!initialLoadDone && walletList.length > 0 && stakeList.length > 0) {
            // Trigger onChangeWallet with the first wallet's value
            const firstWallet = walletList[0];
            onChangeWallet({ target: { value: firstWallet.bitcoin_address, selectedOptions: [{ dataset: { id: firstWallet.wallet_canister.toText() } }] } });
            // Trigger onChangeStake with the first stake pool's value
            const firstStake = stakeList[0];
            onChangeStake({ target: { value: firstStake.bitcoin_address, selectedOptions: [{ dataset: { id: firstStake.os_canister.toText() } }] } });
            setInitialLoadDone(true);
        }
    }, [walletList, stakeList, initialLoadDone]);

    useEffect(() => {
        if (identity) {
            setIslogin(true)
        } else {
            console.log("------------------------------------------------------")
            setIsLoading(false)
            setIslogin(false)
        }
    }, [identity])

    // Get the principal from the backend when an identity is available
    useEffect(() => {
        debugger
        if (osBackend) {
            setIsOsInited(true)
        }
        if (identity && osBackend) {
            get_wallets()
            get_stake_pool()
            get_wallet_count()

        }
    }, [osBackend, identity]);

    useEffect(() => {
        if (identity && stakeBackend) {
            get_tvl()
        }
    }, [stakeBackend, identity]);
    useEffect(() => {
        console.log(isstakeBalance); // { num: 1 } 数据已更新
    }, [isstakeBalance])
    /*
    useEffect(() => {
        if (identity && walletBackend) {
            get_tvl()
            get_balance(wallet)
            get_stake_balance()
        }
    }, [walletBackend, identity]);
    */

    // useEffect(() => {
    //     get_balance()
    //     get_stake_records()
    // }, [wallet])
    useEffect(() => {
        if (currentWallet) {
            const updateData = async () => {
                await updateWalletData(wallet);
            };
            updateData();
        }
    }, [currentWallet]);
    async function onChangeWallet(event: React.ChangeEvent<HTMLSelectElement>) {
        setWallet(event.target.value)
        setTransferAddress('')
        setTransferBalance(0)
        setStakeBalance(0)

        // 查找选中的 wallet 项
        const selectedItem = walletList.find(item => item.bitcoin_address === event.target.value);
        // 如果找到了选中的项，并且它不在 walletSelect 数组中，则添加到数组中
        if (selectedItem) {
            setWalletSelect([selectedItem]);
        }

        const selectOption = event.target.selectedOptions[0]
        if (selectOption.dataset.id) {
            await setCurrentWallet(selectOption.dataset.id)
        }
    }
    /*--- change stake select ---*/
    function onChangeStake(event: React.ChangeEvent<HTMLSelectElement>) {
        get_stake_pool()
        // 查找选中的 wallet 项
        const selectedItem = stakeList.find(item => item.bitcoin_address === event.target.value);
        // 如果找到了选中的项，并且它不在 walletSelect 数组中，则添加到数组中
        if (selectedItem) {
            setStakeSelect([selectedItem]);
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
    function onMaxClick() {
        setStakeBalance(balance)
        setTransferBalance(balance)
    }
    function get_stake_pool() {
        if (!osBackend) return;
        setIsLoading(true)
        console.log('-------------------------------222')
        osBackend.list_staking_pool().then((value: StakingPoolInfo[]) => {
            console.log(value)
            setStakeList(value)
            if (value.length > 0) {
                const stakePool = value[0]
                setStakeAddress(stakePool.bitcoin_address)
                setStakeCanister(stakePool.staking_pool_canister)
                setStakepoolCanister(stakePool.staking_pool_canister.toText())
            }
            setIsLoading(false)
        })
    }
    function get_wallets() {
        if (!osBackend) return;
        setIsLoading(true)
        osBackend.my_wallets().then((value: WalletInfo[]) => {
            setWalletList(value);
            setIsLoading(false)
        })
    }
    function get_wallets_metadata() {
        if (!walletBackend) return;
        walletBackend.metadata().then((value) => {
            if ('Ok' in value) {
                setWalletMetadata(value.Ok)
            }
        })
        console.log("------------------!!!")
        console.log(walletMetadata)
    }

    function get_wallet_count() {
        if (!osBackend) return;
        setIsLoading(true)
        osBackend.count_wallet().then((value: BigInt) => {
            setUsers(Number(value));
            setIsLoading(false)
        })
    }

    function get_tvl() {
        if (!stakeBackend) return;
        // if(wallet.length <=1) return;
        setIsLoading(true);
        // walletBackend.metadata().then((value) => {
        //     console.log(value);
        // })
        stakeBackend.tvl().then((v: BigInt) => {
            setTvl(Number(v) * 1.0 / btc)
            setIsLoading(false);
        })
    }

    async function updateWalletData(addr: string) {
        if (!walletBackend) return;

        // 初始化加载状态
        setIsLoading(true);

        // 初始化状态值
        if (!addr || addr.length < 1) {
            setBalance(0);
            setStakeRecords([]);
            setIsLoading(false);
            return;
        }

        try {

            // 并行调用 get_balance 和 get_stake_records
            const [balanceResult, stakeRecordsResult] = await Promise.all([
                walletBackend.balance(addr),
                walletBackend.list_staking()
            ]);

            // 处理余额结果
            if ('Err' in balanceResult) {
                toast({
                    title: 'Balance',
                    description: "get balance error",
                    status: 'error',
                    position: 'top',
                    duration: 9000,
                    isClosable: true,
                });
            } else {
                const b: bigint = balanceResult.Ok;
                setBalance(Number(b) / btc);
            }

            // 处理 staking 记录结果
            if ('Ok' in stakeRecordsResult) {
                const records: StakingRecord[] = stakeRecordsResult.Ok;
                setStakeRecords(records);
                let r: bigint = 0n;
                records.forEach((record) => {
                    r += record.sent_amount;
                });
                setTotalBalance(Number(r) * 1.0 / btc);
            }

        } catch (error) {
            console.error('Error updating wallet data:', error);
            toast({
                title: 'Error',
                description: "Error updating wallet data",
                status: 'error',
                position: 'top',
                duration: 9000,
                isClosable: true,
            });
        } finally {
            setIsLoading(false);
        }
    }
    async function get_stake_records(addr: string) {
        if (!walletBackend) return;
        if (!addr || addr.length <= 1) {
            setStakeRecords([]);
            return;
        }
        setIsLoading(true);

        try {
            const v: StakingRecords = await walletBackend.list_staking();
            if ('Ok' in v) {
                const records: StakingRecord[] = v.Ok;
                setStakeRecords(records);
                let r: bigint = 0n;
                records.forEach((record) => {
                    r += record.sent_amount;
                });
                setTotalBalance(Number(r) * 1.0 / btc);
            }
        } catch (error) {
            console.error('Error getting stake records:', error);
        } finally {
            setIsLoading(false);
        }
    }

    async function get_balance(addr: string) {
        if (!walletBackend) return;
        // if(wallet.length <=1) return;
        setIsLoading(true);
        if (!addr || addr.length < 1) {
            setBalance(0);
            setStakeRecords([]);
            setIsLoading(false);
            return;
        }

        try {
            const value: BalanceResult = await walletBackend.balance(addr);
            if ('Err' in value) {
                toast({
                    title: 'Balance',
                    description: "get balance error",
                    status: 'error',
                    position: "top",
                    duration: 9000,
                    isClosable: true,
                });
            } else {
                console.log("-------------111")
                const b: bigint = value.Ok;
                setBalance(Number(b) / btc);
                console.log(addr)
                console.log(balance)
                console.log(Number(b) / btc)
            }
        } catch (error) {
            console.error('Error getting balance:', error);
            toast({
                title: 'Balance',
                description: "get balance error",
                status: 'error',
                position: "top",
                duration: 9000,
                isClosable: true,
            });
        } finally {
            setIsLoading(false);
        }
    }
    async function get_stakebalance(addr: string) {
        if (!walletBackend) return;
        setIsLoading(true);
        if (!addr || addr.length < 1) {
            setBalance(0);
            setStakeRecords([]);
            setIsLoading(false);
            return;
        }
        try {
            const stakeRequest: TotalStakingRequest = {
                'sender_address': addr,
                'staking_canister': stakeCanister,
            }
            const value = await walletBackend.total_staking(stakeRequest);
            if ('Err' in value) {
                toast({
                    title: 'Balance',
                    description: "get balance error",
                    status: 'error',
                    position: "top",
                    duration: 9000,
                    isClosable: true,
                });
            } else {
                const b: bigint = value.Ok;
                setIsstakeBalance(Number(b) / btc);
                console.log("---------------nnn")
                console.log(b)
            }
        } catch (error) {
            console.error('Error getting balance:', error);
            toast({
                title: 'Balance',
                description: "get balance error",
                status: 'error',
                position: "top",
                duration: 9000,
                isClosable: true,
            });
        } finally {
            setIsLoading(false);
        }
    }
    async function get_wallet_utxos(addr: string) {
        if (!walletBackend) return;
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
            const value: utxosRecords[] = await walletBackend.utxos(stakeRequest);
            if ('Err' in value) {
                toast({
                    title: 'utxo',
                    description: "get balance error",
                    status: 'error',
                    position: "top",
                    duration: 9000,
                    isClosable: true,
                });
            } else {
                console.log("-------mmm", value)
                if ('Ok' in value) {
                    const result = value.Ok as UtxosResponse[];
                    setWalletUtxos(result)
                }
                console.log("--------------------```")
                console.log(walletUtxos)
            }
        } catch (error) {
            console.error('Error getting utxos:', error);
            toast({
                title: 'Balance',
                description: "get utxos error",
                status: 'error',
                position: "top",
                duration: 9000,
                isClosable: true,
            });
        } finally {
            setIsLoading(false);
        }
    }
    function get_stake_balance() {
        if (!osBackend) return;
        osBackend.my_wallets().then((value: WalletInfo[]) => {
            setWalletList(value);
        })
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
    function transfer_balance() {
        if (!walletBackend) return
        setIsLoading(true)
        const TransferInfo: TransferRequest = {
            txs: [
                {
                    recipient: transferAddress,
                    amount: BigInt(transferBalance * btc)
                }
            ]
        }
        walletBackend.transfer_from_p2pkh(TransferInfo).then((result) => {
            if ('Err' in result) {
                toast({
                    title: 'Transfer',
                    description: 'transfer balance error',
                    status: 'error',
                    duration: 9000,
                    isClosable: true,
                })
            } else {
                toast({
                    title: 'Transfer',
                    status: 'success',
                    position: "top",
                    duration: 9000,
                    isClosable: true,
                    render: () => (
                        <Box color='white' p={3} bg='green.500'>
                            <Text>transfer balance success</Text>
                            <Text>{"txid:" + result.Ok}</Text>
                        </Box>
                    )
                })
            }
            refresh()
            setTransferBalance(0)
            setIsLoading(false);
        })
    }
    function stake_balance() {
        if (!walletBackend) return
        if (!stakeCanister) return
        setIsLoading(true);
        const stakeRequest: StakingRequest = {
            'staking_address': stakeAddress,
            'staking_canister': stakeCanister,
            'amount': BigInt(stakeBalance * btc),
        }
        walletBackend.staking_to_pool(stakeRequest).then((result: StakeResult) => {
            if ('Err' in result) {
                toast({
                    title: 'Stake',
                    description: "stake balance error",
                    status: 'error',
                    position: "top",
                    duration: 9000,
                    isClosable: true,
                })
            } else {
                toast({
                    title: 'Stake',
                    status: 'success',
                    position: "top",
                    duration: 9000,
                    isClosable: true,
                    render: () => (
                        <Box color='white' p={3} bg='green.500'>
                            <Text>stake balance success</Text>
                            <Text>{"txid:" + result.Ok}</Text>
                        </Box>
                    )
                })
            }
            refresh()
            setStakeBalance(0)
            setIsLoading(false);
        })
    }
    function unstake_balance(txid, addr, network) {
        if (!walletBackend) return
        if (!stakeCanister) return
        setIsLoading(true);
        console.log(network)
        const unstakeRequest: RedeemRequest = {
            'txid': txid,
            'recipient': addr,
            'network': network,
        }
        stakeBackend.redeem(unstakeRequest).then((result: RedeemResponse) => {
            if ('Err' in result) {
                toast({
                    title: 'Unstake',
                    description: "unstake balance error",
                    status: 'error',
                    position: "top",
                    duration: 9000,
                    isClosable: true,
                })
            } else {
                toast({
                    title: 'Unstake',
                    status: 'success',
                    position: "top",
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
            refresh()
            setIsLoading(false);
        })
    }
    // function get_owner() {
    //     if (!walletBackend) return;
    //     walletBackend.owner().then((value) => {
    //         console.log(value);
    //     })
    // }
    function onCreateWallet() {
        onCreateClose()
        if (!osBackend || !identity) {
            return
        }
        setIsLoading(true)
        osBackend.create_wallet_canister(walletName).then(
            (v) => {
                get_wallets()
                get_wallet_count()
                setIsLoading(false)
            }, (e) => {
                setIsLoading(false)
            }
        )

    }
    function refresh() {

        get_wallets()
        get_balance(wallet)
        get_tvl()
        get_stake_records(wallet)
        get_wallet_count()
    }
    function onStake() {
        stake_balance()

    }
    function sub(s: string) {
        const l = s.length
        return s.substring(0, 3) + "..." + s.substring(l - 3, l);
    }
    function test() {
        const updatedWalletList = walletList.map(wallet => {
            if (wallet.bitcoin_address === 'mpHDyVUSXuKkyySbU2mrG1GK1nauEiqjuo') {
                return { ...wallet, balance: 1 };
            }
            return wallet;
        });
        setWalletList(updatedWalletList);
        console.log(walletList)
    }
    const formatDate = (bigintTimestamp) => {
        const date = new Date(Number(bigintTimestamp / 1000000n)); // Assuming the timestamp is in nanoseconds, convert to milliseconds
        return date.toLocaleString();
    };
    return (
        <>
            <Flex direction='column' ml='20%' minWidth='1500px'>
                {isLoading &&
                    <Flex zIndex={999999} height="100%" bg="#000" opacity="0.5" width="100%" position="fixed" align="center" justifyContent="center" top={0} left={0}>
                        <Spinner color='purple.500' size="xl" speed="0.65s"></Spinner>
                    </Flex>}
                <Flex mt={6} direction='row'>
                    <Flex direction='column'>
                        <Text>
                            <Heading>bitcoinOS</Heading>  A Decentralized Bitcoin Finance & Assets Management Platform
                        </Text>
                        <br />
                        <Text mt={2}>
                            <Heading>osBTC</Heading> Earn BTCs And Credits

                        </Text>
                    </Flex>
                    {/* <Flex>
                        <Image src="bitcoinos.jpg"></Image>
                    </Flex> */}
                </Flex>
                <Flex mt={5}>
                    <Button onClick={test}>test</Button>
                    <Text pr={3}>
                        TVL: {tvl}
                    </Text>
                    <Spacer></Spacer>
                    <Text mr="30%">
                        Users: {users}
                    </Text>
                </Flex>
                <Box mt={2} boxShadow="lg" border="1px" borderColor="gray.200" borderRadius="md" mr="30%" p={3} zIndex={4}>

                    <Flex width='100%' mb={4}>
                        <Flex direction='column'>
                            <Flex>
                                <Text mr={2}>Wallets:</Text>
                                <Select onChange={onChangeWallet} mr={10} width="100%">
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
                            {wallet.length > 0 && <Text fontSize='sm' mt="2">address:  {(wallet)}</Text>}
                            {wallet.length > 0 && <Text fontSize='sm' mt="2">canister: {walletSelect.map((item, index) => (
                                item.wallet_canister.toText()
                            ))}</Text>}
                            {wallet.length > 0 && <Text fontSize='sm' mt="2">{walletSelect.map((item, index) => (
                                Object.keys(item.network).map((key) => (
                                    <p key={key}>Network: {key}</p>
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


                    <Flex>
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
                                        <VStack align='left'>
                                            <HStack align='end'>
                                                <Text fontSize='sm'>BTC Balance:{balance}</Text>
                                                <Spacer></Spacer>
                                                <Flex>
                                                    <Text fontSize='sm'>osBTC Balance:{totalBalance}</Text>
                                                    {/* <Button
                                                    bgColor="orange.400"
                                                    color="white"
                                                    size='sm'
                                                    _hover={{ bg: "orange.200", borderColor: "orange.400" }}
                                                    onClick={refresh}>
                                                    <BsBoxArrowUpRight  />
                                                </Button> */}
                                                </Flex>
                                            </HStack>
                                            <HStack bg="gray.200" p={1} borderRadius="lg">

                                                <InputGroup>

                                                    <Input type="string" value={transferAddress} border="none" placeholder='address' isDisabled={!isLogin} onChange={handleChangeTransferAddress}></Input >

                                                </InputGroup>
                                            </HStack>
                                            <HStack bg="gray.200" p={1} borderRadius="lg">
                                                <InputGroup>
                                                    <InputLeftElement
                                                        pointerEvents="none"
                                                    >
                                                        <Image src='./favicon.png' boxSize="1.2rem" />
                                                    </InputLeftElement>

                                                    <Input type="number" value={transferBalance} border="none" placeholder='0.0' isDisabled={!isLogin} onChange={handleChangeTransfer}></Input >

                                                    <InputRightElement  >
                                                        <Button color="orange.300" isDisabled={!isLogin} p={2} fontSize="0.8rem" onClick={onMaxClick}>MAX</Button>
                                                    </InputRightElement>
                                                </InputGroup>
                                            </HStack>
                                            <Text fontSize="0.8rem" color='red'><span>{balanceError}</span></Text>
                                            <Text fontSize='sm'>Exchange Rate 1.00 BTC = 1.00 osBTC</Text>
                                            <Flex width='100%' direction='column' align="center" pt={4}>
                                                {isLogin && <Button height="2.5rem" width="40%" color="white" bgColor="orange.400" _hover={{ bg: "orange.200", borderColor: "orange.400" }} isDisabled={transferBalance <= 0 || !isOsInited} onClick={transfer_balance}>Transfer</Button>}
                                                {!isLogin && <Button height="2.5rem" width="40%" color="white" bgColor="orange.400" _hover={{ bg: "orange.200", borderColor: "orange.400" }}>Login</Button>}
                                            </Flex>
                                        </VStack>
                                    </Flex>
                                </TabPanel>
                                <TabPanel>
                                    <Flex mt={2} justifyContent="center">
                                        <VStack align='left'>
                                            <HStack align='end'>
                                                <Text fontSize='sm'>BTC Balance:{balance}</Text>
                                                <Spacer></Spacer>
                                                <Flex>
                                                    <Text fontSize='sm'>osBTC Balance:{totalBalance}</Text>
                                                    {/* <Button
                                                    bgColor="orange.400"
                                                    color="white"
                                                    size='sm'
                                                    _hover={{ bg: "orange.200", borderColor: "orange.400" }}
                                                    onClick={refresh}>
                                                    <BsBoxArrowUpRight  />
                                                </Button> */}
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

                                                    <InputRightElement  >
                                                        <Button color="orange.300" isDisabled={!isLogin} p={2} fontSize="0.8rem" onClick={onMaxClick}>MAX</Button>
                                                    </InputRightElement>
                                                </InputGroup>
                                            </HStack>
                                            <Text fontSize="0.8rem" color='red'><span>{balanceError}</span></Text>
                                            <Text fontSize='sm'>Exchange Rate 1.00 BTC = 1.00 osBTC</Text>
                                            <Flex width='100%' direction='column' align="center" pt={4}>
                                                {isLogin && <Button height="2.5rem" width="40%" color="white" bgColor="orange.400" _hover={{ bg: "orange.200", borderColor: "orange.400" }} isDisabled={stakeBalance <= 0 || !isOsInited} onClick={onStake}>Stake</Button>}
                                                {!isLogin && <Button height="2.5rem" width="40%" color="white" bgColor="orange.400" _hover={{ bg: "orange.200", borderColor: "orange.400" }}>Login</Button>}
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
                        <Flex>

                            {stakeSelect.length > 0 && (
                                <Box maxWidth="600px" mx="auto">
                                    <Text fontSize="2xl" textAlign="center" mb={4}>
                                        Stake Pool
                                    </Text>
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
                                                </React.Fragment>
                                            ))}
                                        </Tbody>
                                    </Table>
                                </Box>
                            )}
                        </Flex>
                    </Flex>
                </Box >
                <Modal isOpen={isWalletOpen} onClose={onWalletClose} isCentered={true}>
                    <ModalOverlay />
                    <ModalContent maxW="35%">
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
                                {walletUtxos && Array.isArray(walletUtxos.utxos) && walletUtxos.utxos.length > 0 && (
                                    <Table variant='simple' size='sm'>
                                        <Thead>
                                            <Tr>
                                                <Th>Height</Th>
                                                <Th>txid</Th>
                                            </Tr>
                                        </Thead>
                                        <Tbody>
                                            {walletUtxos.utxos.map((item, index) => (
                                                <React.Fragment key={index}>
                                                    <Tr>
                                                        <Td>{item.height}</Td>
                                                        <Td>{item.outpoint.txid}</Td>
                                                    </Tr>
                                                </React.Fragment>
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
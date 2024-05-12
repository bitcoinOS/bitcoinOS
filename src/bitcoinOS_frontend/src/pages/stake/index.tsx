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
    Spinner
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
import { useEffect, useState, useRef } from 'react';
import { WalletStore, StakepoolStore } from "../../store/index"
import { useWalletBackend, Result_1 as BalanceResult, StakingRequest, Result_3 as StakeResult, StakingRecords, StakingRecord } from "../../ic/WalletActors";
import { useOsBackend, WalletInfo, StakingPoolInfo } from "../../ic/OsActors";
import { useSatkePoolBackend } from "../../ic/StakePoolActors";
import { useInternetIdentity } from "ic-use-internet-identity";
import { Principal } from "@dfinity/principal"
export default function Stake() {
    const toast = useToast();
    const { actor: walletBackend } = useWalletBackend();
    const { actor: osBackend } = useOsBackend();
    const { actor: stakeBackend } = useSatkePoolBackend();
    const { identity } = useInternetIdentity();
    const [walletList, setWalletList] = useState<WalletInfo[]>([])
    const [wallet, setWallet] = useState<string>("")
    const [balance, setBalance] = useState<number>(0)

    const [totalBalance, setTotalBalance] = useState<number>(0)

    const [stakeBalance, setStakeBalance] = useState<number>(0)
    const { currentWallet, setCurrentWallet } = WalletStore();
    const { stakepoolCanister, setStakepoolCanister } = StakepoolStore();
    const [balanceError, setBalanceError] = useState<string>("");
    const [isLogin, setIslogin] = useState<boolean>(false)

    const [tvl, setTvl] = useState<number>(0)
    const [users, setUsers] = useState<number>(0)

    const [isWalletInited, setIsWalletInited] = useState<boolean>(false)
    const [isOsInited, setIsOsInited] = useState<boolean>(false)
    const [isStakePoolInited, setIsStakePoolInited] = useState<boolean>(false)
    const { isOpen, onOpen, onClose } = useDisclosure()
    const [walletName, setWalletName] = useState<string>("");
    const [isLoading, setIsLoading] = useState<boolean>(false)
    const [stakeAddress, setStakeAddress] = useState<string>("")
    const [stakeCanister, setStakeCanister] = useState<Principal>();
    const [stakeRecords, setStakeRecords] = useState<StakingRecord[]>([])

    const btc = 100000000
    useEffect(() => {

        if (identity) {
            setIslogin(true)
        }
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
    }, [])

    useEffect(() => {
        if (identity) {
            setIslogin(true)
        } else {
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
        if (identity && walletBackend) {
            get_tvl()
            get_balance(wallet)
            get_stake_balance()
        }
    }, [walletBackend, identity]);

    // useEffect(() => {
    //     get_balance()
    //     get_stake_records()
    // }, [wallet])

    function onChangeWallet(event: React.ChangeEvent<HTMLSelectElement>) {
        setWallet(event.target.value)
        get_balance(event.target.value)
        get_stake_records(event.target.value)
        const selectOption = event.target.selectedOptions[0]
        if (selectOption.dataset.id) {
            setCurrentWallet(selectOption.dataset.id)
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
    }
    function get_stake_pool() {
        if (!osBackend) return;
        setIsLoading(true)
        osBackend.list_staking_pool().then((value: StakingPoolInfo[]) => {
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
    function get_stake_records(addr:string) {
        if (!walletBackend) return;
        if(!addr ||  addr.length <=1) {
            setStakeRecords([])
            return
        };
        setIsLoading(true);

        walletBackend.list_staking().then((v: StakingRecords) => {
            if ('Ok' in v) {
                const records: StakingRecord[] = v.Ok
                setStakeRecords(records)
                let r: bigint = 0n
                records.map((v) => {
                    r = r + (v.sent_amount)
                })
                setTotalBalance(Number(r) * 1.0 / btc)
            }
            setIsLoading(false);
        })
    }

    function get_balance(addr:string) {
        if (!walletBackend) return;
        // if(wallet.length <=1) return;
        setIsLoading(true);
        if(!addr ||  addr.length <1){
            setBalance(0)
            setStakeRecords([])
            setIsLoading(false);
            return ;
        }
        walletBackend.balance(addr).then((value: BalanceResult) => {
            if ('Err' in value) {
                toast({
                    title: 'Balance',
                    description: "get balance error",
                    status: 'error',
                    position: "top",
                    duration: 9000,
                    isClosable: true,
                })
            } else {
                const b: bigint = value.Ok
                setBalance(Number(b) / btc)
            }
            setIsLoading(false);
        })
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
        onClose()
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
    return (
        <>
            <Flex direction='column' ml='20%' >
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
                                <Select onChange={onChangeWallet} mr={10} width="100%" placeholder='Select Wallet'>
                                    {
                                        walletList.map((item, index) => (<option key={index} value={item.bitcoin_address} data-id={item.wallet_canister.toText()}>{item.name}</option>))
                                    }
                                </Select>
                            </Flex>
                            {wallet.length >0 && <Text fontSize='sm' mt="2">{(wallet)}</Text>}
                        </Flex>
                        <Button
                            bgColor="orange.400"
                            color="white"
                            isDisabled={!isLogin || !isOsInited}
                            _hover={{ bg: "orange.200", borderColor: "orange.400" }}
                            onClick={onOpen}
                        >
                            Create Wallet
                        </Button>
                        <Spacer></Spacer>
                        <Button
                            bgColor="orange.400"
                            color="white"
                            isDisabled={!isLogin || !isOsInited}
                            _hover={{ bg: "orange.200", borderColor: "orange.400" }}
                            onClick={refresh}>
                            <BsArrowClockwise />
                        </Button>
                        <Modal
                            isOpen={isOpen}
                            onClose={onClose}
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
                                    <Button color="white" bgColor="gray.500" onClick={onClose}>Cancel</Button>
                                </ModalFooter>
                            </ModalContent>
                        </Modal>
                    </Flex>


                    <Flex>
                        <Tabs>
                            <TabList>
                                <Tab mr={10} _selected={{ color: 'orange.400' }}>Stake</Tab>
                                <Tab mr={10} _selected={{ color: 'orange.400' }}>Unstake</Tab>
                                <Tab mr={10} _selected={{ color: 'orange.400' }}>Detail</Tab>
                            </TabList>

                            <TabPanels>
                                <TabPanel>
                                    <Flex mt={2}>
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
                                    <Text p={4} color="purple.500">Please note that unstaking will come soon</Text>
                                </TabPanel>
                                <TabPanel>
                                    <TableContainer>
                                        <Table variant='simple'>
                                            <Thead>
                                                <Tr>
                                                    <Th>Date</Th>
                                                    <Th>Stake pool</Th>
                                                    <Th> Ammount(BTC) </Th>
                                                </Tr>
                                            </Thead>
                                            <Tbody>
                                                {
                                                    stakeRecords.map((v, i) => (

                                                        <Tr>
                                                            <Td>{new Date(Number(v.sent_time) / 1000000).toDateString()}</Td>
                                                            <Td>{sub(v.staking_address)} </Td>
                                                            <Td >{Number(v.sent_amount) / btc}</Td>
                                                        </Tr>
                                                    ))
                                                }
                                            </Tbody>
                                        </Table>
                                    </TableContainer>
                                </TabPanel>
                            </TabPanels>
                        </Tabs>
                    </Flex>
                </Box>
            </Flex>
        </>
    )
}
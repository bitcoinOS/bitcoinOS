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
import { useToast } from '@chakra-ui/react'
import {
    BsArrowClockwise,
} from 'react-icons/bs'
import { Tabs, TabList, TabPanels, Tab, TabPanel } from "@chakra-ui/react"
import { Select } from "@chakra-ui/react"
import { useEffect, useState, useRef } from 'react';
import WalletStore from "../../store/index"
import { useWalletBackend, Result_1 as BalanceResult, StakingRequest,Result_3 as StakeResult } from "../../ic/WalletActors";
import { useOsBackend, WalletInfo,StakingPoolInfo } from "../../ic/OsActors";
import { useInternetIdentity } from "ic-use-internet-identity";
import { Principal } from "@dfinity/principal"
export default function Stake() {
    const toast = useToast();
    const { actor: walletBackend } = useWalletBackend();
    const { actor: osBackend } = useOsBackend();
    const { identity } = useInternetIdentity();
    const [walletList, setWalletList] = useState<WalletInfo[]>([])
    const [wallet, setWallet] = useState<string>("")
    const [balance, setBalance] = useState<number>(0)

    const [totalBalance, setTotalBalance] = useState<number>(0)

    const [stakeBalance, setStakeBalance] = useState<number>(0)
    const { currentWallet, setCurrentWallet } = WalletStore();
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
    const [stakeAddress,setStakeAddress] = useState<string>("")
    const [stakeCanister,setStakeCanister] = useState<Principal>();


    const btc = 100000000
    useEffect(() => {
        setTvl(100)
        setUsers(30)
        if (identity) {
            setIslogin(true)
        }
        if (!walletBackend) {
            setIsWalletInited(false);
        } else {
            setIsWalletInited(true);
        }
        if (!osBackend) {
            setIsOsInited(false)
        } else {
            setIsOsInited(true)
            get_wallets()

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
        }
    }, [osBackend, identity]);

    useEffect(() => {
        setTotalBalance(11)
        get_balance()
        // setBalance(13)
    }, [wallet])

    function onChangeWallet(event: React.ChangeEvent<HTMLSelectElement>) {
        setWallet(event.target.value)
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
    function get_stake_pool(){
        if (!osBackend) return;
        setIsLoading(true)
        osBackend.list_staking_pool().then((value: StakingPoolInfo[]) => {
            if(value.length >0){
                const stakePool = value[0]
                setStakeAddress(stakePool.bitcoin_address)
                setStakeCanister(stakePool.staking_pool_canister)
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
    function get_balance() {
        if (!walletBackend) return;
        if(wallet.length <=1) return;
        setIsLoading(true);
        // walletBackend.metadata().then((value) => {
        //     console.log(value);
        // })
        walletBackend.balance(wallet).then((value: BalanceResult) => {
            if(value.Err){
                toast({
                    title: 'Balance',
                    description: "get balance error",
                    status: 'error',
                    position:"top",
                    duration: 9000,
                    isClosable: true,
                  })
            }else{
                const b:bigint = value.Ok
                setBalance(Number(b)/btc)
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
    function stake_balance() {
        if (!walletBackend) return
        if(!stakeCanister) return
        setIsLoading(true);
        const stakeRequest: StakingRequest = {
            'staking_address': stakeAddress,
            'staking_canister':  stakeCanister,
            'amount': BigInt(stakeBalance * btc),
        }
        walletBackend.staking_to_pool(stakeRequest).then((result:StakeResult)=>{
            if(result.Err){
                toast({
                    title: 'Stake',
                    description: "stake balance error",
                    status: 'error',
                    position:"top",
                    duration: 9000,
                    isClosable: true,
                  })
            }else{
                toast({
                    title: 'Stake',
                    status: 'success',
                    position:"top",
                    duration: 9000,
                    isClosable: true,
                    render: () => (
                        <Box color='white' p={3} bg='green.500'>
                          <Text>stake balance success</Text>
                          <Text>{"txid:"+result.Ok}</Text>
                        </Box>
                      )
                  })
            }
            refresh()
            setIsLoading(false);
        })
    }
    function get_owner() {
        if (!walletBackend) return;
        walletBackend.owner().then((value) => {
            console.log(value);
        })
    }
    function onCreateWallet() {
        onClose()
        if (!osBackend || !identity) {
            return
        }
        setIsLoading(true)
        osBackend.create_wallet_canister(walletName).then(
            (v) => {
                get_wallets()
                setIsLoading(false)
            }, (e) => {
                setIsLoading(false)
            }
        )

    }
    function refresh() {
        
        get_wallets()
        get_balance()
        // get_owner()
         
    }
    function onStake(){
        stake_balance()
        
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
                        <Heading>bitcoinOS</Heading>  A Decentralized Multi-chain Bitcoin Assets Management System
                    </Text>
                    <Text mt={2}>
                        <Heading>osBTC</Heading> Earn BTC  And Secure OS Points

                    </Text>
                    </Flex>
                    <Flex>
                        <Image src="bitcoinos.jpg"></Image>
                    </Flex>
                </Flex>
                <Flex mt={5}>
                    <Text pr={3}>
                        TVL: ${tvl}
                    </Text>
                    <Spacer></Spacer>
                    <Text mr="30%">
                        Users: {users}
                    </Text>
                </Flex>
                <Box mt={2} boxShadow="lg" border="1px" borderColor="gray.200" borderRadius="md" mr="30%" p={3} zIndex={4}>

                    <Flex width='100%' mb={4}>
                        <Text mr={2}>Wallets:</Text>
                        <Select onChange={onChangeWallet} mr={10} width="30%" placeholder='Select Wallet'>
                            {
                                walletList.map((item, index) => (<option key={index} value={item.bitcoin_address} data-id={item.wallet_canister.toText()}>{item.name}</option>))
                            }
                        </Select>
                        <Button
                            bgColor="purple.500"
                            color="white"
                            isDisabled={!isLogin || !isOsInited}
                            _hover={{ bg: "purple.300", borderColor: "purple.500" }}
                            onClick={onOpen}
                        >
                            Create Wallet
                        </Button>
                        <Spacer></Spacer>
                        <Button
                            bgColor="purple.500"
                            color="white"
                            isDisabled={!isLogin || !isOsInited}
                            _hover={{ bg: "purple.300", borderColor: "purple.500" }}
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
                                <Tab mr={10}>Stake</Tab>
                                <Tab mr={10}>Unstake</Tab>
                                <Tab mr={10}>Detail</Tab>
                            </TabList>

                            <TabPanels>
                                <TabPanel>
                                    <Flex mt={2}>
                                        <VStack align='left'>
                                            <HStack align='end'>
                                                <Text fontSize='sm'>BTC Balance:{balance}</Text>
                                                <Spacer></Spacer>
                                                <Text fontSize='sm'>osBTC Balance:{totalBalance}</Text>
                                            </HStack>
                                            <HStack bg="gray.200" p={1} borderRadius="lg">
                                                <InputGroup>
                                                    <InputLeftElement
                                                        pointerEvents="none"
                                                    >
                                                        <Image src='https://res1.sft-api.com/token/WBTC.png' boxSize="1.2rem" />
                                                    </InputLeftElement>
                                                    <Input type="number" value={stakeBalance} border="none" placeholder='0.0' isDisabled={!isLogin} onChange={handleChangeStake}></Input >

                                                    <InputRightElement  >
                                                        <Button color="purple.500" isDisabled={!isLogin} p={2} fontSize="0.8rem" onClick={onMaxClick}>MAX</Button>
                                                    </InputRightElement>
                                                </InputGroup>
                                            </HStack>
                                            <Text fontSize="0.8rem" color='red'><span>{balanceError}</span></Text>
                                            <Text fontSize='sm'>Exchange Rate 1.00 BTC = 1.00 osBTC</Text>
                                            <Flex width='100%' direction='column' align="center" pt={4}>
                                                {isLogin && <Button height="2.5rem" width="40%" color="white" bgColor="purple.500" _hover={{ bg: "purple.300", borderColor: "purple.500" }} isDisabled={stakeBalance <= 0 || !isOsInited} onClick={onStake}>Stake</Button>}
                                                {!isLogin && <Button height="2.5rem" width="40%" color="white" bgColor="purple.500" _hover={{ bg: "purple.300", borderColor: "purple.500" }}>Login</Button>}
                                            </Flex>
                                        </VStack>
                                    </Flex>
                                </TabPanel>
                                <TabPanel>
                                    <Text p={4} color="purple.500">Please note that unstaking will come soon</Text>
                                </TabPanel>
                                <TabPanel>
                                    <p>Please note that detail will come soon</p>
                                </TabPanel>
                            </TabPanels>
                        </Tabs>
                    </Flex>
                </Box>
            </Flex>
        </>
    )
}
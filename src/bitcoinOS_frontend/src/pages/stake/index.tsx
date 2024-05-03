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
import { Tabs, TabList, TabPanels, Tab, TabPanel } from "@chakra-ui/react"
import { Select } from "@chakra-ui/react"
import { useEffect, useState, useRef } from 'react';
// import UserStore from "../../store/index"
import { useWalletBackend } from "../../ic/WalletActors";
import { useOsBackend } from "../../ic/OsActors";
import { useInternetIdentity } from "ic-use-internet-identity";

export default function Stake() {
    const { actor: walletBackend } = useWalletBackend();
    const { actor: osBackend } = useOsBackend();
    const { identity } = useInternetIdentity();
    const [walletList, setWalletList] = useState<string[]>([])
    const [wallet, setWallet] = useState<string>("")
    const [balance, setBalance] = useState<number>(0)
    const [totalBalance, setTotalBalance] = useState<number>(0)
    const [stakeBalance, setStakeBalance] = useState<number>(0)
    // const { principal, setPrincipal } = UserStore();
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
    useEffect(() => {
        setTvl(100)
        setUsers(30)
        setBalance(12)
        if (identity) {
            setIslogin(true)
        }
        if (!walletBackend) {
            setIsWalletInited(false);
            return;
        } else {
            setIsWalletInited(true);
        }
        if(!osBackend){
            setIsOsInited(false)
        }else{
            setIsOsInited(true)
            // osBackend.l
        }

        const data = ['1','2','3']
        setWalletList(data);
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
        if (!walletBackend) {
            setIsWalletInited(false);
        } else {
            setIsWalletInited(true);
        }
        if (identity && walletBackend) {
            walletBackend.counter().then((c: bigint) => {
                debugger
                // setBalance(parseFloat(c.toString()))
            });
        }
    }, [walletBackend, identity]);

    useEffect(() => {
        setTotalBalance(11)
        setBalance(13)
    }, [wallet])

    function onChangeWallet(event: React.ChangeEvent<HTMLSelectElement>) {
        setWallet(event.target.value)

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
    function onCreateWallet() {
        onClose()
        if (!osBackend || !identity){
            return 
        }
        setIsLoading(true)
        osBackend.create_wallet_canister(walletName).then(
            (v)=>{
                setIsLoading(false)
            },(e)=>{
                setIsLoading(false)
            }
        )

    }
    return (
        <>
            <Flex direction='column' ml='20%' mr="20%">
               {isLoading && 
               <Flex zIndex={999999} height="100%" bg="#000" opacity ="0.5"  width="100%" position="fixed" align="center" justifyContent="center" top={0} left={0}>
                <Spinner color='purple.500'  size="xl" speed="0.65s"></Spinner>
               </Flex>}                                                                                 
                <Flex mt={6} direction='column'>
                    <Text>
                        <Heading>BitcoinOS</Heading>  An Asset Management System Based  On RGB  And  ICP
                    </Text>
                    <Text mt={2}>
                        <Heading>OSBTC</Heading> Earn BTC  And Secure OS Points

                    </Text>
                </Flex>
                <Flex mt={5}>
                    <Text pr={3}>
                        TVL: ${tvl}
                    </Text>

                    <Text>
                        Users: {users}
                    </Text>
                </Flex>
                <Box mt={2} boxShadow="lg" border="1px" borderColor="gray.200" borderRadius="md" mr="30%" p={3} zIndex={4}>

                    <Flex width='100%' mb={4}>
                        <Text mr={2}>Wallets:</Text>
                        <Select onChange={onChangeWallet} mr={10} width="30%">
                            {
                                walletList.map((item, index) => (<option key={index} value={item}>{item}</option>))
                            }
                        </Select>
                        <Button
                            bgColor="purple.500"
                            color="white"
                            isDisabled={!isLogin || !isWalletInited}
                            _hover={{ bg: "purple.300", borderColor: "purple.500" }}
                            onClick={onOpen}
                        >
                            Create Wallet
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
                                        <Input  placeholder="wallet name"   onChange={event => setWalletName(event.currentTarget.value)}/>
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
                                    <Button color="white"  bgColor="gray.500" onClick={onClose}>Cancel</Button>
                                </ModalFooter>
                            </ModalContent>
                        </Modal>
                    </Flex>


                    <Flex>
                        <Tabs>
                            <TabList>
                                <Tab mr={10}>Stake</Tab>
                                <Tab mr={10}>Unstake</Tab>
                                {/* <Tab mr={10}>Detail</Tab> */}
                            </TabList>

                            <TabPanels>
                                <TabPanel>
                                    <Flex mt={2}>
                                        <VStack align='left'>
                                            <HStack align='end'>
                                                <Text fontSize='sm'>BTC Balance:{balance}</Text>
                                                <Spacer></Spacer>
                                                <Text fontSize='sm'>OSBTC Balance:{totalBalance}</Text>
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
                                            <Text fontSize='sm'>Exchange Rate 1.00 BTC = 1.00 OSBTC</Text>
                                            <Flex width='100%' direction='column' align="center" pt={4}>
                                                {isLogin && <Button height="2.5rem" width="40%" color="white" bgColor="purple.500" _hover={{ bg: "purple.300", borderColor: "purple.500" }} isDisabled={stakeBalance <= 0}>Stake</Button>}
                                                {!isLogin && <Button height="2.5rem" width="40%" color="white" bgColor="purple.500" _hover={{ bg: "purple.300", borderColor: "purple.500" }}>Login</Button>}
                                            </Flex>
                                        </VStack>
                                    </Flex>
                                </TabPanel>
                                <TabPanel>
                                    <Text p={4} color="purple.500">Please note that unstaking will come soon</Text>
                                </TabPanel>
                                {/* <TabPanel>
                                    <p>Three!</p>
                                </TabPanel> */}
                            </TabPanels>
                        </Tabs>
                    </Flex>
                </Box>
            </Flex>
        </>
    )
}
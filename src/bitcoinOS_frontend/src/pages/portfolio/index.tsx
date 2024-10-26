import {
    Flex,
    Heading,
    Text,
    useToast,
    Button,
    Image,
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
    Input,
    InputGroup,
    InputRightElement,
    InputLeftElement,
    Select,
    Spinner,
    IconButton,
    Avatar,
    Box,
    Divider,
    Tag,
} from '@chakra-ui/react'
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
import QRCode from 'qrcode.react';
import { Skeleton, SkeletonCircle, SkeletonText } from '@chakra-ui/react'
import { BsPlusLg, BsSend, BsArrowClockwise, BsCopy, BsArrowDownUp } from "react-icons/bs";
import { usePointBackend, Metadata } from "../../ic/PointActors";
import { StakingRecord, useWalletBackend } from '../../ic/WalletActors';
import { usestakeBackend } from '../../ic/StakeActors';

import { useInternetIdentity } from "ic-use-internet-identity";
import { useSiwbIdentity } from 'ic-use-siwb-identity';

import { useEffect, useState } from 'react';
import { truncateMiddle, satoshisToBTC, formatDateminute } from '../../utils/utils'
import { CurrentStatus, getNetworkInfo, useStakeListStore } from '../../store';
import { usePointStore } from '../../store/useStakePool';
import { checkIdentityExpiration } from '../../utils/utils';
import { useConnectStore } from '../../store/useConnectStore';
import { WalletStore, IcpWalletStore } from '../../store/useWalletStore';
import { usePoolrecordStore } from '../../store/useStakePool';
import { userStore } from "../../store/useMarathonStore";

import useGetStakePool from '../../utils/poolActor'
import useGetWalletPool from '../../utils/walletActor';
import { useOsBackend } from '../../ic/OsActors';
import { RedeemRequest, useSatkePoolBackend } from '../../ic/StakePoolActors';
import { CreateWalletModal } from '../../components/CreateModal';
import { OgStakeModal } from '../../components/Modal/OgStakeModal';
import { UserModal } from '../../components/Modal/UserModal';

import IcpModal from '../../components/ConnectModal/IcpModal';
import IcpList from '../../components/ConnectModal/IcpList';
import IcpConnectButton from '../../components/marathon/IcpConnectButton'

import { ICPLogoIcon, } from '../../components/ImageIcon/ImageIcon';

import { idlFactory } from '../../components/ConnectModal/nft.did.js';

export default function Portfolio() {
    const [currentAccount, setCurrentAccount] = useConnectStore((state) => [state.currentAccount, state.setCurrentAccount])

    const { isLoading, setIsLoading } = CurrentStatus()

    const toast = useToast();
    const { actor: pointBackend } = usePointBackend();
    const { actor: walletBackend } = useWalletBackend();
    const { actor: osBackend } = useOsBackend();
    const { actor: stakeBackend } = useSatkePoolBackend();
    const { actor: stakeogBackend } = usestakeBackend();
    const { identity, login, clear } = useInternetIdentity();
    const { identity: siwb_identity, identityAddress, clear: siwb_clear } = useSiwbIdentity();

    const [isLogin, setIslogin] = useState<boolean>(false);
    const [isPointInited, setIsPointInited] = useState<boolean>(false);
    const [isOsInited, setIsOsInited] = useState<boolean>(false)
    const [isStakePoolInited, setIsStakePoolInited] = useState<boolean>(false)
    const [isStakeInited, setIsStakeInited] = useState<boolean>(false)
    const [isWalletInited, setIsWalletInited] = useState<boolean>(false)

    const { btcprice } = getNetworkInfo()
    const { pointRank, setPointRank } = usePointStore()
    const [firstGetrecords, setFirstrecords] = useState<boolean>(false)
    const [firstGetbalance, setFirstGetbalance] = useState<boolean>(false)

    // User Info
    const { userInfo } = userStore();

    // Wallet Info
    const [walletName, setWalletName] = useState<string>("");
    const [createWallet, setCreateWallet] = useState<boolean>(false)
    const { balance, walletList, walletSelect, currentWallet, wallet, setWalletList, setWalletSelect, setCurrentWallet, totalBalance, setTotalBalance } = WalletStore();
    const { IcpWallet, BindIcpWallet } = IcpWalletStore()

    const { isOpen: isCreateOpen, onOpen: onCreateOpen, onClose: onCreateClose } = useDisclosure();
    // first create wallet
    const { isOpen: isfirstCreateOpen, onOpen: onfirstCreateOpen, onClose: onfirstCreateClose } = useDisclosure();

    const [showICP, setShowICP] = useState<boolean>(true)
    // Transfer Info
    /*--- transfer Info ---*/
    const [btcFee, setBtcFee] = useState<number>(0)
    const [transferSubmit, setTransferSubmit] = useState<boolean>(false)
    const [btcUnit, setBtcUnit] = useState('btc');
    const [btc, setBtc] = useState(100000000); // starting value
    const [balanceError, setBalanceError] = useState<string>("");
    const [transferBalance, setTransferBalance] = useState<number>(0)
    const [transferAddress, setTransferAddress] = useState<string>("")
    const { isOpen: isTransferOpen, onOpen: onTransferOpen, onClose: onTransferClose } = useDisclosure();
    const { get_wallets, get_balance, get_icp_bind, get_icp_nft, stake_in_nft, get_user_stakednft_point, bind_icpWallet, transfer_balance, onRefresh_balance } = useGetWalletPool()

    // Unstake Info
    const { isOpen: isUnstakeOpen, onOpen: onUnstakeOpen, onClose: onUnstakeClose } = useDisclosure();
    const { currentPool, setCurrentPool } = useStakeListStore()
    // Pool Records Info
    const [retryCount, setRetryCount] = useState(0);

    const { updateWalletData, get_stake_records, unstake_balance, get_stake_pool, get_btc_fee, get_btc_price } = useGetStakePool()
    const { stakeRecords, setStakeRecords, allStakeRecords, setAllStakeRecords } = usePoolrecordStore();
    const stakeList = useStakeListStore((state) => state.stakeList);
    // Deposit Info
    const [isStake, setIsStake] = useState<boolean>(true)

    const { isOpen: isDepositOpen, onOpen: onDepositOpen, onClose: onDepositClose } = useDisclosure();

    const { isOpen: isOpenIcpList, onOpen: onOpenIcpList, onClose: onCloseIcpList } = useDisclosure()
    const { isOpen: isOpenOgStake, onOpen: onOpenOgStake, onClose: onCloseOgStake } = useDisclosure()
    const { isOpen: isOpenUserinfo, onOpen: onOpenUserinfo, onClose: onCloseUserinfo } = useDisclosure()

    const btcunity = 100000000;
    // bigint type
    //const btcunity = 100000000n;

    useEffect(() => {
        setFirstGetbalance(false)
        //get_btc_price()
        if (identity) {
            setIslogin(true);
        }
        if (!osBackend) {
            setIsOsInited(false)
        } else {
            setIsOsInited(true)
            // get_wallets(osBackend)
        }
        if (!pointBackend) {
            setIsPointInited(false);
        } else {
            setIsPointInited(true);
        }
        if (!stakeBackend) {
            setIsStakePoolInited(false);
        } else {
            //get_tvl();
            setIsStakePoolInited(true);
        }
        if (!stakeogBackend) {
            setIsStakeInited(false);
        } else {
            //get_tvl();
            setIsStakeInited(true);
        }
    }, []);
    useEffect(() => {
        if (!osBackend) {
            setIsOsInited(false)
        } else {
            setIsOsInited(true)
        }
    }, [osBackend])
    useEffect(() => {
        if (identity) {
            setIslogin(true);
        } else {
            setIslogin(false);
            setFirstGetbalance(false)
        }
    }, [identity]);

    useEffect(() => {
        if (stakeBackend) {
            setIsStakePoolInited(true);
        } else {
            setIsStakePoolInited(false);
        }
    }, [stakeBackend]);

    useEffect(() => {
        if (!walletBackend) {
            setIsWalletInited(false)
        } else {
            setIsWalletInited(true)
        }
    }, [walletBackend])

    useEffect(() => {
        if (pointBackend) {
            setIsPointInited(true);
        } else {
            setIsPointInited(false);
        }
    }, [pointBackend]);

    useEffect(() => {
        if (stakeogBackend) {
            setIsStakeInited(true);
        } else {
            setIsStakeInited(false);
        }
    }, [stakeogBackend]);
    //-------------
    // first to load stake pool info
    //-------------
    useEffect(() => {
        // debugger
        if (osBackend) {
            setIsOsInited(true)
        }

        if (osBackend) {
            if (identity || siwb_identity) {
                get_icp_bind(osBackend);
            }
        }

        if (identity && osBackend && !wallet) {
            get_wallets(osBackend);
        }
        // create wallet if no wallet ---
        // if (identity) {
        //     if (walletList && walletList.length === 0) {
        //         onfirstCreateOpen()
        //     } else {
        //         onfirstCreateClose()
        //     }
        // }
        if (osBackend && stakeList.length === 0) {
            get_stake_pool(osBackend)
        }
    }, [osBackend, identity]);

    //-------------
    // first to get stake records
    //-------------
    useEffect(() => {
        if (retryCount < 5 && stakeRecords.length === 0) {
            if (!currentPool || !currentPool.staking_pool_canister) {
                if (stakeList && stakeList.length > 0) {
                    setCurrentPool(stakeList[0]);
                    if (currentAccount && currentAccount.type === 'WIZZ') {
                        get_stake_records(stakeList[0].staking_pool_canister.toText(), currentAccount.address)
                    }
                    if (currentAccount && currentAccount.type === 'INTERNET_IDENTITY' && walletSelect && walletSelect.bitcoin_address) {
                        get_stake_records(stakeList[0].staking_pool_canister.toText(), walletSelect.bitcoin_address)
                    }
                    if (currentAccount && currentAccount.type === 'UNISAT') {
                        get_stake_records(stakeList[0].staking_pool_canister.toText(), currentAccount.address)
                    }
                    setRetryCount(prevRetryCount => prevRetryCount + 1);
                    return;
                } else {
                    console.log("stakeList is empty");
                    return;
                }
            }
            if (currentAccount && currentAccount.type === 'WIZZ') {
                get_stake_records(currentPool.staking_pool_canister.toText(), currentAccount.address)
            }
            if (currentAccount && currentAccount.type === 'INTERNET_IDENTITY' && walletSelect && walletSelect.bitcoin_address) {
                get_stake_records(currentPool.staking_pool_canister.toText(), walletSelect.bitcoin_address)
            }
            if (currentAccount && currentAccount.type === 'UNISAT') {
                get_stake_records(currentPool.staking_pool_canister.toText(), currentAccount.address)
            }
            setRetryCount(prevRetryCount => prevRetryCount + 1);
        }
    }, [currentAccount, stakeList]);

    //-------------
    // first to get wallet balance
    //-------------
    useEffect(() => {
        if (walletBackend && walletSelect && walletSelect.bitcoin_address && !firstGetbalance) {
            get_balance(walletBackend, walletSelect.bitcoin_address)
            setFirstGetbalance(true)
        }
    }, [walletBackend, walletSelect]);

    async function get_fee() {
        const fee = await get_btc_fee(walletBackend)
        setBtcFee(fee)
    }
    //-------------
    // ii wallet create new function
    //-------------
    const onCreateWallet = async () => {
        console.log(osBackend)
        if (!osBackend || !identity) {
            return
        }
        setIsLoading(true)
        try {
            const result = await osBackend.create_wallet_canister(walletName)

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
            setCreateWallet(false)
            //get_wallet_count()
            onCreateClose()
            setIsLoading(false)
        } catch (error) {
            console.log(error)
            setIsLoading(false)
            onCreateClose()
            setCreateWallet(false)
        } finally {
            setIsLoading(false)
            onCreateClose()
            setCreateWallet(false)
            console.log('end')
        }


    }
    //-------------
    // wallet select function
    //-------------
    const selectWallet = async (value) => {
        if (isLoading) return;
        setIsLoading(true)
        setCurrentWallet(value.wallet_canister.toText())
        setWalletSelect(value)
        await get_balance(walletBackend, value.bitcoin_address)
        await get_stake_records(currentPool.staking_pool_canister.toText(), walletSelect.bitcoin_address)
        setIsLoading(false)
        onCreateClose()
    }
    const handleSelectChange = async (event) => {
        const selectedValue = event.target.value;
        const selectedStake = stakeList.find(stake => stake.staking_pool_canister.toText() === selectedValue);
        setCurrentPool(selectedStake);
        if (currentAccount && currentAccount.type === 'INTERNET_IDENTITY') {
            await get_stake_records(selectedStake.staking_pool_canister.toText(), walletSelect.bitcoin_address)
        } else {
            await get_stake_records(selectedStake.staking_pool_canister.toText(), currentAccount.address)
        }
    };

    //-------------
    // wallet transfer limit
    //-------------
    function handleChangeTransfer(event: React.ChangeEvent<HTMLInputElement>) {
        const value = parseFloat(event.target.value)
        if (value >= balance) {
            setBalanceError("*BTC balance is insufficient ")
        } else {
            setTransferBalance(parseFloat(event.target.value))
            setBalanceError("")
        }
    }
    //-------------
    // wallet transfer function
    //-------------
    const transferBlanace = async () => {
        try {
            const result = await transfer_balance(walletBackend, transferAddress, transferBalance, btc);
            onTransferClose();
        } catch (error) {
            console.error('Error in transferBlanace:', error);
        }
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
    function onMaxClick() {
        setTransferBalance(balance)
    }

    const getIcpWallet = () => {
        if (BindIcpWallet) {
            return BindIcpWallet;
        } else if (IcpWallet) {
            return IcpWallet;
        } else {
            return 'ICP Wallet';
        }
    };

    const handleCopy = (text) => {
        // copy address
        if (!currentAccount) return
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

    //refrash page
    const reset = () => {
        if (currentAccount && currentAccount.type === 'INTERNET_IDENTITY') {
            get_wallets(osBackend)
            setCurrentWallet(walletSelect.wallet_canister.toText())
            get_balance(walletBackend, walletSelect.bitcoin_address)
            get_stake_records(currentPool.staking_pool_canister.toText(), walletSelect.bitcoin_address)
        }
        if (currentAccount && currentAccount.type === 'UNISAT') {
            get_stake_records(currentPool.staking_pool_canister.toText(), currentAccount.address)
        }
        if (currentAccount && currentAccount.type === 'WIZZ') {
            get_stake_records(currentPool.staking_pool_canister.toText(), currentAccount.address)
        }
    }

    // unstake action
    const UnstakeOpen = (stake) => {
        setCurrentPool(stake)
        onUnstakeOpen()
    }

    const unstake_action = async () => {
        console.log(currentPool)
        try {
            //const result = await unstake_balance(stakeBackend, currentPool.txid, walletSelect.bitcoin_address, currentPool.network)
            onUnstakeClose()
        } catch (error) {
            console.error('Error in transferBlanace:', error);
        }
    }

    const test = async () => {
        //console.log(stakeogBackend)
        //const a = await stake_in_nft(stakeogBackend, 0)
        const a = await get_user_stakednft_point(pointBackend)
    }
    return (
        <>
            <Flex
                direction='column'
                alignItems='center'
                bgGradient="linear(to-b, #F5F7FF 100%, #FFFFFF 100%)"
                minH='100vh'
            >
                <OgStakeModal
                    isStake={isStake}
                    isOpen={isOpenOgStake}
                    onClose={onCloseOgStake}
                    stakeBackend={stakeogBackend}
                    stakeogBackend={stakeogBackend}
                />
                <UserModal
                    isOpen={isOpenUserinfo}
                    onClose={onCloseUserinfo}
                    osBackend={osBackend}
                />
                <IcpModal isOpen={isOpenIcpList} onClose={onCloseIcpList}>
                    <IcpList />
                </IcpModal>
                {(!currentAccount || (currentAccount && currentAccount.type === 'INTERNET_IDENTITY')) ? (
                    <Flex
                        direction='column'
                        width='100%'
                        alignItems='center'
                    >
                        <Flex
                            maxWidth='1120px'
                            width='78%'
                            mt='10'
                            mb='2'
                        >
                            {/*
                            <Button onClick={test}>test</Button>
                            */}
                            <Text>
                                <Heading
                                    fontSize={{ md: '28px', lg: '32px', xl: '35px', '2xl': '42px', '3xl': '44px' }}
                                >Portfolio</Heading>
                            </Text>
                        </Flex>
                        <Flex
                            maxWidth='1120px'
                            width='78%'
                            justifyContent='space-between'
                            py='6'
                        >
                            <Flex>
                                {userInfo && userInfo.user_img && userInfo.user_img.length > 0
                                    ?
                                    (
                                        <Avatar
                                            size='lg'
                                            name='Dan Abrahmov'
                                            src={Array.isArray(userInfo.user_img) ? userInfo.user_img[0] || '' : userInfo.user_img || ''} />
                                    )
                                    :
                                    (
                                        <Avatar
                                            size='lg'
                                            name='Dan Abrahmov'
                                            src='https://white-philosophical-blackbird-295.mypinata.cloud/ipfs/QmP9qg9P9Q4dfWaXzHnbF2qCuyAB45oAWmoimWYaAkxGZ1/1.png' />
                                    )
                                }

                                <Flex
                                    direction='column'
                                    ml='2'
                                >
                                    <Text fontSize='24px' fontWeight='700'>
                                        {userInfo && userInfo.name && userInfo.name.length > 0
                                            ? userInfo.name
                                            : "Default Name"}
                                    </Text>
                                    <Text fontSize='14px' fontWeight='400'>
                                        {currentAccount && currentAccount.address && (
                                            <Text fontSize='14px' fontWeight='400'>
                                                {truncateMiddle(currentAccount.address, 5, 5)}
                                            </Text>
                                        )}
                                    </Text>
                                </Flex>
                            </Flex>
                            <Button
                                width='144px'
                                height='44px'
                                onClick={onOpenUserinfo}
                            >Edit Portfolio</Button>
                        </Flex>
                        <Flex
                            maxWidth='1120px'
                            width='78%'
                            borderRadius='3xl'
                            bgGradient='linear(to-r, #FFE8CC 0%, #F0D9FF 100%)'
                            direction='column'
                            p='6'
                        >
                            <Flex
                                width='100%'
                                mb='6'
                                alignItems='center'
                                justifyContent='space-between'
                            >
                                <Flex>
                                    {walletSelect && walletSelect.bitcoin_address && walletSelect.name ? (
                                        <Flex alignItems='center'>
                                            <Text mr='6' fontWeight='bold' fontSize={{ md: '24px', lg: '24px', xl: '24px', '2xl': '24px', '3xl': '24px' }}>
                                                {walletSelect.name}
                                            </Text>
                                            <Flex
                                                alignItems='center'
                                                width='180px'
                                                height='40px'
                                                borderRadius='8px'
                                                border='1px solid white'
                                                cursor='pointer'
                                                bgColor={!showICP ? '#FFFFFF' : 'transparent'}
                                                _hover={{ opacity: 0.8 }}
                                                onClick={() => setShowICP(false)}
                                            >
                                                <Image src='/marathon/task-1.svg' width='16px' mr='2' />
                                                <Text>{truncateMiddle(walletSelect.bitcoin_address, 5, 5)}</Text>
                                                <Button variant='ghost' onClick={() => handleCopy(walletSelect.bitcoin_address)}>
                                                    <BsCopy size='15px' />
                                                </Button>
                                            </Flex>
                                        </Flex>
                                    ) : (
                                        <Flex><Skeleton height='30px' width='100px' /></Flex>
                                    )}
                                    {/*
                                <Flex>
                                    <Button
                                        colorScheme='#000000'
                                        variant='outline'
                                        onClick={() => { setCreateWallet(false); onCreateOpen() }}
                                        leftIcon={<BsArrowDownUp />}
                                    >Switch Wallet</Button>
                                </Flex>
                                */}
                                    <Flex
                                        pl='2'
                                        ml='3'
                                        alignItems='center'
                                        width='180px'
                                        height='40px'
                                        borderRadius='8px'
                                        border='1px solid white'
                                        cursor='pointer'
                                        bgColor={showICP ? '#FFFFFF' : 'transparent'}
                                        _hover={{ opacity: 0.8 }}
                                        onClick={() => setShowICP(true)}
                                    >
                                        <Box width="20px" ml='2' mr="1">
                                            <ICPLogoIcon />
                                        </Box>
                                        <Text>{getIcpWallet() ? truncateMiddle(getIcpWallet(), 5, 5) : 'ICP Wallet'}</Text>
                                        <Button variant='ghost' onClick={() => handleCopy(getIcpWallet())}>
                                            <BsCopy size='15px' />
                                        </Button>
                                    </Flex>
                                </Flex>
                            </Flex>
                            <Flex
                                width='100%'
                                height='152px'
                                bgColor='white'
                                borderRadius='xl'
                                p='6'
                                justifyContent="space-between"
                            >{showICP ?
                                (
                                    <Flex
                                        width='100%'
                                        justifyContent='space-between'
                                    >
                                        <Flex alignItems='center'>
                                            <Image height='96px' src='/marathon/rewards_og.png' />
                                            <Flex
                                                direction='column'
                                                height='96px'
                                                ml='5'
                                                justifyContent='space-between'
                                            >
                                                <Flex direction='column'>
                                                    <Text fontSize='16px' fontWeight='700' color='gray-700'>BifiPal OG NFT</Text>
                                                    <Text fontSize='14px' fontWeight='400' color='gray-500'>Holding BifiPal OG NFT to get bonus points</Text>
                                                </Flex>

                                                <Flex>
                                                    <Button
                                                        width='104px'
                                                        height='32px'
                                                        bgColor='#FFA033'
                                                        color='black'
                                                        onClick={() => { get_icp_nft(); setIsStake(true); onOpenOgStake() }}
                                                        isDisabled={!IcpWallet || !BindIcpWallet}
                                                    >
                                                        Stake
                                                    </Button>
                                                    <Button
                                                        ml='2'
                                                        width='104px'
                                                        height='32px'
                                                        bgColor='white'
                                                        color='black'
                                                        border='1px solid black'
                                                        onClick={() => { setIsStake(false); onOpenOgStake() }}
                                                        isDisabled={!IcpWallet || !BindIcpWallet}
                                                    >
                                                        Unstake
                                                    </Button>
                                                    <Button
                                                        ml='2'
                                                        width='104px'
                                                        height='32px'
                                                        bgColor='black'
                                                        color='white'
                                                    >
                                                        Buy
                                                    </Button>
                                                </Flex>
                                            </Flex>
                                        </Flex>
                                        <Flex alignItems='center'>
                                            <IcpConnectButton
                                                connectWallet={IcpWallet}
                                                bindWallet={BindIcpWallet}
                                                onOpenIcpList={onOpenIcpList}
                                                onCloseIcpList={onCloseIcpList}
                                                osBackend={osBackend}
                                            />
                                        </Flex>
                                    </Flex>
                                )
                                :
                                (
                                    <Flex width='100%' justifyContent="space-between">
                                        <Flex>
                                            <Flex
                                                direction='column'
                                                mr='20'
                                            >
                                                <Text fontSize='18px'>Balance</Text>
                                                <Text fontSize='28px' fontWeight='bold'>{balance} btc</Text>
                                                {/*
                                        <Text fontSize='18px'>≈${balance * btcprice}</Text>
                                        */}
                                            </Flex>
                                            <Flex
                                                direction='column'
                                                ml='10'
                                            >
                                                <Text fontSize='18px'>Staking</Text>
                                                <Text fontSize='28px' fontWeight='bold'>{totalBalance} btc</Text>
                                                {/*
                                        <Text fontSize='18px'>≈${totalBalance * btcprice}</Text>
                                        */}
                                            </Flex>
                                        </Flex>
                                        <Flex
                                            direction='column'
                                            justifyContent='space-between'
                                        >
                                            <Flex justifyContent='flex-end'>
                                                <Button variant='ghost' onClick={reset}>
                                                    <BsArrowClockwise size='32px' />
                                                </Button>
                                            </Flex>
                                            <Flex>
                                                <Button
                                                    leftIcon={<BsPlusLg />}
                                                    bgColor='black'
                                                    color='white'
                                                    width='120px'
                                                    onClick={onDepositOpen}
                                                >Deposit</Button>
                                                <Button
                                                    width='120px'
                                                    bgColor='white'
                                                    border='1px solid black'
                                                    ml='10'
                                                    leftIcon={<BsSend />}
                                                    onClick={() => { setTransferSubmit(false); onTransferOpen() }}
                                                >Send</Button>
                                            </Flex>
                                        </Flex>
                                    </Flex>
                                )
                                }
                            </Flex>

                        </Flex>
                        <Flex
                            maxWidth='1120px'
                            width='78%'
                            borderRadius='3xl'
                            bgColor='white'
                            direction='column'
                            p='6'
                            pl='8'
                            mt='10'
                        >
                            <Flex mb='2' justifyContent='space-between'>
                                <Flex>
                                    <Text fontWeight='bold' fontSize='24px'>Records</Text>
                                </Flex>
                                <Flex>
                                    <Select placeholder='' onChange={handleSelectChange}>
                                        {stakeList && stakeList.length > 0 ? (
                                            stakeList.map((stake, index) => (
                                                <option key={index} value={stake.staking_pool_canister.toText()}>{stake.name}</option>
                                            ))
                                        ) : (
                                            <Text>No staking pools available.</Text>
                                        )}
                                    </Select>
                                </Flex>
                            </Flex>
                            <Flex justifyContent='center'>
                                <Table variant='simple' size="sm">
                                    <Thead fontSize='12px'>
                                        <Tr>
                                            <Th textTransform="none">Date</Th>
                                            <Th textTransform="none">Stake pool</Th>
                                            <Th textTransform="none">Ammount(btc) </Th>
                                            <Th textTransform="none">Days</Th>
                                            <Th textTransform="none">Status</Th>
                                        </Tr>
                                    </Thead>
                                    <Tbody>
                                        {
                                            stakeRecords.map((v, i) => {

                                                return (
                                                    <Tr fontSize='14px'>
                                                        <Td>{formatDateminute(v.sent_time)}</Td>
                                                        <Td>{truncateMiddle(v.staking_address, 5, 5)}</Td>
                                                        <Td>{Number(v.sent_amount) / btcunity}</Td>
                                                        <Td>{v.duration_in_day.toString()}</Td>
                                                        <Td>
                                                            {Object.keys(v.status)[0]}
                                                        </Td>
                                                        <Td>
                                                            {/*<Button onClick={() => UnstakeOpen(v)}>unstake</Button>*/}
                                                            <Button isDisabled>unstake</Button>
                                                        </Td>
                                                    </Tr>
                                                );
                                            })
                                        }
                                    </Tbody>
                                </Table>
                            </Flex>
                        </Flex>
                        <Modal
                            isCentered
                            isOpen={isCreateOpen}
                            onClose={onCreateClose}
                        >
                            <ModalOverlay />
                            <ModalContent>
                                <ModalHeader>
                                    {!createWallet ? (
                                        <Text>Select Wallet</Text>
                                    ) : (
                                        <Text>Create your wallet</Text>
                                    )}
                                </ModalHeader>
                                <ModalCloseButton />
                                <ModalBody pb={6}>
                                    {!createWallet ? (
                                        <FormControl>
                                            {!isLoading ? (
                                                <Flex direction='column'>
                                                    {walletList && walletList.length > 0 ? (
                                                        walletList.map((wallet, index) => (
                                                            <Flex
                                                                mt='2'
                                                                py='2'
                                                                px='6'
                                                                borderRadius='xl'
                                                                style={{ cursor: 'pointer' }}
                                                                border={`1px solid ${wallet.bitcoin_address === walletSelect.bitcoin_address ? '#FFA940' : '#E0E0E0'}`}
                                                                onClick={() => selectWallet(wallet)}
                                                            >
                                                                <Flex direction='column'>
                                                                    <Text fontSize='24px' fontWeight='bold'>{wallet.name}</Text>
                                                                    <Flex alignItems='center'>
                                                                        <Text fontSize='15px' mr='2'>{truncateMiddle(wallet.bitcoin_address, 5, 5)}</Text>
                                                                        <Button variant='ghost' onClick={(e) => {
                                                                            e.stopPropagation(); // Blocking event bubbling
                                                                            handleCopy(wallet.bitcoin_address);
                                                                        }}>
                                                                            <BsCopy size='15px' />
                                                                        </Button>
                                                                    </Flex>
                                                                </Flex>
                                                                <Flex>

                                                                </Flex>
                                                            </Flex>
                                                        ))
                                                    ) : (
                                                        <Text>No Wallet available.</Text>
                                                    )}
                                                    {walletList && walletList.length < 3 && (
                                                        <Button
                                                            mt='6'
                                                            bgColor='white'
                                                            border='1px solid black'
                                                            borderRadius='lg'
                                                            width='100%'
                                                            leftIcon={<BsPlusLg />}
                                                            onClick={() => setCreateWallet(true)}
                                                        >
                                                            Create New Wallet
                                                        </Button>
                                                    )}
                                                </Flex>
                                            ) : (
                                                <Flex align='center' width='100%' justifyContent='center' height='100px'>
                                                    <Spinner
                                                        thickness='4px'
                                                        speed='0.65s'
                                                        emptyColor='gray.200'
                                                        color='blue.500'
                                                        size='xl'
                                                    />
                                                </Flex>
                                            )}
                                        </FormControl>
                                    ) : (
                                        <FormControl>
                                            <FormLabel>wallet name</FormLabel>
                                            <Input placeholder="wallet name" onChange={event => setWalletName(event.currentTarget.value)} />
                                        </FormControl>
                                    )}
                                </ModalBody>
                                {!createWallet ? (
                                    <></>
                                ) : (
                                    <ModalFooter>
                                        <Flex
                                            width='100%'
                                            justifyContent='center'
                                        >
                                            <Button
                                                mr={3}
                                                width='50%'
                                                color="black"
                                                bgColor="white"
                                                border='1px solid black'
                                                onClick={() => setCreateWallet(false)}
                                            >Cancel</Button>
                                            <Button
                                                width='50%'
                                                bgColor="black"
                                                color="white"
                                                _hover={{ bg: "purple.300", borderColor: "purple.500" }}
                                                isLoading={isLoading}
                                                loadingText='Creating...'
                                                onClick={onCreateWallet}
                                            >
                                                Save
                                            </Button>
                                        </Flex>
                                    </ModalFooter>
                                )}
                            </ModalContent>
                        </Modal>
                        {/* Send token modal*/}
                        <Modal
                            isOpen={isTransferOpen}
                            onClose={onTransferClose}
                            isCentered
                            closeOnOverlayClick={false}
                        >
                            <ModalOverlay />
                            <ModalContent pt='10' pb='8' borderRadius="3xl">
                                <ModalHeader>
                                    {!transferSubmit ? (
                                        <Text fontSize='24px' fontWeight='700'>Send</Text>
                                    ) : (
                                        <Text fontSize='24px' fontWeight='700'>Approve Transaction</Text>
                                    )}</ModalHeader>
                                <ModalCloseButton />
                                <ModalBody pb={6}>
                                    {!transferSubmit ? (
                                        <Flex direction='column' justifyContent="center">
                                            <FormControl width='100%' mb='6'>
                                                <FormLabel textAlign="start" fontSize='14px' fontWeight='500'>Asset</FormLabel>
                                                <Flex
                                                    borderRadius='lg'
                                                    justifyContent='flex-start'
                                                    border='1px solid #EAEAEA'
                                                    bgColor='#F5F5F5'
                                                >
                                                    <Flex pl='2'>
                                                        <Image src='./favicon.png' boxSize="20px" />
                                                        <Text ml='2'>btc</Text>
                                                    </Flex>
                                                </Flex>
                                            </FormControl>
                                            <FormControl width='100%' mb='6'>
                                                <FormLabel textAlign="start" fontSize='14px' fontWeight='500'>Recipient</FormLabel>
                                                <Input placeholder="address" onChange={event => setTransferAddress(event.currentTarget.value)} />
                                            </FormControl>
                                            <FormControl width='100%'>
                                                <FormLabel textAlign="start" fontSize='14px' fontWeight='500'>Amount</FormLabel>
                                                <InputGroup>
                                                    <InputLeftElement pointerEvents="none">
                                                        <Image src='./favicon.png' boxSize="1.2rem" />
                                                    </InputLeftElement>

                                                    <Input
                                                        type="number"
                                                        value={transferBalance}
                                                        border="1px solid #E6EAF5"
                                                        placeholder='0.0'
                                                        onChange={handleChangeTransfer}
                                                        pr="4.5rem" // Add padding to the right to make space for the InputRightElement
                                                    />

                                                    <InputRightElement width="auto" display="flex" alignItems="center">
                                                        <Select value={btcUnit} onChange={onChangebtcunit} width="auto" mr={2}>
                                                            <option value="btc">btc</option>
                                                            <option value="satoshi">satoshi</option>
                                                        </Select>
                                                        <Button mr='2' height='80%' color="orange.300" isDisabled={!isLogin} p={2} fontSize="0.8rem" onClick={onMaxClick}>
                                                            MAX
                                                        </Button>
                                                    </InputRightElement>
                                                </InputGroup>
                                            </FormControl>
                                            <Text fontSize="0.8rem" color='red'><span>{balanceError}</span></Text>
                                        </Flex>
                                    ) : (
                                        <Flex
                                            direction='column'
                                        >
                                            <Flex
                                                width='100%'
                                                direction='column'
                                                bgColor='#F5F5F5'
                                                borderRadius='xl'
                                            >
                                                <Flex
                                                    p='3'
                                                    fontSize='14px'
                                                    width='100%'
                                                    justifyContent='space-between'
                                                >
                                                    <Text color='#000000' fontWeight='800'>Send</Text>
                                                    <Text>{transferBalance} btc</Text>
                                                </Flex>
                                                <Flex
                                                    p='3'
                                                    fontSize='14px'
                                                    width='100%'
                                                    justifyContent='space-between'
                                                >
                                                    <Text color='#000000' fontWeight='800'>From</Text>
                                                    {walletSelect && walletSelect.bitcoin_address && (
                                                        <Text textAlign='right' width='80%'>{walletSelect.bitcoin_address}</Text>
                                                    )}
                                                </Flex>
                                                <Flex
                                                    p='3'
                                                    fontSize='14px'
                                                    width='100%'
                                                    justifyContent='space-between'
                                                >
                                                    <Text color='#000000' fontWeight='800'>to</Text>
                                                    <Text textAlign='right' width='80%'>{transferAddress}</Text>
                                                </Flex>
                                            </Flex>
                                            <Flex
                                                width='100%'
                                                direction='column'
                                                bgColor='#F5F5F5'
                                                borderRadius='xl'
                                                mt='3'
                                            >
                                                <Flex
                                                    p='3'
                                                    fontSize='14px'
                                                    width='100%'
                                                    justifyContent='space-between'
                                                >
                                                    <Text color='#000000' fontWeight='800'>Network Fee</Text>
                                                    <Text>{btcFee} btc</Text>
                                                </Flex>
                                            </Flex>
                                        </Flex>
                                    )}
                                </ModalBody>

                                <ModalFooter>
                                    {!transferSubmit ? (
                                        <Flex justifyContent="center" width='100%'>
                                            <Button
                                                width='60%'
                                                bgColor="white"
                                                color="black"
                                                border='1px solid black'
                                                mr='6'
                                                _hover={{ bg: "purple.300", borderColor: "purple.500" }}
                                                onClick={onTransferClose}>
                                                Cancel
                                            </Button>
                                            <Button
                                                width='60%'
                                                bgColor="#000000"
                                                color="white"
                                                _hover={{ bg: "purple.300", borderColor: "purple.500" }}
                                                onClick={() => { get_fee(); setTransferSubmit(true) }}>
                                                Next
                                            </Button>
                                        </Flex>
                                    ) : (
                                        <Flex justifyContent="center" width='100%'>
                                            <Button
                                                isLoading={isLoading}
                                                loadingText='Submitting'
                                                width='100%'
                                                bgColor="#000000"
                                                color="white"
                                                _hover={{ bg: "purple.300", borderColor: "purple.500" }}
                                                onClick={transferBlanace}>
                                                Approve
                                            </Button>
                                        </Flex>


                                    )}
                                </ModalFooter>
                            </ModalContent>
                        </Modal>
                        {/* unstake modal */}
                        <Modal
                            isOpen={isUnstakeOpen}
                            onClose={onUnstakeClose}
                            isCentered
                            closeOnOverlayClick={true}
                        >
                            <ModalOverlay />
                            <ModalContent>
                                <ModalHeader>Unstake</ModalHeader>
                                <ModalCloseButton />
                                <ModalBody>
                                    <Flex align='center' width='100%' justifyContent='center'>
                                        <Text>Unstake now</Text>
                                    </Flex>
                                </ModalBody>
                                <ModalFooter>
                                    <Flex width='100%' justifyContent='center'>
                                        <Button isLoading={isLoading} loadingText='Unstaking' onClick={unstake_action}>Approve</Button>
                                    </Flex>
                                </ModalFooter>
                            </ModalContent>
                        </Modal>
                        {/* deposit modal */}
                        <Modal
                            isOpen={isDepositOpen}
                            onClose={onDepositClose}
                            isCentered
                            closeOnOverlayClick={true}
                        >
                            <ModalOverlay />
                            <ModalContent>
                                <ModalHeader><Text fontSize='24px' fontWeight='700'>Deposit</Text></ModalHeader>
                                <ModalCloseButton />
                                <ModalBody>
                                    {walletSelect && walletSelect.bitcoin_address && (
                                        <Flex direction='column' align='center' width='100%' justifyContent='center'>
                                            <QRCode value={walletSelect.bitcoin_address} size={180} />
                                            <Flex fontSize='16px' fontWeight='500' mt='2'>
                                                <Image src='./favicon.png' width='22px' mr='2' />
                                                Bitcoin
                                            </Flex>
                                        </Flex>
                                    )}
                                </ModalBody>
                                <ModalFooter>
                                    {walletSelect && walletSelect.bitcoin_address && (
                                        <Flex
                                            direction='column'
                                            width='100%'
                                            justifyContent='center'
                                            bgColor='#F5F5F5'
                                            borderRadius='xl'
                                        >
                                            <Flex justifyContent='center' p='6'>
                                                <Text width='80%' textAlign='center' fontSize='14px'>{walletSelect.bitcoin_address}</Text>
                                                <Button
                                                    width='10px'
                                                    variant='ghost'
                                                    onClick={() => handleCopy(walletSelect.bitcoin_address)}
                                                    leftIcon={<BsCopy />}
                                                ></Button>
                                            </Flex>

                                        </Flex>
                                    )}
                                </ModalFooter>
                            </ModalContent>
                        </Modal>
                        <Modal
                            isOpen={isfirstCreateOpen}
                            onClose={onfirstCreateClose}
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
                                            isLoading={isLoading}
                                            loadingText='Creating...'
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
                ) : (
                    <Flex
                        direction='column'
                        width='100%'
                        alignItems='center'
                    >
                        <Flex
                            maxWidth='1120px'
                            width='78%'
                            mt='10'
                            mb='2'
                        >
                            {/*
                            <Button onClick={test}>test</Button>
                            */}
                            <Text>
                                <Heading
                                    fontSize={{ md: '28px', lg: '32px', xl: '35px', '2xl': '42px', '3xl': '44px' }}
                                >Portfolio</Heading>
                            </Text>
                        </Flex>
                        <Flex
                            maxWidth='1120px'
                            width='78%'
                            justifyContent='space-between'
                            py='6'
                        >
                            <Flex>
                                {userInfo && userInfo.user_img && userInfo.user_img.length > 0
                                    ?
                                    (
                                        <Avatar
                                            size='lg'
                                            name='Dan Abrahmov'
                                            src={Array.isArray(userInfo.user_img) ? userInfo.user_img[0] || '' : userInfo.user_img || ''} />
                                    )
                                    :
                                    (
                                        <Avatar
                                            size='lg'
                                            name='Dan Abrahmov'
                                            src='https://white-philosophical-blackbird-295.mypinata.cloud/ipfs/QmP9qg9P9Q4dfWaXzHnbF2qCuyAB45oAWmoimWYaAkxGZ1/1.png' />
                                    )
                                }
                                <Flex
                                    direction='column'
                                    ml='2'
                                >
                                    <Text fontSize='24px' fontWeight='700'>
                                        {userInfo && userInfo.name && userInfo.name.length > 0
                                            ? userInfo.name
                                            : "Default Name"}
                                    </Text>
                                    <Text fontSize='14px' fontWeight='400'>
                                        {currentAccount && currentAccount.address && (
                                            <Text fontSize='14px' fontWeight='400'>
                                                {truncateMiddle(currentAccount.address, 5, 5)}
                                            </Text>
                                        )}
                                    </Text>
                                </Flex>
                            </Flex>
                            <Button
                                width='144px'
                                height='44px'
                                onClick={onOpenUserinfo}
                            >Edit Portfolio</Button>
                        </Flex>
                        <Flex
                            maxWidth='1120px'
                            width='78%'
                            borderRadius='3xl'
                            bgGradient='linear(to-r, #FFE8CC 0%, #F0D9FF 100%)'
                            direction='column'
                            p='6'
                        >
                            <Flex
                                width='100%'
                                mb='6'
                                alignItems='center'
                                justifyContent='space-between'
                            >
                                <Flex>
                                    {currentAccount && currentAccount.address ? (
                                        <Flex
                                            alignItems='center'
                                            width='180px'
                                            height='40px'
                                            borderRadius='8px'
                                            border='1px solid white'
                                            cursor='pointer'
                                            bgColor={!showICP ? '#FFFFFF' : 'transparent'}
                                            _hover={{ opacity: 0.8 }}
                                            onClick={() => setShowICP(false)}
                                        >
                                            <Image src='/marathon/task-1.svg' width='16px' mr='2' />
                                            <Text>{truncateMiddle(currentAccount.address, 5, 5)}</Text>
                                            <Button variant='ghost' onClick={() => handleCopy(currentAccount.address)}>
                                                <BsCopy size='15px' />
                                            </Button>
                                        </Flex>
                                    ) : (
                                        <Flex><Skeleton height='30px' width='100px' /></Flex>
                                    )}
                                    <Flex
                                        ml='3'
                                        alignItems='center'
                                        width='180px'
                                        height='40px'
                                        borderRadius='8px'
                                        border='1px solid white'
                                        cursor='pointer'
                                        bgColor={showICP ? '#FFFFFF' : 'transparent'}
                                        _hover={{ opacity: 0.8 }}
                                        onClick={() => setShowICP(true)}
                                    >
                                        <Box width="20px" ml='2' mr="1">
                                            <ICPLogoIcon />
                                        </Box>
                                        <Text>{getIcpWallet() ? truncateMiddle(getIcpWallet(), 5, 5) : 'ICP Wallet'}</Text>
                                        <Button variant='ghost' onClick={() => handleCopy(getIcpWallet())}>
                                            <BsCopy size='15px' />
                                        </Button>
                                    </Flex>
                                </Flex>
                            </Flex>
                            <Flex
                                width='100%'
                                height='152px'
                                bgColor='white'
                                borderRadius='xl'
                                p='6'
                                justifyContent="space-between"
                            >{showICP ?
                                (
                                    <Flex
                                        width='100%'
                                        justifyContent='space-between'
                                    >
                                        <Flex alignItems='center'>
                                            <Image height='96px' src='/marathon/rewards_og.png' />
                                            <Flex
                                                direction='column'
                                                height='96px'
                                                ml='5'
                                                justifyContent='space-between'
                                            >
                                                <Flex direction='column'>
                                                    <Text fontSize='16px' fontWeight='700' color='gray-700'>BifiPal OG NFT</Text>
                                                    <Text fontSize='14px' fontWeight='400' color='gray-500'>Holding BifiPal OG NFT to get bonus points</Text>
                                                </Flex>
                                                <Flex
                                                >
                                                    <Button
                                                        width='104px'
                                                        height='32px'
                                                        bgColor='#FFA033'
                                                        color='black'
                                                        onClick={() => { get_icp_nft(); setIsStake(true); onOpenOgStake() }}
                                                        isDisabled={!IcpWallet || !BindIcpWallet}
                                                    >Stake</Button>
                                                    <Button
                                                        ml='2'
                                                        width='104px'
                                                        height='32px'
                                                        bgColor='white'
                                                        color='black'
                                                        border='1px solid black'
                                                        onClick={() => { setIsStake(false); onOpenOgStake() }}
                                                        isDisabled={!IcpWallet || !BindIcpWallet}
                                                    >Unstake</Button>
                                                    <Button
                                                        ml='2'
                                                        width='104px'
                                                        height='32px'
                                                        bgColor='black'
                                                        color='white'
                                                    >Buy</Button>
                                                </Flex>
                                            </Flex>
                                        </Flex>
                                        <Flex alignItems='center'>
                                            <IcpConnectButton
                                                connectWallet={IcpWallet}
                                                bindWallet={BindIcpWallet}
                                                onOpenIcpList={onOpenIcpList}
                                                onCloseIcpList={onCloseIcpList}
                                                osBackend={osBackend}
                                            />
                                        </Flex>
                                    </Flex>
                                )
                                :
                                (
                                    <Flex>
                                        <Flex
                                            direction='column'
                                            mr='20'
                                        >
                                            <Flex alignItems='center' height='45px'>
                                                <Text fontSize='18px'>Balance</Text>
                                                <IconButton
                                                    icon={<BsArrowClockwise />}
                                                    variant="ghost"
                                                    aria-label="Refresh"
                                                    onClick={onRefresh_balance}
                                                    cursor="pointer"
                                                    _hover={{ bg: "gray.100" }}
                                                />
                                            </Flex>
                                            <Text fontSize='28px' fontWeight='bold'>{satoshisToBTC(currentAccount.balance || 0)}btc</Text>
                                            {/*
                                        <Text fontSize='18px'>≈${balance * btcprice}</Text>
                                        */}
                                        </Flex>
                                        <Flex
                                            direction='column'
                                            ml='10'
                                        >
                                            <Flex
                                                height='45px'
                                                alignItems='center'
                                            >
                                                <Text fontSize='18px'>Staking</Text>
                                            </Flex>
                                            <Text fontSize='28px' fontWeight='bold'>{totalBalance}btc</Text>
                                            {/*
                                        <Text fontSize='18px'>≈${totalBalance * btcprice}</Text>
                                        */}
                                        </Flex>
                                    </Flex>
                                )
                                }
                            </Flex>
                        </Flex>
                        <Flex
                            maxWidth='1120px'
                            width='78%'
                            borderRadius='3xl'
                            bgColor='white'
                            direction='column'
                            p='6'
                            pl='8'
                            mt='10'
                        >
                            <Flex mb='2' justifyContent='space-between'>
                                <Flex>
                                    <Text fontWeight='bold' fontSize='24px'>Records</Text>
                                </Flex>
                                <Flex>
                                    <Select placeholder='' onChange={handleSelectChange}>
                                        {stakeList && stakeList.length > 0 ? (
                                            stakeList.map((stake, index) => (
                                                <option key={index} value={stake.staking_pool_canister.toText()}>{stake.name}</option>
                                            ))
                                        ) : (
                                            <Text>No staking pools available.</Text>
                                        )}
                                    </Select>
                                </Flex>
                            </Flex>
                            <Flex justifyContent='center'>
                                <Table variant='simple' size="sm">
                                    <Thead fontSize='12px'>
                                        <Tr>
                                            <Th textTransform="none">Date</Th>
                                            <Th textTransform="none">Stake pool</Th>
                                            <Th textTransform="none">Ammount(BTC) </Th>
                                            <Th textTransform="none">Day</Th>
                                            <Th textTransform="none">Status</Th>
                                        </Tr>
                                    </Thead>
                                    <Tbody>
                                        {
                                            stakeRecords.map((v, i) => {

                                                return (
                                                    <Tr fontSize='14px'>
                                                        <Td>{formatDateminute(v.sent_time)}</Td>
                                                        <Td>{truncateMiddle(v.staking_address, 5, 5)}</Td>
                                                        <Td>{Number(v.sent_amount) / btcunity}</Td>
                                                        <Td>{v.duration_in_day.toString()}</Td>
                                                        <Td>
                                                            {Object.keys(v.status)[0]}
                                                        </Td>
                                                        <Td>
                                                            {/*<Button onClick={() => UnstakeOpen(v)}>unstake</Button>*/}
                                                            <Button isDisabled>unstake</Button>
                                                        </Td>
                                                    </Tr>
                                                );
                                            })
                                        }
                                    </Tbody>
                                </Table>
                            </Flex>
                        </Flex>
                    </Flex>
                )}
            </Flex>
        </>
    );
}
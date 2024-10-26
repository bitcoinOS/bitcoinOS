import React from 'react';
import { useState, useEffect } from 'react';
import {
    Flex,
    Box,
    Grid,
    Text,
    useToast,
    Button,
    Image,
    useDisclosure,
    HStack,
    PinInput,
    PinInputField,
    Spinner,
    IconButton,
    Popover,
    PopoverTrigger,
    Portal,
    PopoverContent,
    PopoverArrow,
    PopoverHeader,
    PopoverCloseButton,
    PopoverBody,
    PopoverFooter,
} from '@chakra-ui/react';
import {
    Table,
    Thead,
    Tbody,
    Tfoot,
    Tr,
    Th,
    Td,
} from '@chakra-ui/react'
import { BsCopy, BsXLg, BsArrowClockwise, BsExclamationCircleFill } from "react-icons/bs";
import {
    Modal,
    ModalOverlay,
    ModalContent,
    ModalHeader,
    ModalFooter,
    ModalBody,
    ModalCloseButton,
} from '@chakra-ui/react'
import { useNavigate } from 'react-router-dom';

import { usePointBackend } from '../../ic/PointActors';
import { useOsBackend } from '../../ic/OsActors';
import { usestakeBackend } from '../../ic/StakeActors';

import useGetMarathon from '../../utils/marathonActor';
import useGetWalletPool from '../../utils/walletActor';
import useGetStakePool from '../../utils/poolActor';

import { WalletStore, StakeNftInfo, IcpWalletStore } from '../../store/useWalletStore';
import { userStore, boxStore } from '../../store/useMarathonStore';
import { useConnectStore } from '../../store/useConnectStore';

import { formatDateminute, truncateMiddle, satoshisToBTC } from '../../utils/utils';

import { useInternetIdentity } from 'ic-use-internet-identity';
import { useSiwbIdentity } from 'ic-use-siwb-identity';

import { CurrentStatus, useStakeListStore } from '../../store';

import BoxTime from '../../components/marathon/boxTime'
import FAQAccordion from '../../components/marathon/fapText';

const Home: React.FC = () => {
    const { actor: pointBackend } = usePointBackend();
    const { actor: osBackend } = useOsBackend();
    const { actor: stakeogBackend } = usestakeBackend();

    const toast = useToast();
    const navigate = useNavigate();

    const { identity, isLoggingIn, login, clear, isLoginSuccess } = useInternetIdentity();
    const { identity: siwb_identity, identityAddress, clear: siwb_clear } = useSiwbIdentity();

    const [currentAccount, setCurrentAccount] = useConnectStore((state) => [state.currentAccount, state.setCurrentAccount])
    const { currentPool, setCurrentPool, poolDetail, setPoolDetail } = useStakeListStore();


    const [isPointInited, setIsPointInited] = useState<boolean>(false)
    const [isOsInited, setIsOsInited] = useState<boolean>(false)
    const [isStakeOgInited, setIsStakeOgInited] = useState<boolean>(false)

    const { isLoading, setIsLoading } = CurrentStatus()
    const [codepin, setCodepin] = useState('');

    const [openBox, setOpenBox] = useState<boolean>(false)
    const { isOpen, onOpen, onClose } = useDisclosure()
    const { isOpen: isBoxOpen, onOpen: onBoxOpen, onClose: onBoxClose } = useDisclosure()

    const { get_reward, get_nft_reward, get_boxes, get_boxes_record, get_user_stakebtc, get_invite_info, get_info, open_box, update_code } = useGetMarathon();
    const { get_pointrank } = useGetStakePool();
    const { update_login } = useGetMarathon();
    const { get_icp_bind, onRefresh_balance, get_user_stakednft } = useGetWalletPool()
    const { balance, walletSelect, walletList, totalBalance } = WalletStore();
    const { userStake, userInfo, userReward, userNftReward, inviteInfo } = userStore();
    const { boxNum, boxReward, boxRecord, remainingTimes } = boxStore();
    const { BindIcpWallet } = IcpWalletStore()
    const { stakedNFTs } = StakeNftInfo()
    const remainingTime = boxStore(state => state.remainingTimes[0] || 0);

    const [firstBoxnum, setFirstBoxnum] = useState<boolean>(false)

    const btcunity = 100000000;

    useEffect(() => {
        setFirstBoxnum(false)
    }, [])

    useEffect(() => {
        if (osBackend) {
            setIsOsInited(true)
        }

    }, [osBackend])

    useEffect(() => {
        if (pointBackend) {
            setIsPointInited(true)
        }

    }, [pointBackend])

    useEffect(() => {
        if (stakeogBackend) {
            setIsStakeOgInited(true)
        }

    }, [stakeogBackend])

    useEffect(() => {
        if (identity || siwb_identity) {
            if (!userInfo) {
                get_info(osBackend)
            }
            if (!BindIcpWallet) {
                get_icp_bind(osBackend)
            }
        }

    }, [osBackend, identity, siwb_identity])

    useEffect(() => {
        //---------------
        //get user staked nft info
        //---------------
        if (!userNftReward) {
            get_nft_reward(pointBackend)
        }
    }, [BindIcpWallet])

    useEffect(() => {
        if (identity !== null && identity !== undefined) {
            if (currentAccount && currentAccount.type === 'INTERNET_IDENTITY') {
                if (!userReward) {
                    get_reward(pointBackend)
                }
                if (!firstBoxnum) {
                    get_boxes(pointBackend)
                    setFirstBoxnum(true)
                }
                get_user_stakebtc(pointBackend)
                if (!boxRecord || boxRecord.length === 0) {
                    get_boxes_record(pointBackend)
                }
                if (!inviteInfo) {
                    get_invite_info(pointBackend)
                }
            }
        }

    }, [identity])

    useEffect(() => {
        if (siwb_identity !== null && siwb_identity !== undefined) {
            if (currentAccount && (currentAccount.type === 'UNISAT' || currentAccount.type === 'WIZZ')) {
                if (!userReward) {
                    get_reward(pointBackend)
                }
                if (!firstBoxnum) {
                    get_boxes(pointBackend)
                    setFirstBoxnum(true)
                }
                get_user_stakebtc(pointBackend)
                if (!boxRecord || boxRecord.length === 0) {
                    get_boxes_record(pointBackend)
                }
                if (!inviteInfo) {
                    get_invite_info(pointBackend)
                }
            }
        }

    }, [siwb_identity, currentAccount])

    const test = async () => {
        console.log('test1')
        const a = await get_nft_reward(pointBackend)
        //const a = get_pointrank(pointBackend)
    }

    const opentoBox = async () => {
        await open_box(pointBackend)
        onBoxOpen()
    }

    const sendCode = async (pinValue) => {
        setIsLoading(true)
        await update_code(osBackend, pinValue)
        setIsLoading(false)
        onClose()
        console.log('Entered PIN:', pinValue);
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

    return (
        <Flex
            width='100%'
            justifyContent='center'
            zIndex='2'
            direction='column'
        >
            {/*
            <Button onClick={test} zIndex='2'>test</Button>
            */}

            {openBox &&
                <Flex
                    zIndex={999999}
                    height="100vw"
                    bg="rgba(0, 0, 0, 0.6)"
                    width="100%"
                    position="fixed"
                    align="center"
                    justify="center"
                    top={0}
                    left={0}
                    backdropFilter="blur(5px)"
                >
                    <Flex direction='column'>
                        <Box
                            width="320px"
                            height="auto"
                            sx={{
                                padding: '16px',
                                display: 'flex',
                                justifyContent: 'center',
                                alignItems: 'center',
                            }}
                        >
                            <Image src="/marathon/marathon-box.svg" width="100%" />
                        </Box>
                        <Flex justifyContent='space-between'>
                            <Button
                                border='2px solid black'
                                bgColor='#FFB866'
                                onClick={() => setOpenBox(false)}
                            >open one</Button>
                            <Button
                                border='2px solid black'
                                bgColor='#FFB866'
                            >open all</Button>
                        </Flex>
                    </Flex>
                </Flex>
            }
            <Flex
                justifyContent='center'
                fontSize='56px'
                fontWeight='700'
            >
                btc Marathon: Phase 0
            </Flex>
            <Flex
                mt='10'
                justifyContent='center'
                direction='column'
                alignItems='center'
            >
                <Flex
                    justifyContent='space-between'
                    alignItems='center'
                    px='10'
                    width='1109px'
                    height='56px'
                    bgImage='/marathon/wallet-card.svg'
                    zIndex='1'
                >
                    <Flex alignItems='center' fontSize='16px' fontWeight='700'>
                        <Text mr='2'>Bitcoin Address: </Text>
                        {currentAccount ? (
                            (() => {
                                switch (currentAccount.type) {
                                    case 'INTERNET_IDENTITY':
                                        return (
                                            <>
                                                {walletSelect && walletSelect.bitcoin_address ? (
                                                    <>
                                                        {truncateMiddle(walletSelect.bitcoin_address, 5, 5)}
                                                        <Button variant='ghost' onClick={() => handleCopy(walletSelect.bitcoin_address)}>
                                                            <BsCopy size='15px' />
                                                        </Button>
                                                    </>
                                                ) : (
                                                    <></>
                                                )}
                                            </>
                                        );
                                    case 'WIZZ':
                                        return (
                                            <>
                                                {currentAccount && currentAccount.address ? (
                                                    <>
                                                        {truncateMiddle(currentAccount.address, 5, 5)}
                                                        <Button variant='ghost' onClick={() => handleCopy(currentAccount.address)}>
                                                            <BsCopy size='15px' />
                                                        </Button>
                                                    </>
                                                ) : (
                                                    <></>
                                                )}
                                            </>
                                        );
                                    case 'UNISAT':
                                        return (
                                            <>
                                                {currentAccount && currentAccount.address ? (
                                                    <>
                                                        {truncateMiddle(currentAccount.address, 5, 5)}
                                                        <Button variant='ghost' onClick={() => handleCopy(currentAccount.address)}>
                                                            <BsCopy size='15px' />
                                                        </Button>
                                                    </>
                                                ) : (
                                                    <></>
                                                )}
                                            </>
                                        );
                                    default:
                                        return <Text></Text>;
                                }
                            })()
                        ) : (
                            <Text></Text>
                        )}
                    </Flex>
                    <Flex fontSize='16px' fontWeight='700' alignItems='center'>
                        <Text mr='2'>Balance:</Text>
                        <Text>
                            {currentAccount ? (
                                (() => {
                                    switch (currentAccount.type) {
                                        case 'INTERNET_IDENTITY':
                                            return balance
                                        case 'WIZZ':
                                            return satoshisToBTC(currentAccount.balance);
                                        case 'UNISAT':
                                            return satoshisToBTC(currentAccount.balance);
                                        default:
                                            return 0;
                                    }
                                })()
                            ) : (
                                0
                            )}
                        </Text>
                        <Text ml='2' mr='2'>btc</Text>
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
                <Flex
                    justifyContent='space-between'
                    px='32'
                    width='1248px'
                    height='460px'
                    bgImage='/marathon/marathon-card.svg'
                    zIndex='1'
                >
                    <Flex
                        direction='column'
                    >
                        <Flex
                            alignItems='center'
                        >
                            <Flex
                                justifyContent='center'
                                direction='column'
                                //width='260px'
                                height='160px'
                            >
                                <Text fontSize='24px' fontWeight='700'>My Points</Text>
                                <Flex alignItems='center'>
                                    {userReward ? (
                                        <Text fontSize='64px' fontWeight='700'>{Number(userReward)}</Text>
                                    ) : (
                                        <Text fontSize='64px' fontWeight='700'>0</Text>
                                    )}
                                    <Flex direction='column' justifyContent='center' pt='2'>
                                        <Flex justifyContent='center' alignItems='center'>
                                            <Text fontSize='14px' fontWeight='700' mr='1'>OG NFT earned</Text>
                                            <Popover placement='top-start'>
                                                <PopoverTrigger>
                                                    <Box display="inline-block">
                                                        <BsExclamationCircleFill />
                                                    </Box>
                                                </PopoverTrigger>
                                                <PopoverContent width='140px' height='65px'>
                                                    <PopoverCloseButton />
                                                    <PopoverBody>
                                                        <Text fontSize='12px' fontWeight='400'>Stake OG NFT to earn more points!</Text>
                                                        <Text
                                                            fontSize='12px'
                                                            fontWeight='400'
                                                            color="blue.500"
                                                            cursor="pointer"
                                                            _hover={{ textDecoration: 'underline' }}
                                                            onClick={() => { navigate('/portfolio'); }}
                                                        >Go to Stake</Text>
                                                    </PopoverBody>
                                                </PopoverContent>
                                            </Popover>
                                        </Flex>
                                        <Text
                                            fontSize='40px'
                                            fontWeight='700'
                                            bgGradient="linear(90deg, #FFA033 0%, #FF66FF 100%)"
                                            bgClip="text"
                                            sx={{
                                                '-webkit-background-clip': 'text',
                                                '-webkit-text-fill-color': 'transparent',
                                            }}
                                        >+{userNftReward ? (
                                            <>{Number(userNftReward)}</>
                                        ) : (
                                            <>0</>
                                        )}</Text>
                                    </Flex>
                                </Flex>
                            </Flex>
                        </Flex>
                        <Flex>
                            <Flex
                                justifyContent='center'
                                direction='column'
                                height='250px'
                                width='260px'
                            >
                                <Text fontSize='24px' fontWeight='700'>My Staked</Text>
                                <Text fontSize='48px' fontWeight='700'>{userStake?.stake_amount
                                    ? `${(Number(userStake.stake_amount) / btcunity).toFixed(6)} btc`
                                    : '0 btc'}</Text>
                                <Button bgColor='#FFA033' width='128px' onClick={() => { setPoolDetail(false); navigate('/pools'); }}>Stake more</Button>
                            </Flex>
                            {/*
                            <Flex
                                mt='16'
                                width='110px'
                                height='110px'
                                direction='column'
                                borderRadius='50%'
                                position="relative"
                                justifyContent="center"
                                alignItems="center"
                                _before={{
                                    content: '""',
                                    position: 'absolute',
                                    top: '-4px',
                                    right: '-4px',
                                    bottom: '-4px',
                                    left: '-4px',
                                    background: 'linear-gradient(180deg, #FFDCB3 0%, #F7AAF4 100%)',
                                    borderRadius: '50%',
                                    zIndex: -1,
                                }}
                                _after={{
                                    content: '""',
                                    position: 'absolute',
                                    top: '0',
                                    right: '0',
                                    bottom: '0',
                                    left: '0',
                                    background: 'white',
                                    borderRadius: '50%',
                                    zIndex: -1,
                                }}
                            >
                                <Flex direction='column' alignItems='center'>
                                    <Text fontSize='14px' fontWeight='700'>Boost</Text>
                                    <Text
                                        fontSize='32px'
                                        fontWeight='700'
                                        background="linear-gradient(90deg, #B511E0 0%, #FF8901 100%)"
                                        backgroundClip="text"
                                        sx={{
                                            '-webkit-background-clip': 'text',
                                            '-webkit-text-fill-color': 'transparent',
                                        }}
                                    >
                                        {stakedNFTs && stakedNFTs.length > 0 ? stakedNFTs.length : 0}x
                                    </Text>
                                </Flex>
                                <Flex direction='column' alignItems='center'>
                                    <Text fontSize='12px' fontWeight='700'>Staked NFT</Text>
                                </Flex>
                            </Flex>
                            */}
                        </Flex>
                    </Flex>
                    <Flex
                        direction='column'
                        alignItems='center'
                        justifyContent='center'
                    >
                        <Box position='relative'>
                            <Image src='/marathon/marathon-box.svg' width='320px' />
                            <Box
                                position='absolute'
                                top='2px'
                                right='-70px'
                                bgGradient="linear(to-r, #CCCCCC, #F5F5F5)"
                                color='black'
                                px='2'
                                py='1'
                                minW='110px'
                                height='40px'
                                borderRadius='20px'
                                fontSize='24px'
                                fontWeight='700'
                                textAlign='center'

                            >
                                <Flex
                                    justifyContent='center'
                                    alignItems='center'
                                ><BsXLg /><Text ml='2'>{boxNum.toString()}</Text></Flex>
                            </Box>
                        </Box>
                        <Flex
                            direction='column'
                            alignItems='center'
                        >
                            <Button
                                mt='6'
                                height='56px'
                                width='240px'
                                borderRadius='16px'
                                border='2px solid #000000'
                                bgGradient="linear(to-r, #FFB866, #CC66FF, #33CCFF)"
                                onClick={opentoBox}
                                isDisabled={boxNum === 0}
                                isLoading={isLoading}
                                _disabled={{
                                    opacity: 0.6,
                                    cursor: 'not-allowed',
                                }}
                            >Unbox</Button>
                        </Flex>
                    </Flex>
                </Flex>
            </Flex>
            <Flex
                mt='6'
                justifyContent='center'
            >
                <Flex
                    direction='column'
                    width='1120px'
                    height='490px'
                    bgColor='white'
                    borderRadius='16px'
                    zIndex='1'
                >
                    <Flex
                        width='1120px'
                        px='8'
                        pt='6'
                        height='60px'
                    >
                        <Text fontSize='24px' fontWeight='700'>
                            BifiPal Tasks
                        </Text>
                    </Flex>
                    <Flex
                        justifyContent='center'
                        alignItems='center'
                        direction='column'
                        width='1120px'
                        height='430px'
                        bgColor='white'
                        borderRadius='16px'
                        zIndex='1'
                    >
                        <Flex
                            mb='6'
                            px='6'
                            width='1060px'
                            height='96px'
                            bgColor='#F5F5F5'
                            borderRadius='8px'
                            justifyContent='space-between'
                            alignItems='center'
                        >
                            <Flex
                                alignItems='center'
                            >
                                <Image src='/marathon/task-3.svg' mr='3' />
                                <Flex
                                    direction='column'

                                >
                                    <Text fontSize='24px' fontWeight='700'>Daily Check-In</Text>
                                    <Text fontSize='16px' fontWeight='400'>Complete a daily check-in to get a box.</Text>
                                </Flex>
                            </Flex>
                            <Flex>
                                {remainingTime === 0 ? (
                                    <>
                                        <Button
                                            bgColor='#FFA033'
                                            width='128px'
                                            height='40px'
                                            isLoading={isLoading}
                                            isDisabled={remainingTimes.length === 0}
                                            onClick={async () => { await update_login(osBackend, pointBackend) }}
                                        >
                                            Check in
                                        </Button>
                                    </>
                                ) : (
                                    <Text fontSize='16px'>
                                        <BoxTime />
                                    </Text>
                                )}
                            </Flex>
                        </Flex>
                        <Flex
                            mb='6'
                            px='6'
                            width='1060px'
                            height='96px'
                            bgColor='#F5F5F5'
                            borderRadius='8px'
                            justifyContent='space-between'
                            alignItems='center'
                        >
                            <Flex
                                alignItems='center'
                            >
                                <Image src='/marathon/task-1.svg' mr='3' />
                                <Flex
                                    direction='column'

                                >
                                    <Text fontSize='24px' fontWeight='700'>Daily Stake</Text>
                                    <Text fontSize='16px' fontWeight='400'>Successful completion of the daily stake earns you a box.</Text>
                                </Flex>
                            </Flex>
                            <Flex>
                                <Button bgColor='#FFA033' width='128px' height='40px' onClick={() => { setPoolDetail(false); navigate('/pools'); }}>Stake Now</Button>
                            </Flex>
                        </Flex>
                        <Flex
                            p='6'
                            width='1060px'
                            height='144px'
                            bgColor='#F5F5F5'
                            borderRadius='8px'
                            justifyContent='space-between'
                            alignItems='center'
                        >
                            <Flex
                                alignItems='center'
                            >
                                <Image src='/marathon/task-2.svg' mr='3' />
                                <Flex
                                    direction='column'

                                >
                                    <Text fontSize='24px' fontWeight='700'>Referral Friends</Text>
                                    <Text mb='2' fontSize='16px' fontWeight='400'>Get a box for every successful referral.</Text>
                                    <Text
                                        pl='2'
                                        borderRadius='4px'
                                        fontSize='20px'
                                        bgGradient="linear(to-r, #FFB866 0%, white 100%)"
                                    >Users Referred: {Number(inviteInfo?.avalable_invite_count ?? 0)}</Text>
                                </Flex>
                            </Flex>
                            <Flex direction='column'>
                                <Flex
                                    mb='4'
                                    width='308px'
                                    height='40px'
                                    border='1px solid black'
                                    borderRadius='8px'
                                    justifyContent='space-between'
                                >
                                    <Flex
                                        pl='2'
                                        width='180px'
                                        alignItems='center'
                                        fontSize='14px'
                                        fontWeight='500'
                                        color='#2D3748'
                                    >
                                        Your referral code:
                                    </Flex>
                                    <Flex alignItems='center' pr='2'>
                                        {userInfo ? (
                                            <Text mr='2' fontSize='16PX' fontWeight='700'>{userInfo.invite_code}</Text>
                                        ) : (
                                            <Text></Text>
                                        )}
                                        <BsCopy
                                            cursor='pointer'
                                            onClick={() => userInfo && userInfo.invite_code && handleCopy(userInfo.invite_code)} size='16px'
                                        />
                                    </Flex>
                                </Flex>
                                {userInfo && userInfo.invited_code && userInfo.invited_code.length > 0 ? (
                                    <Flex
                                        width='308px'
                                        height='40px'
                                        bgColor='#E0E0E0'
                                        borderRadius='8px'
                                        alignItems="center"
                                        justifyContent="center"
                                        fontWeight='700'
                                    >
                                        <Text fontSize='14px'>
                                            You accepted a referral from {userInfo.invited_code}
                                        </Text>
                                    </Flex>
                                ) : (
                                    <Button
                                        width='308px'
                                        height='40px'
                                        bgColor='#FFA033'
                                        onClick={onOpen}
                                    >
                                        Enter referral code
                                    </Button>
                                )}
                            </Flex>
                        </Flex>
                    </Flex>
                </Flex>
            </Flex>
            <Flex
                mt='6'
                justifyContent='center'
            >
                <Flex
                    direction='column'
                    width='540px'
                    height='610px'
                    bgColor='white'
                    borderRadius='16px'
                    zIndex='1'
                    mr='10'
                >
                    <Flex padding="6">
                        <Text fontSize="24px" fontWeight="bold">My Reward</Text>
                    </Flex>
                    <Grid
                        templateColumns="repeat(3, 1fr)"
                        gap={4}
                        padding="4"
                    >
                        <Flex
                            direction='column'
                            justifyContent='center'
                            alignItems='center'
                        >
                            <Image width='120px' src='/marathon/rewards_credit.png' />
                            <Text fontSize='16px'>
                                {userReward ? (
                                    <Text fontSize='16px'>{Number(userReward)}</Text>
                                ) : (
                                    <Text fontSize='16px'>0</Text>
                                )}
                            </Text>
                        </Flex>
                        <Flex
                            direction='column'
                            justifyContent='center'
                            alignItems='center'
                        >
                            <Image width='120px' src='/marathon/rewards_boost.png' />
                            <Text fontSize='16px'>Coming soon</Text>
                        </Flex>
                        <Flex
                            direction='column'
                            justifyContent='center'
                            alignItems='center'
                        >
                            <Image width='120px' src='/marathon/rewards_og.png' />
                            <Text fontSize='16px'>Coming soon</Text>
                        </Flex>
                        <Flex
                            direction='column'
                            justifyContent='center'
                            alignItems='center'
                        >
                            <Image width='120px' src='/marathon/rewards_offer.png' />
                            <Text fontSize='16px'>Coming soon</Text>
                        </Flex>
                    </Grid>
                </Flex>
                <Flex
                    direction='column'
                    width='540px'
                    height='610px'
                    bgColor='white'
                    borderRadius='16px'
                    zIndex='1'
                >
                    <Flex
                        p='6'
                        width='100%'
                        justifyContent='space-between'
                    >
                        <Text fontSize='24px' fontWeight='700'>Unbox Record</Text>
                        <Text fontSize='16px' fontWeight='500'>Total opened : {boxRecord && boxRecord.length ? boxRecord.length : 0} </Text>
                    </Flex>
                    <Flex
                        justifyContent='center'
                        direction='column'
                    >
                        <Box>
                            <Table variant='simple' size='md'>
                                <Thead>
                                    <Tr fontSize={{ md: '12px', lg: '12px', xl: '12px', '2xl': '12px', '3xl': '12px' }}>
                                        <Th textTransform="none">Reward Date</Th>
                                        <Th textTransform="none">Open Date</Th>
                                        <Th textTransform="none" isNumeric>Reward</Th>
                                    </Tr>
                                </Thead>
                            </Table>
                        </Box>
                        <Box
                            maxH='460px'
                            overflowY="auto"
                            flex="1"
                            sx={{
                                '&::-webkit-scrollbar': {
                                    width: '8px',
                                    borderRadius: '8px',
                                    backgroundColor: `rgba(0, 0, 0, 0.02)`,
                                },
                                '&::-webkit-scrollbar-thumb': {
                                    backgroundColor: `rgba(0, 0, 0, 0.08)`,
                                    borderRadius: '8px',
                                },
                                '&::-webkit-scrollbar-thumb:hover': {
                                    backgroundColor: `rgba(0, 0, 0, 0.15)`,
                                },
                                scrollbarWidth: 'thin',
                                scrollbarColor: `rgba(0, 0, 0, 0.08) rgba(0, 0, 0, 0.02)`,
                            }}
                        >
                            <Table variant='simple' size='md'>
                                <Tbody>
                                    {boxRecord && boxRecord.length > 0 ? (
                                        boxRecord.map((v, index) => {
                                            return (
                                                <Tr key={index} fontSize={{ md: '14px', lg: '14px', xl: '14px', '2xl': '14px', '3xl': '14px' }}>
                                                    <Td>
                                                        {formatDateminute(v.create_time)}
                                                    </Td>
                                                    <Td>
                                                        <div>
                                                            {formatDateminute(v.open_time)}
                                                        </div>
                                                    </Td>
                                                    <Td isNumeric>{Number(v.point)} Point</Td>
                                                </Tr>

                                            );
                                        })
                                    ) : (
                                        <Flex justifyContent='center' alignItems='center'>
                                            <Text>No Box Records.</Text>
                                        </Flex>
                                    )}
                                </Tbody>
                            </Table>
                        </Box>
                    </Flex>
                </Flex>
            </Flex>
            <Flex
                mt='6'
                justifyContent='center'
            >
                <Flex
                    p='6'
                    direction='column'
                    width='1120px'
                    minH='534px'
                    bgColor='white'
                    borderRadius='16px'
                    zIndex='1'
                >
                    <Flex color='#2D3748' fontSize='24px' fontWeight='700'>FAQ</Flex>
                    <FAQAccordion />
                    {/*
                    <Flex
                        mt='2'
                        mb='6'
                        px='6'
                        width='1060px'
                        height='80px'
                        bgColor='#F5F5F5'
                        borderRadius='8px'
                        justifyContent='space-between'
                        alignItems='center'
                    >
                        <Text fontSize='24px' fontWeight='700'>What is btc marathon?</Text>
                        <BsPlusLg />
                    </Flex>
                    <Flex
                        mt='2'
                        mb='6'
                        px='6'
                        py='5'
                        width='1060px'
                        height='124px'
                        bgColor='#F5F5F5'
                        borderRadius='8px'
                        justifyContent='space-between'
                        direction='column'
                    >
                        <Flex width='100%' justifyContent='space-between'>
                            <Text fontSize='24px' fontWeight='700'>How to get box?</Text>
                            <BsXLg />
                        </Flex>
                        <Text fontSize='16px' fontWeight='400'>In btc Marathon: Phase 0 you can earn box by logging in daily, completing stake daily and inviting friends.</Text>
                    </Flex>
                    <Flex
                        mt='2'
                        mb='6'
                        px='6'
                        width='1060px'
                        height='80px'
                        bgColor='#F5F5F5'
                        borderRadius='8px'
                        justifyContent='space-between'
                        alignItems='center'
                    >
                        <Text fontSize='24px' fontWeight='700'>How can I get Credit Points?</Text>
                        <BsPlusLg />
                    </Flex>
                    <Flex
                        mt='2'
                        mb='6'
                        px='6'
                        width='1060px'
                        height='80px'
                        bgColor='#F5F5F5'
                        borderRadius='8px'
                        justifyContent='space-between'
                        alignItems='center'
                    >
                        <Text fontSize='24px' fontWeight='700'>Will my credit score be zeroed out at the end of the marathon?</Text>
                        <BsPlusLg />
                    </Flex>
                    */}
                </Flex>
            </Flex>
            <Modal
                isCentered
                isOpen={isOpen}
                onClose={onClose}
            >
                <ModalOverlay />
                <ModalContent>
                    <ModalHeader>Enter Invitation Code</ModalHeader>
                    <ModalCloseButton />
                    <ModalBody pb={6} mt='10' mb='6'>
                        <HStack justifyContent='center'>
                            <PinInput type='alphanumeric' onChange={(value) => setCodepin(value)}>
                                <PinInputField />
                                <PinInputField />
                                <PinInputField />
                                <PinInputField />
                                <PinInputField />
                                <PinInputField />
                                <PinInputField />
                                <PinInputField />
                            </PinInput>
                        </HStack>
                    </ModalBody>

                    <ModalFooter justifyContent='center'>
                        <Button
                            bgColor='black'
                            color='white'
                            width='200px'
                            onClick={() => sendCode(codepin)}
                            isLoading={isLoading}
                        >
                            Accept
                        </Button>
                    </ModalFooter>
                </ModalContent>
            </Modal>
            <Modal
                isCentered
                isOpen={isBoxOpen}
                onClose={onBoxClose}
            >
                <ModalOverlay />
                <ModalContent pb='2' borderRadius='xl'>
                    <ModalHeader>

                    </ModalHeader>
                    <ModalCloseButton />
                    <ModalBody pb={6} mb='6'>
                        {!isLoading ? (
                            <Flex width='100%' direction='column' justifyContent='center' alignItems='center'>
                                <Text fontSize='24px' fontWeight='700'>Congratulations!</Text>
                                <Text fontSize='16px' fontWeight='400'>
                                    {boxReward && boxReward.open_count
                                        ? `${Number(boxReward.open_count)}`
                                        : '0'} boxes opened</Text>
                                <Text
                                    mt='10'
                                    fontSize='40px'
                                    fontWeight='700'
                                    bgGradient="linear(to-r, #FFB866, #CC66FF, #33CCFF)"
                                    bgClip="text"
                                >
                                    {boxReward && boxReward.box_point
                                        ? `+${Number(boxReward.box_point)}`
                                        : '+0'}

                                </Text>
                            </Flex>
                        ) : (
                            <Flex direction='column' alignItems='center' justifyContent='center'>
                                <Text>
                                    Opening...
                                </Text>
                                <Spinner
                                    color="purple.500"
                                    size="xl"
                                    speed="0.65s"
                                    thickness="4px" // 调整厚度
                                />
                            </Flex>
                        )}
                    </ModalBody>

                    <ModalFooter justifyContent='center'>
                        {!isLoading ? (
                            <Button bgColor='#FFA033' color='black' width='200px' onClick={onBoxClose}>
                                Let's go!
                            </Button>
                        ) : (<></>)}
                    </ModalFooter>
                </ModalContent>
            </Modal>
        </Flex >
    );
};

export default Home;
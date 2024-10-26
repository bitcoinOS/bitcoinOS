import React from 'react';
import { Flex, Box, Text, useToast, Tooltip } from '@chakra-ui/react';

import {
    Table,
    Thead,
    Tbody,
    Tfoot,
    Tr,
    Th,
    Td,
    Button, Tabs, TabList, TabPanels, Tab, TabPanel
} from '@chakra-ui/react'
import { BsCopy } from "react-icons/bs";

import Footer from '../../components/Footer';

import { usePointBackend, RewardRecord, Metadata } from "../../ic/PointActors";
import { useInternetIdentity } from "ic-use-internet-identity";
import { useSiwbIdentity } from 'ic-use-siwb-identity';
import { useEffect, useState } from 'react';
import { truncateMiddle } from '../../utils/utils'
import { usePointStore } from '../../store/useStakePool';
import { userStore } from '../../store/useMarathonStore';
import { checkIdentityExpiration } from '../../utils/utils';
import { useConnectStore } from '../../store/useConnectStore';

import useGetStakePool from '../../utils/poolActor';
import useGetMarathon from '../../utils/marathonActor';
import { formatDateminute } from '../../utils/utils';
const Leaderboard: React.FC = () => {

    const [currentAccount, setCurrentAccount] = useConnectStore((state) => [state.currentAccount, state.setCurrentAccount])

    const toast = useToast();
    const { actor: pointBackend } = usePointBackend();
    const { identity, login, clear } = useInternetIdentity();
    const { identity: siwb_identity, identityAddress, clear: siwb_clear } = useSiwbIdentity();

    const [isLogin, setIslogin] = useState<boolean>(false);
    const [isPointInited, setIsPointInited] = useState<boolean>(false);
    const { pointRank, creditRank, rankStatus, setPointRank, setCreditRank } = usePointStore()
    const { userReward, userCredit } = userStore();

    const [retryCount, setRetryCount] = useState(0);

    const { get_pointrank, get_pointrank_status } = useGetStakePool();
    const { get_reward } = useGetMarathon();

    const btcunity = 100000000;
    // bigint type
    //const btcunity = 100000000n;

    useEffect(() => {
        if (identity) {
            setIslogin(true);
        }
        if (!pointBackend) {
            setIsPointInited(false);
        } else {
            setIsPointInited(true);
        }
    }, []);

    useEffect(() => {
        if (identity) {
            setIslogin(true);
        } else {
            setIslogin(false);
        }
    }, [identity]);

    useEffect(() => {
        if (pointBackend) {
            setIsPointInited(true);
        } else {
            setIsPointInited(false);
        }
    }, [pointBackend]);

    useEffect(() => {
        if (pointBackend) {
            if (retryCount < 3) {
                get_pointrank_status(pointBackend)
                setRetryCount(prevRetryCount => prevRetryCount + 1);
            }
        }
    }, [pointBackend, identity, siwb_identity, retryCount])

    useEffect(() => {
        get_pointrank()
    }, [])

    const getAccount = React.useMemo(() => {
        if (!currentAccount) {
            return () => null;
        }
        console.log(currentAccount)
        switch (currentAccount.type) {
            case 'UNISAT':
                return () => siwb_identity ? siwb_identity.getPrincipal().toString() : null;
            case 'INTERNET_IDENTITY':
                return () => identity ? identity.getPrincipal().toString() : null;
            case 'WIZZ':
                return () => siwb_identity ? siwb_identity.getPrincipal().toString() : null;
            default:
                return () => null;
        }
    }, [currentAccount, siwb_identity, identity]);

    const account = getAccount();

    const matchingRecordIndex = creditRank.findIndex(record => record.user_id.toString() === account);
    const matchingRecord = matchingRecordIndex !== -1 ? creditRank[matchingRecordIndex] : null;

    const test = async () => {
        const a = await get_pointrank()
        //console.log('Test function getAccount result:', getAccount);
        //get_Pointrank()
    }

    const handleCopy = (text) => {
        // copy address
        navigator.clipboard.writeText(text).then(() => {
            toast({
                title: "Account copied",
                status: "success",
                position: "bottom-right",
                duration: 2000,
                isClosable: true,
            })
        })
    }
    const [tabIndex, setTabIndex] = useState(0);
    return (
        <Box
            minH='100vh'
            display="flex"
            flexDirection="column"
            bgGradient="linear(to-r, #EAC1FF 0%, #FFEFDC 100%)"
            position="relative"
            overflow="hidden"

            pb='10'
        >
            {/*
            <Button zIndex='2' onClick={test}>test</Button>
            */}
            <Box
                position="absolute"
                top="0"
                left="0"
                right="0"
                bottom="0"
                //bgImage='url(/marathon/background.svg)'
                bgImage='url(/marathon/line_50.png)'
                style={{
                    filter: 'brightness(150%) contrast(10%)',
                    opacity: 0.4
                }}
                backgroundPosition="center"
                zIndex="1"
            />
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
            >
                <Flex
                    px='6'
                    direction='column'
                    width='1120px'
                    minH='878px'
                    bgColor='white'
                    borderRadius='16px'
                    zIndex='1'
                >
                    <Flex justifyContent='center'>
                        <Text fontSize='16px' fontWeight='400'>
                            {rankStatus?.update_time && (
                                <Text>Updated:  {formatDateminute(rankStatus.update_time)} UTC</Text>
                            )}
                        </Text>
                    </Flex>

                    <Flex
                        justifyContent='center'
                    >
                        <Table variant='simple' size='lg'>
                            <Thead>
                                <Tr
                                    sx={{
                                        position: 'relative',
                                        isolation: 'isolate',
                                        '&::before': {
                                            content: '""',
                                            position: 'absolute',
                                        }
                                    }}
                                    fontSize={{ md: '12px', lg: '12px', xl: '12px', '2xl': '12px', '3xl': '12px' }}
                                >
                                    <Th textTransform="none">Rank</Th>
                                    <Th textTransform="none">Account</Th>
                                    {/*
                                    <Th textTransform="none">Staked</Th>
                                    */}
                                    <Th textTransform="none" isNumeric>Points</Th>
                                </Tr>
                            </Thead>
                            <Tbody >

                                {userCredit && (
                                    <Tr
                                        sx={{
                                            position: 'relative',
                                            isolation: 'isolate',
                                            '&::before': {
                                                content: '""',
                                                position: 'absolute',
                                                inset: '-2px',
                                                padding: '2px',
                                                borderRadius: '8px',
                                                background: 'linear-gradient(to right, #FFB866, #CC66FF, #33CCFF)',
                                                WebkitMask: 'linear-gradient(#fff 0 0) content-box, linear-gradient(#fff 0 0)',
                                                WebkitMaskComposite: 'xor',
                                                maskComposite: 'exclude',
                                                zIndex: -1,
                                            },
                                        }}
                                        fontSize={{ md: '14px', lg: '14px', xl: '14px', '2xl': '14px', '3xl': '14px' }}
                                    >
                                        <Td>
                                            <Flex>
                                                <Text>
                                                    {Number(userCredit.point_rank) === 0 ? '- -' : Number(userCredit.point_rank)}
                                                </Text>
                                                <Text
                                                    ml='2'
                                                    fontSize='14px'
                                                    fontWeight='700'
                                                    sx={{
                                                        background: "linear-gradient(to right, #FF8800, #AA00FF, #3600FF, #33CCFF)",
                                                        backgroundClip: "text",
                                                        textFillColor: "transparent",
                                                        WebkitBackgroundClip: "text",
                                                        WebkitTextFillColor: "transparent",
                                                    }}
                                                >Your position</Text>
                                            </Flex></Td>
                                        <Td>
                                            <Tooltip label={userCredit.user_id.toString()} aria-label="Full account">
                                                <span>{userCredit.user_id.toString()}</span>
                                            </Tooltip>
                                        </Td>
                                        <Td isNumeric>
                                            {userCredit.point}
                                        </Td>
                                    </Tr>
                                )}


                                {pointRank.map((record, index) => (
                                    <Tr
                                        key={index}
                                        sx={{
                                            position: 'relative',
                                            isolation: 'isolate',
                                            '&::before': {
                                                content: '""',
                                                position: 'absolute',
                                            }
                                        }}
                                        fontSize={{ md: '14px', lg: '14px', xl: '14px', '2xl': '14px', '3xl': '14px' }}
                                    >
                                        <Td>{Number(record.point_rank)}</Td>
                                        <Td>
                                            <Flex alignItems='center'>
                                                <Tooltip label={record.user_id.toString()} aria-label="Full account">
                                                    <span>{record.user_id.toString()}</span>
                                                </Tooltip>
                                                <Button variant='ghost' onClick={() => handleCopy(record.user_id.toString())}>
                                                    <BsCopy size='15px' />
                                                </Button>
                                            </Flex>
                                        </Td>
                                        {/*
                                        <Td>
                                            {(Number(record.actual_amount) / btcunity).toString()} btc
                                        </Td>
                                        */}
                                        <Td isNumeric>
                                            {record.point}
                                        </Td>
                                    </Tr>
                                ))}
                            </Tbody>
                        </Table>
                    </Flex>

                </Flex>
            </Flex>
            <Footer />
        </Box >
    );
};

export default Leaderboard;
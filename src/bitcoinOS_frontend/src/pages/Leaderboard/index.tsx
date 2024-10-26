import React from 'react';
import { useRef, useState, useEffect } from 'react'
import {
    Flex,
    Box,
    Tabs,
    TabList,
    Tab,
    TabPanels,
    TabPanel,
    Button
} from '@chakra-ui/react';

// subpage component
import Home from './Home';
import Rule from './Rule';
import Reward from './Reward';
import Leaderboard2 from './Leaderboard';

import Footer from '../../components/Footer';

import { useIsScrolledToBottom } from '../../hooks/useIsScrolledToBottom'
import { CreateWalletModal } from '../../components/CreateModal';
import { useDisclosure } from '@chakra-ui/react';
import { useOsBackend } from '../../ic/OsActors';
import { useWalletBackend } from '../../ic/WalletActors';
import { useIdentitySelect } from '../../utils/utils';
import useGetWalletPool from '../../utils/walletActor';

import { WalletStore } from '../../store/useWalletStore';
import { useConnectStore } from '../../store/useConnectStore';

const Leaderboard = () => {
    const { actor: osBackend } = useOsBackend();
    const { actor: walletBackend } = useWalletBackend();

    const [isOsInited, setIsOsInited] = useState<boolean>(false)
    const [isWalletInited, setIsWalletInited] = useState<boolean>(false)

    const isScrolledToBottom = useIsScrolledToBottom()
    const [tabListHeight, setTabListHeight] = useState(0);
    const tabListRef = useRef(null);

    const { isOpen, onOpen, onClose } = useDisclosure();
    const { identity_select } = useIdentitySelect();
    const { balance, walletList, walletSelect, currentWallet, wallet, setWalletList, setWalletSelect, setCurrentWallet, totalBalance, setTotalBalance } = WalletStore();
    const { get_wallets, get_balance, transfer_balance } = useGetWalletPool()
    const [currentAccount] = useConnectStore((state) => [state.currentAccount])

    const [firstGetbalance, setFirstGetbalance] = useState<boolean>(false)

    useEffect(() => {
        if (osBackend) {
            setIsOsInited(true)
        }

    }, [osBackend])

    useEffect(() => {
        // debugger
        if (osBackend) {
            setIsOsInited(true)
        }

        if (identity_select && osBackend && !wallet) {
            get_wallets(osBackend);
        }
        // if (identity_select) {
        //     if (walletList && walletList.length === 0 && currentAccount.type === 'INTERNET_IDENTITY') {
        //         onOpen()
        //     } else {
        //         onClose()
        //     }
        // }
        // if (osBackend && stakeList.length === 0) {
        //     get_stake_pool(osBackend)
        // }
    }, [osBackend, identity_select]);

    useEffect(() => {
        if (walletBackend && walletSelect && walletSelect.bitcoin_address && !firstGetbalance) {
            console.log(walletBackend)
            get_balance(walletBackend, walletSelect.bitcoin_address)
            setFirstGetbalance(true)
        }
    }, [walletBackend, walletSelect]);

    useEffect(() => {
        if (tabListRef.current) {
            setTabListHeight(tabListRef.current.offsetHeight);
        }
    }, []);

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
            <Button zIndex='2' onClick={onOpen}>test </Button>
            */}
            <Flex>
                <CreateWalletModal
                    isOpen={isOpen}
                    onClose={onClose}
                    osBackend={osBackend}
                    identity={identity_select}
                />
            </Flex>
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
            <Tabs isFitted width='100vw' variant="line" flex="1" display="flex" flexDirection="column">
                <TabPanels
                    flex="1"
                    width='100vw'
                    justifyContent='center'
                    overflow='hidden'
                >
                    <TabPanel><Home /></TabPanel>
                    <TabPanel><Reward /></TabPanel>
                    <TabPanel><Leaderboard2 /></TabPanel>
                </TabPanels>
                {/*
                <Box
                    ref={tabListRef}
                    position="fixed"
                    bottom="0"
                    left="0"
                    right="0"
                    zIndex="10"
                    bg=""
                >
                    <Flex justify="center">
                        <TabList
                            px='16'
                            pt='2'
                            width="900px"
                            height="80px"
                            alignItems='center'
                            justifyContent='center'
                            zIndex='2'
                            bgImage='/marathon/tabsbg.svg'
                            backgroundSize={{ base: "auto", md: "cover" }}
                            backgroundPosition={{ base: "top left", md: "center" }}
                            backgroundRepeat="no-repeat"
                            borderBottom="none"
                        >
                            <Tab
                                fontSize='20px'
                                fontWeight='700'
                                width='100px'
                                height='56px'
                                borderRadius='10px'
                                position='relative'
                                overflow='hidden'
                                _selected={{
                                    '&::before': {
                                        content: '""',
                                        position: 'absolute',
                                        top: '-2px',
                                        left: '-2px',
                                        right: '-2px',
                                        bottom: '-2px',
                                        background: 'linear-gradient(to right, #FFB866, #CC66FF)',
                                        borderRadius: '12px',
                                        zIndex: -1,
                                    },
                                    '&::after': {
                                        content: '""',
                                        position: 'absolute',
                                        top: '2px',
                                        left: '2px',
                                        right: '2px',
                                        bottom: '2px',
                                        background: 'white',
                                        borderRadius: '8px',
                                        zIndex: -1,
                                    },
                                }}

                            >Marathon</Tab>
                            <Tab
                                fontSize='20px'
                                fontWeight='700'
                                width='100px'
                                height='56px'
                                borderRadius='10px'
                                position='relative'
                                overflow='hidden'
                                _selected={{
                                    '&::before': {
                                        content: '""',
                                        position: 'absolute',
                                        top: '-2px',
                                        left: '-2px',
                                        right: '-2px',
                                        bottom: '-2px',
                                        background: 'linear-gradient(to right, #FFB866, #CC66FF)',
                                        borderRadius: '12px',
                                        zIndex: -1,
                                    },
                                    '&::after': {
                                        content: '""',
                                        position: 'absolute',
                                        top: '2px',
                                        left: '2px',
                                        right: '2px',
                                        bottom: '2px',
                                        background: 'white',
                                        borderRadius: '8px',
                                        zIndex: -1,
                                    },
                                }}
                            >Reward</Tab>
                            <Tab
                                fontSize='20px'
                                fontWeight='700'
                                width='100px'
                                height='56px'
                                borderRadius='10px'
                                position='relative'
                                overflow='hidden'
                                _selected={{
                                    '&::before': {
                                        content: '""',
                                        position: 'absolute',
                                        top: '-2px',
                                        left: '-2px',
                                        right: '-2px',
                                        bottom: '-2px',
                                        background: 'linear-gradient(to right, #FFB866, #CC66FF)',
                                        borderRadius: '12px',
                                        zIndex: -1,
                                    },
                                    '&::after': {
                                        content: '""',
                                        position: 'absolute',
                                        top: '2px',
                                        left: '2px',
                                        right: '2px',
                                        bottom: '2px',
                                        background: 'white',
                                        borderRadius: '8px',
                                        zIndex: -1,
                                    },
                                }}
                            >Leaderboard</Tab>
                        </TabList>
                    </Flex>
                </Box>
                */}
            </Tabs>
            <Footer />

        </Box >
    );
};

export default Leaderboard;
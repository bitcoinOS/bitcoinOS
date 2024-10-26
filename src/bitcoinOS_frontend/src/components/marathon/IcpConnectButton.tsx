import React, { useEffect } from 'react';
import { Flex, Button, Text, useDisclosure } from '@chakra-ui/react';

import { usestakeBackend } from '../../ic/StakeActors';

import useGetWalletPool from '../../utils/walletActor';
import { IcpWalletStore, StakeNftInfo } from '../../store/useWalletStore';
import { Observable } from '@dfinity/agent/lib/cjs/observable';

import { BindIcpModal } from '../Modal/BindIcpModal';

const IcpConnectButton = ({ connectWallet, bindWallet, onOpenIcpList, onCloseIcpList, osBackend }) => {
    const { actor: stakeogBackend } = usestakeBackend();

    const { get_user_stakednft } = useGetWalletPool()
    const { IcpWallet } = IcpWalletStore()
    const { stakedNFTs } = StakeNftInfo()

    const { isOpen: isOpenBindIcp, onOpen: onOpenBindIcp, onClose: onCloseBindIcp } = useDisclosure()

    useEffect(() => {
        if (IcpWallet) {
            onCloseIcpList()
            // todo: detect if the user has any wallet, if so, close the modal
            // else open the modal for creating a wallet
        }
    }, [IcpWallet])

    useEffect(() => {
        get_user_stakednft(stakeogBackend)
    }, [stakeogBackend])

    if (!connectWallet) {
        return (
            <Flex alignItems='center'>
                <Button
                    onClick={onOpenIcpList}
                    variant="outline"
                    border="1px dashed"
                    borderColor="#2D3748"
                    color="#2D3748"
                    _hover={{
                        bg: 'gray.100',
                    }}
                    width='200px'
                    height='44px'
                >
                    Connect ICP Wallet
                </Button>
            </Flex>
        );
    } else {
        if (!bindWallet) {
            return (
                <>
                    <BindIcpModal
                        isOpen={isOpenBindIcp}
                        onClose={onCloseBindIcp}
                        osBackend={osBackend}
                    />
                    <Button
                        width='200px'
                        height='44px'
                        bgColor='gray.200'
                        color='black'
                        onClick={onOpenBindIcp}
                    //onClick={() => bind_icpWallet(osBackend)}
                    >
                        Bind Wallet
                    </Button>
                </>
            );
        } else {
            return (
                <Flex
                    bgGradient="linear(to-b, #FFDCB3 0%, #F7AAF4 100%)"
                    width='96px'
                    height='96px'
                    py={4}
                    px={2}
                    borderRadius="md"
                    direction='column'
                >
                    <Text fontSize='14px' fontWeight='500'>Staked NFT</Text>
                    <Text fontSize='24px' fontWeight='700'>
                        {stakedNFTs && stakedNFTs.length > 0 ? stakedNFTs.length : 0}
                    </Text>
                </Flex>
            );
        }
    }
};

export default IcpConnectButton;
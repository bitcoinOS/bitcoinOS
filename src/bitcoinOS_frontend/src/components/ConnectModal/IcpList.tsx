import { Button, Flex, Icon, Tag } from "@chakra-ui/react";
import {
    Modal,
    ModalOverlay,
    ModalContent,
    ModalHeader,
    ModalFooter,
    ModalBody,
    ModalCloseButton,
    useDisclosure
} from '@chakra-ui/react'

import { ConnectWalletType } from "../../store/useConnectStore";
import useWalletConnector from "../../hooks/useWalletConnector";

import { idlFactory } from './nft.did.js';

import useGetWalletPool from "../../utils/walletActor";
import { IcpWalletStore } from "../../store/useWalletStore";

import { icphost, nftCanisterId } from "../../utils/utils";

export default function IcpList() {
    const {
        connect,
        connectingWallet,
        isInstalled
    } = useWalletConnector()

    const { isOpen, onOpen, onClose } = useDisclosure()

    const { get_icp_nft } = useGetWalletPool()
    const { IcpWallet, BindIcpWallet, setIcpWallet } = IcpWalletStore()
    const wallets = [
        {
            type: ConnectWalletType.WIZZ,
            name: 'Plug Wallet',
            icon: (
                <img
                    src="https://tjobulav7wqqn6ws3kimu7zm4bl5ckyfk5mvua37cjojmbdvhxva.arweave.net/mlwaLBX9oQb60tqQyn8s4FfRKwVXWVoDfxJclgR1Peo"
                    alt="Plug Wallet"
                    style={{ width: '24px', height: '24px' }}
                />
            ),
            showInstalled: true,
            onConnect: () => IcpConnect(),
        },
        // {
        //     type: ConnectWalletType.UNISAT,
        //     name: 'Bitfinity Wallet',
        //     icon: <UnisatIcon />,
        //     showInstalled: true,
        //     onConnect: () => connect(ConnectWalletType.UNISAT, false),
        // },
    ]
    const IcpConnect = async () => {
        try {
            const publicKey = await globalThis.ic.plug.requestConnect({
                whitelist: [nftCanisterId],
                host: icphost
                //host: 'http://localhost:4943'
            });
            console.log(`The connected user's public key is:`, publicKey);

            const principalId = await globalThis.ic.plug.agent.getPrincipal();
            if (BindIcpWallet) {
                if (BindIcpWallet !== principalId.toText()) {
                    onOpen()
                    return;
                }
            }
            setIcpWallet(principalId.toText())
            get_icp_nft()
        } catch (e) {
            console.error('Error connecting to Plug or fetching NFTs:', e);
        }
    }
    return (
        <Flex direction={'column'} gap={3}>
            <Modal closeOnOverlayClick={false} isOpen={isOpen} onClose={onClose}>
                <ModalOverlay />
                <ModalContent>
                    <ModalHeader>Error Icp Wallet</ModalHeader>
                    <ModalCloseButton />
                    <ModalBody pb={6}>

                    </ModalBody>

                    <ModalFooter>
                        <Button colorScheme='blue' mr={3}>
                            Save
                        </Button>
                        <Button onClick={onClose}>Cancel</Button>
                    </ModalFooter>
                </ModalContent>
            </Modal>
            {
                wallets.map(({ icon, onConnect, name, type, showInstalled }) => (
                    <Button leftIcon={icon} isLoading={connectingWallet === type} key={type} h={'56px'} colorScheme='gray' variant='outline' loadingText="connecting" spinnerPlacement='end' fontWeight={500} p={4} onClick={onConnect}>
                        <Flex px={4} flex={1} >{name}</Flex>
                        {
                            (isInstalled(type) && showInstalled) && <Tag size='sm' variant='solid' colorScheme='green'> installed </Tag>
                        }
                    </Button>
                ))
            }
        </Flex>
    )
};

const PlugWalletLogoIcon = () => (
    <svg width="24" height="24" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
        <rect width="24" height="24" rx="12" fill="#111111" />
        <path d="M4 6.87221C4 6.33933 4.43199 5.90735 4.96488 5.90735H19.5467C20.0796 5.90735 20.5116 6.33933 20.5116 6.87221V11.6786C20.5116 16.2381 16.8154 19.9344 12.2558 19.9344C7.69625 19.9344 4 16.2381 4 11.6786V6.87221Z" fill="#00F0FF" />
        <path d="M19.8579 14.9048C18.6018 17.8617 15.671 19.9354 12.2558 19.9354C7.69625 19.9354 4 16.2392 4 11.6796V8.20769C5.56805 7.34189 7.37087 6.84912 9.28878 6.84912C14.3358 6.84912 18.5859 10.2614 19.8579 14.9048Z" fill="#10D9ED" />
        <path d="M17.884 17.7252C16.4093 19.0998 14.4308 19.9409 12.2558 19.9409C7.69625 19.9409 4 16.2446 4 11.685V10.8025C5.39731 9.92864 7.04887 9.42358 8.8184 9.42358C13.5752 9.42358 17.4796 13.0733 17.884 17.7252Z" fill="#FA51D3" />
        <path d="M15.0339 19.4998C14.8622 15.6891 11.7298 12.6525 7.89081 12.6525C6.55653 12.6525 5.30759 13.0193 4.23865 13.6579C4.9033 16.3811 6.907 18.5767 9.50935 19.5014V19.9629C9.50935 20.631 9.86829 21.2149 10.4032 21.5311V22.7877C10.4032 23.4573 10.944 24 11.611 24H12.9276C13.5947 24 14.1355 23.4573 14.1355 22.7877V21.5311C14.6704 21.2149 15.0293 20.631 15.0293 19.9629V19.5014C15.0309 19.5009 15.0324 19.5003 15.0339 19.4998Z" fill="#FFE700" />
        <path d="M7.08093 0.366916C7.08093 0.164274 7.24521 0 7.44785 0H9.12343C9.32607 0 9.49034 0.164274 9.49034 0.366916V5.90734H7.08093V0.366916Z" fill="#031514" />
        <path d="M14.9581 0.366916C14.9581 0.164274 15.1224 0 15.325 0H17.0006C17.2033 0 17.3675 0.164274 17.3675 0.366916V5.90734H14.9581V0.366916Z" fill="#031514" />
        <circle cx="12" cy="12" r="5" fill="#111111" />
        <circle cx="12" cy="12" r="4" fill="white" />
    </svg>
);

const WizzLogoIcon = () => (
    <svg id="wizzLogo" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" width="24" height="24">
        <g transform="translate(0 0)">
            <path fillRule="evenodd" clipRule="evenodd" d="M17.7666 9.35038L16.2082 7.79199L14.6498 9.35038L16.2082 10.9088L17.7666 12.4672L19.3249 10.9088L17.7666 9.35038Z" fill="#FFD815" />
            <path fillRule="evenodd" clipRule="evenodd" d="M17.7664 0L16.208 1.55839L17.7664 3.11678L19.3248 1.55839L17.7664 0Z" fill="#FF9813" />
            <path fillRule="evenodd" clipRule="evenodd" d="M24 6.23344L22.4416 4.67505L20.8832 6.23344L22.4416 7.79183L24 6.23344Z" fill="#FF9813" />
            <path fillRule="evenodd" clipRule="evenodd" d="M14.6498 18.7006L16.2082 20.259L17.7666 18.7006L16.2082 17.1422L14.6498 18.7006Z" fill="#FF9813" />
            <path fillRule="evenodd" clipRule="evenodd" d="M3.74097 7.79191L5.29936 9.3503L6.85774 7.79191L5.29936 6.23352L3.74097 7.79191Z" fill="#FF9813" />
            <path fillRule="evenodd" clipRule="evenodd" d="M11.5331 0L9.97473 1.55839L11.5331 3.11678L13.0914 1.55839L11.5331 0Z" fill="#FF9813" />
            <path fillRule="evenodd" clipRule="evenodd" d="M5.29956 3.11686L6.85795 4.67525L8.41634 3.11686L6.85795 1.55847L5.29956 3.11686Z" fill="#FF9813" />
            <path fillRule="evenodd" clipRule="evenodd" d="M22.4416 10.9087L20.8832 12.4671L22.4416 14.0255L24 12.4671L22.4416 10.9087Z" fill="#FF9813" />
            <path fillRule="evenodd" clipRule="evenodd" d="M19.325 17.1421L20.8833 18.7005L22.4417 17.1421L20.8833 15.5837L19.325 17.1421Z" fill="#FF9813" />
            <path fillRule="evenodd" clipRule="evenodd" d="M17.7665 12.4669L16.2081 10.9085L14.6497 12.4669L13.0913 10.9085L11.533 9.35011L13.0913 7.79172L11.533 6.23344L9.97467 4.67505L8.41629 6.23344L9.97467 7.79172L8.41629 9.35011L9.97467 10.9085L8.41629 12.4669L7.63704 11.6877L6.07865 13.246L7.63704 14.8044L6.07865 16.3628L4.59778 17.8438L3.11677 19.3248L1.55839 20.8832L0 22.4415L1.55839 23.9999L3.11677 22.4415L4.67517 20.8832L6.15603 19.4022L6.23355 19.3248L7.63704 17.9212L7.71442 17.8438L9.19543 16.3628L10.7538 17.9212L12.3122 16.3628L11.533 15.5837L13.0913 14.0253L14.6497 15.5837L16.2081 14.0253L17.7665 15.5837L19.3249 14.0253L17.7665 12.4669Z" fill="#5B5B72" />
            <path fillRule="evenodd" clipRule="evenodd" d="M17.7665 6.23344L16.2081 4.67505L14.6497 6.23344L13.0913 4.67505L11.533 6.23344L13.0913 7.79183L11.533 9.35021L13.0913 10.9086L14.6497 12.4669L16.2081 10.9086L14.6497 9.35021L16.2081 7.79183L17.7665 9.35021L19.3249 7.79183L17.7665 6.23344Z" fill="#FF9813" />
        </g>
    </svg>
);
import React, { useState, useEffect } from 'react';
import {
    useToast,
    Modal,
    ModalOverlay,
    ModalContent,
    ModalHeader,
    ModalCloseButton,
    ModalBody,
    ModalFooter,
    Button,
    FormControl,
    FormLabel,
    Input,
    Select,
    Image,
    Flex,
    TabList,
    Tabs,
    Tab,
    TabPanels,
    TabPanel,
    VStack,
    Text
} from '@chakra-ui/react';
import { CurrentStatus } from '../../store';
import { IcpWalletStore, StakeNftInfo } from '../../store/useWalletStore';
import useGetWalletPool from '../../utils/walletActor';
interface CreateWalletModalProps {
    isStake: boolean;
    isOpen: boolean;
    onClose: () => void;
    stakeBackend;
    stakeogBackend;
}

export const OgStakeModal: React.FC<CreateWalletModalProps> = ({
    isStake,
    isOpen,
    onClose,
    stakeBackend,
    stakeogBackend,
}) => {
    const [selectedTabIndex, setSelectedTabIndex] = useState(1);
    useEffect(() => {
        if (isStake) {
            setSelectedTabIndex(0);
        } else {
            setSelectedTabIndex(1);
        }
    }, [isStake]);


    const [walletName, setWalletName] = React.useState<string>("");
    const { isLoading, setIsLoading } = CurrentStatus()
    const { get_user_stakednft, stake_in_nft, unstake_in_nft } = useGetWalletPool()
    const { icpNft } = IcpWalletStore()
    const { stakedNFTs } = StakeNftInfo()

    const [selectedNftId, setSelectedNftId] = useState('');
    const handleNftSelection = (event) => {
        setSelectedNftId(event.target.value);
    };
    const [selectedunstakeNftId, setSelectedunstakeNftId] = useState('');
    const handleunstakeNftSelection = (event) => {
        setSelectedunstakeNftId(event.target.value);
    };

    useEffect(() => {
        if (icpNft && icpNft.length > 0) {
            setSelectedNftId(icpNft[0].toString());
        } else (
            setSelectedNftId('')
        )
        if (stakedNFTs && stakedNFTs.length > 0) {
            setSelectedunstakeNftId(stakedNFTs[0].nft_id.toString());
        } else {
            setSelectedunstakeNftId('')
        }
    }, [icpNft]);

    useEffect(() => {
        if (isOpen) {
            setGetResult(false);
            setStakeResult("");
        }
    }, [isOpen]);


    const toast = useToast();

    const [getResult, setGetResult] = useState<boolean>(false)
    const [stakeResult, setStakeResult] = useState("")

    const onApprove_stake = async () => {
        if (!selectedNftId) {
            alert("Please select an NFT first.");
            return;
        }
        setIsLoading(true);
        try {
            const res = await stake_in_nft(stakeBackend, Number(selectedNftId));
            console.log(res)
            if ('Err' in res) {
                setGetResult(true)
                const errorMessages = Object.values(res.Err).join(', ');
                setStakeResult(errorMessages || "An error occurred during the staking process.");
                return;
            }
            if ('Ok' in res) {
                get_user_stakednft(stakeogBackend)
                setGetResult(true)
                setStakeResult("Stake Success");
                return;
            }
            // Handle successful staking (e.g., show success message, refresh NFT list, etc.)
            onClose();
        } catch (error) {
            console.error("Error staking NFT:", error);
            alert("Failed to stake NFT. Please try again.");
        } finally {
            setIsLoading(false);
        }
    };

    const onApprove_unstake = async () => {
        if (!selectedunstakeNftId) {
            alert("Please select an NFT first.");
            return;
        }
        setIsLoading(true);
        try {
            const res = await unstake_in_nft(stakeBackend, Number(selectedunstakeNftId));
            console.log(res)
            if ('Err' in res) {
                setGetResult(true)
                const errorMessages = Object.values(res.Err).join(', ');
                setStakeResult(errorMessages || "An error occurred during the unstaking process.");
                return;
            }
            if ('Ok' in res) {
                get_user_stakednft(stakeogBackend)
                setGetResult(true)
                setStakeResult("UnStake Success");
                return;
            }
            // Handle successful staking (e.g., show success message, refresh NFT list, etc.)
            onClose();
        } catch (error) {
            console.error("Error unstaking NFT:", error);
            alert("Failed to unstake NFT. Please try again.");
        } finally {
            setIsLoading(false);
        }
    };

    const onBack = async () => {
        onClose()
        setGetResult(false)
        setStakeResult("")
    }

    return (
        <Modal
            isOpen={isOpen}
            onClose={onClose}
            isCentered
        >
            <ModalOverlay />
            <ModalContent pt='10' pb='8' borderRadius="3xl">
                <ModalHeader></ModalHeader>
                <ModalCloseButton />
                <ModalBody pb={6}>
                    {getResult ? (
                        <Flex justifyContent="center" alignItems="center" height="100%">
                            <VStack spacing={4}>
                                <Text fontSize="xl" fontWeight="bold">
                                    Result
                                </Text>
                                <Text>
                                    {stakeResult}
                                </Text>
                                {/* Add any additional information or actions here */}
                            </VStack>
                        </Flex>
                    ) : (
                        <Tabs variant='unstyled' colorScheme='green' index={selectedTabIndex} onChange={setSelectedTabIndex}>
                            <Flex
                                justifyContent="center"
                                alignItems='center'
                                mb={4}
                            >
                                <TabList
                                    bgColor='#F7FAFC'
                                    width='220px'
                                    height='40px'
                                    justifyContent="center"
                                    alignItems='center'
                                >
                                    <Tab width='104px' height='32px' _selected={{ width: '104px', color: 'black', bg: 'white' }}>Stake</Tab>
                                    <Tab width='104px' height='32px' _selected={{ width: '104px', color: 'black', bg: 'white' }}>Unstake</Tab>
                                </TabList>
                            </Flex>
                            <TabPanels>
                                <TabPanel>
                                    <Flex justifyContent="center">
                                        <FormControl width='340px'>
                                            <FormLabel textAlign="left">Select NFT</FormLabel>
                                            {icpNft && icpNft.length > 0 ? (
                                                <Select onChange={handleNftSelection} value={selectedNftId}>
                                                    {icpNft.map((nftId) => (
                                                        <option key={nftId} value={nftId.toString()}>
                                                            BifiPal OG NFT #{nftId}
                                                        </option>
                                                    ))}
                                                </Select>
                                            ) : (
                                                <Select placeholder='No NFTs available' isDisabled>
                                                    <option>No NFTs found</option>
                                                </Select>
                                            )}
                                        </FormControl>

                                    </Flex>
                                    <Flex justifyContent="center" width='100%' mt='10'>
                                        {getResult ? (
                                            <Button
                                                isLoading={isLoading}
                                                loadingText='Approving...'
                                                width='340px'
                                                bgColor="#FFA033"
                                                color="black"
                                                _hover={{ bg: "purple.300", borderColor: "purple.500" }}
                                                onClick={onBack}
                                            >
                                                Next
                                            </Button>
                                        ) : (
                                            <Button
                                                isLoading={isLoading}
                                                loadingText='Approving...'
                                                width='340px'
                                                bgColor="#FFA033"
                                                color="black"
                                                _hover={{ bg: "purple.300", borderColor: "purple.500" }}
                                                onClick={onApprove_stake}
                                                isDisabled={!selectedNftId}
                                            >
                                                Approve
                                            </Button>
                                        )}
                                    </Flex>
                                </TabPanel>
                                <TabPanel>
                                    <Flex justifyContent="center">
                                        <FormControl width='340px'>
                                            <FormLabel textAlign="left">Select NFT to Unstake</FormLabel>
                                            {stakedNFTs && stakedNFTs.length > 0 ? (
                                                <Select onChange={handleunstakeNftSelection} value={selectedunstakeNftId}>
                                                    {stakedNFTs.map((nft) => (
                                                        <option key={nft.nft_id} value={nft.nft_id.toString()}>
                                                            BifiPal OG NFT #{nft.nft_id}
                                                        </option>
                                                    ))}
                                                </Select>
                                            ) : (
                                                <Select placeholder='No NFTs available' isDisabled>
                                                    <option>No NFTs found</option>
                                                </Select>
                                            )}
                                        </FormControl>
                                    </Flex>
                                    <Flex justifyContent="center" width='100%' mt='10'>
                                        {getResult ? (
                                            <Button
                                                isLoading={isLoading}
                                                loadingText='Approving...'
                                                width='340px'
                                                bgColor="#FFA033"
                                                color="black"
                                                _hover={{ bg: "purple.300", borderColor: "purple.500" }}
                                                onClick={onBack}
                                            >
                                                Next
                                            </Button>
                                        ) : (
                                            <Button
                                                isLoading={isLoading}
                                                loadingText='Approving...'
                                                width='340px'
                                                bgColor="#FFA033"
                                                color="black"
                                                _hover={{ bg: "purple.300", borderColor: "purple.500" }}
                                                onClick={onApprove_unstake}
                                                //isDisabled={true}
                                                isDisabled={!selectedunstakeNftId}
                                            >
                                                Approve
                                            </Button>
                                        )}
                                    </Flex>
                                </TabPanel>
                            </TabPanels>
                        </Tabs>
                    )}
                </ModalBody>

                <ModalFooter>

                </ModalFooter>
            </ModalContent>
        </Modal>
    );
};
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
import { IcpWalletStore } from '../../store/useWalletStore';
import useGetWalletPool from '../../utils/walletActor';
interface BindIcpModalProps {
    isOpen: boolean;
    onClose: () => void;
    osBackend;
}

export const BindIcpModal: React.FC<BindIcpModalProps> = ({
    isOpen,
    onClose,
    osBackend,
}) => {
    const [walletName, setWalletName] = React.useState<string>("");
    const { isLoading, setIsLoading } = CurrentStatus()
    const { bind_icpWallet } = useGetWalletPool()




    const toast = useToast();

    const [getResult, setGetResult] = useState<boolean>(false)
    const [stakeResult, setStakeResult] = useState("")

    const onBindIcp = async () => {
        setIsLoading(true);
        try {
            const res = await bind_icpWallet(osBackend);
            console.log(res)
            if ('Err' in res) {
                setGetResult(true)
                const errorMessages = Object.values(res.Err).join(', ');
                setStakeResult(errorMessages || "An error occurred during the Bindind process.");
                return;
            }
            if ('Ok' in res) {
                setGetResult(true)
                setStakeResult("Bind Success");
                return;
            }
            // Handle successful staking (e.g., show success message, refresh NFT list, etc.)
            onClose();
        } catch (error) {
            setGetResult(true)
            setStakeResult("An error occurred during the Bindind process.");
            console.error("Error Bind Wallet:", error);
            alert("Failed to Bind Icp Wallet. Please try again.");
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
                        <Flex justifyContent='center'>
                            Bind Icp Wallet
                        </Flex>
                    )}
                </ModalBody>

                <ModalFooter>
                    <Flex justifyContent="center" width='100%'>
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
                            <Flex>
                                <Button
                                    isLoading={isLoading}
                                    loadingText='Binding...'
                                    width='150px'
                                    bgColor="#FFA033"
                                    color="black"
                                    _hover={{ bg: "purple.300", borderColor: "purple.500" }}
                                    onClick={onBindIcp}
                                >
                                    Bind
                                </Button>
                                <Button
                                    ml='3'
                                    width='150px'
                                    bgColor="grey.200"
                                    color="black"
                                    _hover={{ bg: "purple.300", borderColor: "purple.500" }}
                                    onClick={onClose}
                                >
                                    Cancel
                                </Button>
                            </Flex>
                        )}
                    </Flex>
                </ModalFooter>
            </ModalContent>
        </Modal>
    );
};
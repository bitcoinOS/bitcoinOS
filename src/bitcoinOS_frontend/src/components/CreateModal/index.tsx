import React from 'react';
import {
    useToast,
    Modal,
    ModalOverlay,
    ModalContent,
    ModalHeader,
    ModalBody,
    ModalFooter,
    Button,
    FormControl,
    FormLabel,
    Input,
    Image,
    Flex
} from '@chakra-ui/react';
import { CurrentStatus } from '../../store';
import useGetWalletPool from '../../utils/walletActor';
interface CreateWalletModalProps {
    isOpen: boolean;
    onClose: () => void;
    osBackend;
    identity;
}

export const CreateWalletModal: React.FC<CreateWalletModalProps> = ({
    isOpen,
    onClose,
    osBackend,
    identity,
}) => {
    const [walletName, setWalletName] = React.useState<string>("");
    const { isLoading, setIsLoading } = CurrentStatus()
    const { get_wallets } = useGetWalletPool()
    const toast = useToast();
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
            //get_wallet_count()
            setIsLoading(false)
        } catch (error) {
            console.log(error)
            setIsLoading(false)
        } finally {
            setIsLoading(false)
            console.log('end')
        }


    }

    return (
        <Modal
            isOpen={isOpen}
            onClose={onClose}
            isCentered
            closeOnOverlayClick={false}
        >
            <ModalOverlay />
            <ModalContent pt='10' pb='8' borderRadius="3xl">
                <ModalHeader></ModalHeader>
                <Image src='./home/createWallet.svg' />
                <ModalBody pb={6}>
                    <Flex justifyContent="center">
                        <FormControl width='70%'>
                            <FormLabel textAlign="center">You need to create a wallet to continue using our products</FormLabel>
                            <Input
                                placeholder="wallet name"
                                value={walletName}
                                onChange={event => setWalletName(event.currentTarget.value)}
                            />
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
                            onClick={onCreateWallet}
                        >
                            Create Now
                        </Button>
                    </Flex>
                </ModalFooter>
            </ModalContent>
        </Modal>
    );
};
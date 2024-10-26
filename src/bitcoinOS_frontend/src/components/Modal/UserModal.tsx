import React, { useState } from 'react';
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
    Text,
    Select,
    Image,
    Flex,
    InputGroup,
    InputRightElement,
} from '@chakra-ui/react';
import { CurrentStatus } from '../../store';
import useGetWalletPool from '../../utils/walletActor';
import AvatarSelector from '../UserInfo'
import useGetMarathon from '../../utils/marathonActor';
interface CreateWalletModalProps {
    isOpen: boolean;
    onClose: () => void;
    osBackend;
}

export const UserModal: React.FC<CreateWalletModalProps> = ({
    isOpen,
    onClose,
    osBackend,
}) => {
    const [walletName, setWalletName] = React.useState<string>("");
    const { isLoading, setIsLoading } = CurrentStatus()
    const { get_wallets, get_user_image, update_user_info } = useGetWalletPool()
    const { get_info } = useGetMarathon();
    const toast = useToast();

    const MAX_LENGTH = 15;

    const [username, setUsername] = useState('');
    const [selectedAvatar, setSelectedAvatar] = useState(null);

    const [updateLoading, setUpdateLoading] = useState<boolean>(false)

    const handleUsernameChange = (e) => {
        const value = e.target.value;
        if (value.length <= MAX_LENGTH) {
            setUsername(value);
        }
    };

    const onUpdate = async () => {
        setUpdateLoading(true)
        const res = await update_user_info(osBackend, username, selectedAvatar)
        if (res) {
            await get_info(osBackend)
        }
        setUpdateLoading(false)
    }

    const test = async () => {
        console.log(selectedAvatar)
    }

    return (
        <Modal
            isOpen={isOpen}
            onClose={onClose}
            isCentered
        >
            <ModalOverlay />
            <ModalContent pt='10' pb='8' borderRadius="3xl">
                <ModalHeader>
                    <Text
                        fontSize='24px'
                        fontWeight='700'
                    >
                        Edit Portfolio
                    </Text>
                </ModalHeader>
                <ModalCloseButton />
                <ModalBody pb={6}>
                    <Flex direction='column' alignItems='center' justifyContent="center">
                        {/*
                        <Button onClick={test}>test</Button>
                        */}
                        <AvatarSelector
                            selectedAvatar={selectedAvatar}
                            onAvatarSelect={setSelectedAvatar}
                        />
                        <FormControl width='340px'>
                            <FormLabel textAlign="left">Username</FormLabel>
                            <InputGroup>
                                <Input
                                    placeholder='Name'
                                    value={username}
                                    onChange={handleUsernameChange}
                                />
                                <InputRightElement width="4.5rem">
                                    <Text fontSize="sm" color="gray.500">
                                        {username.length}/{MAX_LENGTH}
                                    </Text>
                                </InputRightElement>
                            </InputGroup>
                        </FormControl>
                    </Flex>
                </ModalBody>

                <ModalFooter>
                    <Flex justifyContent="center" width='100%'>
                        <Button
                            isLoading={updateLoading}
                            loadingText='Update ...'
                            width='340px'
                            bgColor="#FFA033"
                            color="black"
                            _hover={{ bg: "purple.300", borderColor: "purple.500" }}
                            onClick={onUpdate}
                        >
                            Approve
                        </Button>
                    </Flex>
                </ModalFooter>
            </ModalContent>
        </Modal>
    );
};
import {
    Modal,
    ModalContent,
    ModalBody,
    ModalCloseButton,
    Text,
    ModalOverlay,
    ModalHeader
} from '@chakra-ui/react';
export default function IcpModal({
    isOpen,
    onClose,
    children,
    title,
    closeOnOverlayClick = false
}: Readonly<{
    title?: string | React.ReactNode;
    isOpen: boolean;
    onClose: () => void;
    closeOnOverlayClick?: boolean;
    children?: React.ReactNode;
}>) {
    return (
        <Modal closeOnOverlayClick={closeOnOverlayClick} isOpen={isOpen} onClose={onClose}>

            <ModalOverlay />
            <ModalContent borderRadius={24}>
                <ModalCloseButton top={4} right={4} />
                <ModalHeader fontSize='24px'>Connect Wallet</ModalHeader>
                <ModalBody px={8} pb={8} pt={2}>
                    {
                        title && (
                            typeof title === 'string'
                                ? <Text fontSize={'2xl'} fontWeight={700} pb="6">{title}</Text>
                                : title
                        )
                    }
                    {children}
                </ModalBody>
            </ModalContent>
        </Modal>
    )
};

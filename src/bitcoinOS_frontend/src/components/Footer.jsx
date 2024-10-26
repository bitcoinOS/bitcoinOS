
import { useEffect, useState } from "react";

import {
    Box,
    Stack,
    Heading,
    Flex,
    Text,
    Button,
    useDisclosure,
    Icon,
    IconButton,
    HStack,
    VStack,
    Image,
    Spacer
} from "@chakra-ui/react";

import { Link as ChakraLink } from '@chakra-ui/react'
import {
    BsChevronDown,
    BsCopy,
    BsGithub,
    BsMoonFill,
    BsPerson,
    BsPower,
    BsSunFill,
    BsTwitterX,
} from 'react-icons/bs'
import { useInternetIdentity } from "ic-use-internet-identity";
// import UserStore from "../store/index"
import { LoginButton } from "./LoginButton"


const Footer = () => {
    const { identity } = useInternetIdentity();
    return (
        <Flex
            as="nav"
            align="center"
            wrap="wrap"
            pt={6}
            pb={3}
            justifyContent='center'
            zIndex={2}
        >
            <Flex
                maxWidth='1120px'
                width='78%'
                bgColor='#FFFFFF'
                border='1px solid #CBD5E0'
                borderRadius='3xl'
                p='12'
                px='20'
            >
                <Flex
                    fontSize={{ md: '20px', lg: '22px', xl: '25px', '2xl': '35px', '3xl': '35px' }}
                    width='100%'
                    mb='1%'
                    direction='column'
                    border
                >
                    <Image height={{ md: '36px', lg: '45px', xl: '50px', '2xl': '50px', '3xl': '50px' }} src="./logo.png" marginLeft='0' >
                    </Image>
                    <Text
                        fontFamily='Ubuntu'
                        fontWeight='400'
                        fontStyle='italic'
                        fontSize={{ md: '15px', lg: '15px', xl: '15px', '2xl': '16px', '3xl': '16px' }}
                        lineHeight='16px'
                        mt='2'
                    >Make Bitcoin the Pioneer in Finance</Text>
                </Flex>
                <Flex>
                    <HStack gap='5' display={{ base: 'none', md: 'flex' }}>
                        <ChakraLink
                            isExternal
                            aria-label='Go to BifiPal X page'
                            href='https://bifipal.gitbook.io/bifipal-introduction'
                        >
                            <Icon
                                as={props => <img src="/footer/book.svg" {...props} />}
                                display='block'
                                transition='color 0.2s'
                                fontSize={{ md: '30px', lg: '35px', xl: '40px', '2xl': '40px', '3xl': '40px' }}
                                _hover={{ color: 'gray.600' }}
                            />
                        </ChakraLink>
                        <ChakraLink
                            isExternal
                            aria-label='Go to BifiPal GitHub page'
                            href="https://github.com/bitcoinOS/bitcoinOS"
                        >
                            <Icon
                                ml='10'
                                as={BsGithub}
                                display='block'
                                transition='color 0.2s'
                                fontSize={{ md: '30px', lg: '35px', xl: '40px', '2xl': '40px', '3xl': '40px' }}
                                _hover={{ color: 'gray.600' }}
                            />
                        </ChakraLink>
                        <ChakraLink
                            isExternal
                            aria-label='Go to BifiPal X page'
                            href='https://twitter.com/BifiPal'
                        >
                            <Icon
                                ml='10'
                                as={BsTwitterX}
                                display='block'
                                transition='color 0.2s'
                                fontSize={{ md: '30px', lg: '35px', xl: '40px', '2xl': '40px', '3xl': '40px' }}
                                _hover={{ color: 'gray.600' }}
                            />
                        </ChakraLink>

                    </HStack>
                </Flex>
                {/*
            <Flex
                width='78%'
                wrap="wrap"
            >
                <Flex
                    fontSize={{ md: '20px', lg: '22px', xl: '25px', '2xl': '35px', '3xl': '35px' }}
                    width='100%'
                    mb='1%'
                    direction='row'
                    border
                >
                    <Flex width='35%' mr='10%' alignItems='flex-start' justifyContent='flex-start'><Image height='50px' src="./logo.png" marginLeft='0' ></Image></Flex>
                    <Flex width='15%'><Text fontWeight='bold'>Docs</Text></Flex>
                    <Flex width='15%'><Text fontWeight='bold'>Community</Text></Flex>
                    <Flex width='15%'><Text fontWeight='bold'>About</Text></Flex>
                </Flex>
                <Flex width='100%' fontSize={{ md: '15px', lg: '18px', xl: '20px', '2xl': '22px', '3xl': '25px' }}>
                    <Flex
                        width='35%'
                        mr='10%'
                    >
                        <div>A Decentralized Bitcoin Finance & Assets Management Platform</div>
                    </Flex>
                    <VStack
                        spacing={4}
                        align='stretch'
                        width='15%'
                    >
                        <Box h='40px'>
                            <div>bitcoinOS</div>
                        </Box>
                        <Box h='40px'>
                            <div>ERC-3525</div>
                        </Box>
                    </VStack>
                    <VStack
                        spacing={4}
                        align='stretch'
                        width='15%'
                    >
                        <Box h='40px'>
                            <div>Twitter</div>
                        </Box>
                        <Box h='40px'>
                            <div>Telegram</div>
                        </Box>
                        <Box h='40px'>
                            <div>Github</div>
                        </Box>
                        <Box h='40px'>
                            <div>Discord</div>
                        </Box>
                    </VStack>
                    <VStack
                        spacing={4}
                        align='stretch'
                        width='15%'
                    >
                        <Box h='40px'>
                            <div>Blog</div>
                        </Box>
                        <Box h='40px'>
                            <div>Contact</div>
                        </Box>
                        <Box h='40px'>
                            <div>Apply</div>
                        </Box>
                        <Box h='40px'>
                            <div>bitcoinOS Protocol</div>
                        </Box>
                    </VStack>
                </Flex>
                */}
            </Flex>

        </Flex>
    );
};

export default Footer;
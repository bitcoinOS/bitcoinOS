import React from 'react';
import { Flex, Box, Image, Text, Divider, Button } from '@chakra-ui/react';

import Footer from '../../components/Footer';

const Reward: React.FC = () => {

    const gradientTextStyle = {
        backgroundClip: "text",
        textFillColor: "transparent",
        backgroundImage: "linear-gradient(to right, #AA00FF 0%, #FF8901 100%)",
        WebkitBackgroundClip: "text",
        WebkitTextFillColor: "transparent",
    };

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
                    p='6'
                    width='1120px'
                    height='400PX'
                    bgColor='white'
                    borderRadius='16px'
                    zIndex='1'
                >
                    <Image src='/marathon/rewards_credit.png' width='240px' />
                    <Flex
                        direction='column'
                        width='750px'
                    >
                        <Flex width='100%' justifyContent='space-between' alignItems="flex-end">
                            <Text fontSize='32px' fontWeight='700'>BifiPal Points</Text>
                            <Text fontSize='20px' fontWeight='500'>Total: unlimited</Text>
                        </Flex>
                        <Text mt='6' fontSize='20px' fontWeight='400'>Every box contains points, ensuring each box includes a certain amount of points as a basic reward</Text>
                    </Flex>
                </Flex>
            </Flex>
            <Flex
                mt='10'
                justifyContent='center'
            >
                <Flex
                    p='6'
                    width='1120px'
                    height='400PX'
                    bgColor='white'
                    borderRadius='16px'
                    zIndex='1'
                >
                    <Image src='/marathon/rewards_boost.png' width='240px' />
                    <Flex
                        direction='column'
                        width='750px'
                    >
                        <Flex width='100%' justifyContent='space-between' alignItems="flex-end">
                            <Text fontSize='32px' fontWeight='700'>Box Boost Cards</Text>
                            <Text fontSize='20px' fontWeight='500'>Total: unlimited</Text>
                        </Flex>
                        <Text mt='6' fontSize='20px' fontWeight='400'>Divided into Epic, Rare, and Legendary levels, these cards can be used to speed up the process of opening boxes, allowing you to open more boxes in less time.</Text>
                    </Flex>
                </Flex>
            </Flex>
            <Flex
                mt='10'
                justifyContent='center'
            >
                <Flex
                    p='6'
                    width='1120px'
                    height='400PX'
                    bgColor='white'
                    borderRadius='16px'
                    zIndex='1'
                >
                    <Image src='/marathon/rewards_og.png' width='240px' />
                    <Flex
                        direction='column'
                        width='750px'
                    >
                        <Flex width='100%' justifyContent='space-between' alignItems="flex-end">
                            <Text fontSize='32px' fontWeight='700'>BifiPal OG Box NFT</Text>
                            <Text fontSize='20px' fontWeight='500'>999/1000</Text>
                        </Flex>
                        <Text mt='6' fontSize='20px' fontWeight='400'>OGNFT holders enjoy special privileges and benefits on the platform and can increase their box opening rate.</Text>
                    </Flex>
                </Flex>
            </Flex>
            <Flex
                mt='10'
                justifyContent='center'
            >
                <Flex
                    p='6'
                    width='1120px'
                    height='400PX'
                    bgColor='white'
                    borderRadius='16px'
                    zIndex='1'
                >
                    <Image src='/marathon/rewards_offer.png' width='240px' />
                    <Flex
                        direction='column'
                        width='750px'
                    >
                        <Flex width='100%' justifyContent='space-between' alignItems="flex-end">
                            <Text fontSize='32px' fontWeight='700'>Private Placement Tickets</Text>
                            <Text fontSize='20px' fontWeight='500'>0/4000</Text>
                        </Flex>
                        <Text mt='6' fontSize='20px' fontWeight='400'>Private Placement Tickets allow you to participate in the project's public sale at the best token prices.</Text>
                    </Flex>
                </Flex>
            </Flex>
            <Flex
                mt='10'
                justifyContent='center'
            >
                <Flex
                    p='6'
                    width='1120px'
                    height='400PX'
                    bgColor='white'
                    borderRadius='16px'
                    zIndex='1'
                >
                    <Image src='/marathon/rewards_mysterious.png' width='240px' />
                    <Flex
                        direction='column'
                        width='750px'
                    >
                        <Flex width='100%' justifyContent='space-between' alignItems="flex-end">
                            <Text fontSize='32px' fontWeight='700'>Mysterious Airdrops</Text>
                        </Flex>
                        <Text mt='6' fontSize='20px' fontWeight='400'>Including but not limited to Inscriptions, runes, NFTs, tokens, card items, and more. Discover and uncover all the hidden treasures waiting for you!</Text>
                    </Flex>
                </Flex>
            </Flex>
            <Flex
                mt='10'
                justifyContent='center'
            >
                <Flex
                    p='6'
                    width='1120px'
                    height='1019px'
                    bgColor='white'
                    borderRadius='16px'
                    direction='column'
                    alignItems='center'
                    zIndex='1'
                >
                    <Flex
                        width='1060px'
                        fontSize='24px'
                        fontWeight='700'
                    >
                        Roadmap
                    </Flex>
                    <Flex
                        width='1060px'
                        height='378px'
                        bgColor='#F5F5F5'
                        borderRadius='8px'
                        mt='2'
                        direction='column'
                    >
                        <Flex
                            color='#2D3748'
                            fontSize='32px'
                            fontWeight='700'
                            height='100px'
                            width='100%'
                            pl='10'
                            alignItems='center'
                            borderBottom='1px'
                        >
                            Q3 2024
                        </Flex>
                        <Flex
                            color='#2D3748'
                            fontSize='32px'
                            fontWeight='700'
                            height='130px'
                            width='100%'
                            pt='6'
                            borderBottom='1px'
                        >
                            <Flex width='230px' justifyContent="flex-end" mr='6'>
                                <Text
                                    fontSize='24px'
                                    fontWeight='700'
                                    sx={gradientTextStyle}
                                >
                                    August 2024
                                </Text>
                            </Flex>
                            <Flex direction='column'>
                                <Text fontSize='24px' fontWeight='700'>Public Beta Launch:</Text>
                                <Text fontSize='20px' fontWeight='400'>Explore and test our new platform.</Text>
                                <Text fontSize='20px' fontWeight='400'>Participate in initial token distribution and community activities.</Text>
                            </Flex>
                        </Flex>
                        <Flex
                            color='#2D3748'
                            fontSize='32px'
                            fontWeight='700'
                            width='100%'
                            pt='6'
                        >
                            <Flex width='230px' justifyContent="flex-end" mr='6'>
                                <Text
                                    fontSize='24px'
                                    fontWeight='700'
                                    sx={gradientTextStyle}
                                >
                                    September 2024
                                </Text>
                            </Flex>
                            <Flex direction='column'>
                                <Text fontSize='24px' fontWeight='700'>Boost Feature Launch:</Text>
                                <Text fontSize='20px' fontWeight='400'>Enhance your experience with our new boost feature.</Text>
                                <Text fontSize='20px' fontWeight='400'>Invite friends and earn special rewards.</Text>
                            </Flex>
                        </Flex>
                    </Flex>
                    <Flex
                        width='1060px'
                        height='513px'
                        bgColor='#F5F5F5'
                        borderRadius='8px'
                        mt='6'
                        direction='column'
                    >
                        <Flex
                            color='#2D3748'
                            fontSize='32px'
                            fontWeight='700'
                            height='100px'
                            width='100%'
                            pl='10'
                            alignItems='center'
                            borderBottom='1px'
                        >
                            Q4 2024
                        </Flex>
                        <Flex
                            color='#2D3748'
                            fontSize='32px'
                            fontWeight='700'
                            height='130px'
                            width='100%'
                            pt='6'
                            borderBottom='1px'
                        >
                            <Flex width='230px' justifyContent="flex-end" mr='6'>
                                <Text
                                    fontSize='24px'
                                    fontWeight='700'
                                    sx={gradientTextStyle}
                                >
                                    October 2024
                                </Text>
                            </Flex>
                            <Flex direction='column'>
                                <Text fontSize='24px' fontWeight='700'>Staking and Rewards:</Text>
                                <Text fontSize='20px' fontWeight='400'>Start staking BTC to earn various rewards.</Text>
                                <Text fontSize='20px' fontWeight='400'>Unlock exclusive OG NFTs and public sale access passes.</Text>
                            </Flex>
                        </Flex>
                        <Flex
                            color='#2D3748'
                            fontSize='32px'
                            fontWeight='700'
                            width='100%'
                            pt='6'
                            height='135px'
                            borderBottom='1px'
                        >
                            <Flex width='230px' justifyContent="flex-end" mr='6'>
                                <Text
                                    fontSize='24px'
                                    fontWeight='700'
                                    sx={gradientTextStyle}
                                >
                                    November 2024
                                </Text>
                            </Flex>
                            <Flex direction='column'>
                                <Text fontSize='24px' fontWeight='700'>Public Sale Preparation:</Text>
                                <Text fontSize='20px' fontWeight='400'>Get ready for our public sale.</Text>
                                <Text fontSize='20px' fontWeight='400'>Participate in pre-sale activities and community events.</Text>
                            </Flex>
                        </Flex>
                        <Flex
                            color='#2D3748'
                            fontSize='32px'
                            fontWeight='700'
                            width='100%'
                            pt='6'
                        >
                            <Flex width='230px' justifyContent="flex-end" mr='6'>
                                <Text
                                    fontSize='24px'
                                    fontWeight='700'
                                    sx={gradientTextStyle}
                                >
                                    December 2024
                                </Text>
                            </Flex>
                            <Flex direction='column'>
                                <Text fontSize='24px' fontWeight='700'>Mainnet Launch:</Text>
                                <Text fontSize='20px' fontWeight='400'>Join us for the official launch of our mainnet.</Text>
                                <Text fontSize='20px' fontWeight='400'>Enjoy new features and enhanced platform capabilities.</Text>
                            </Flex>
                        </Flex>
                    </Flex>
                </Flex>
            </Flex>
            <Footer />
        </Box>
    );
};

export default Reward;
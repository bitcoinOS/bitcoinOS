import React from 'react';
import {
    Accordion,
    AccordionItem,
    AccordionButton,
    AccordionPanel,
    Box
} from '@chakra-ui/react';
import { BsPlusLg, BsDashLg } from "react-icons/bs";

const accordionData = [
    {
        title: 'What rewards are in the boxes',
        content: 'A: Each box contains base points. Additionally, it may include rare items, box acceleration cards, and cooperative community rewards.'
    },
    {
        title: 'What is a Private Placement Tickets',
        content: 'A Private Placement Tickets is a rare item that allows the holder to participate in project public offerings (fundraising events).'
    },
    {
        title: "What's the use of an OGNFT?",
        content: 'A: An OGNFT is a special NFT that provides the holder with special privileges on the platform.'
    },
    {
        title: 'How many levels of box acceleration cards are there?',
        content: 'A: Box acceleration cards come in three levels: Rare, Epic, and Legendary.'
    },
    {
        title: 'How do you use box acceleration cards?',
        content: 'A: Box acceleration cards can be used to increase the rate of opening boxes, allowing you to open boxes faster.'
    },
    {
        title: 'What does the cooperative community rewards include?',
        content: 'A: Cooperative community rewards may include token claim cards, access pass cards, NFT cards, etc.'
    },
    {
        title: 'Will there be more types of rewards?',
        content: 'A: Yes, we plan to introduce more diverse and colorful rewards.'
    },
    {
        title: "How can I stay updated with the platform's latest news?",
        content: 'A: You can follow our X (https://x.com/bifipal) or Telegram channels.（https://t.me/bifipal global）'
    },
    {
        title: 'How does the platform protect my account security?',
        content: 'A: We use advanced encryption technology to protect user data, and strongly recommend users enable two-factor authentication (2FA) to enhance account security'
    }
];

const CustomAccordion: React.FC = () => {
    return (
        <Accordion allowMultiple>
            {accordionData.map((item, index) => (
                <AccordionItem
                    sx={{ borderTop: 'none', borderBottom: 'none' }}
                    key={index}
                    mt='2'
                    mb='6'
                >
                    {({ isExpanded }) => (
                        <>
                            <h2>
                                <AccordionButton
                                    width='1060px'
                                    height='80px'
                                    bgColor='#F5F5F5'
                                    borderRadius='8px'
                                    fontSize='24px'
                                    fontWeight='700'
                                >
                                    <Box flex='1' textAlign='left'>
                                        {item.title}
                                    </Box>
                                    {isExpanded ? (
                                        <BsDashLg fontSize='24px' />
                                    ) : (
                                        <BsPlusLg fontSize='24px' />
                                    )}
                                </AccordionButton>
                            </h2>
                            <AccordionPanel pb={4}>
                                {item.content}
                            </AccordionPanel>
                        </>
                    )}
                </AccordionItem>
            ))}
        </Accordion>
    );
};

export default CustomAccordion;
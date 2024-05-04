
import { useEffect, useState } from "react";

import {
  Box,
  Stack,
  Heading,
  Flex,
  Text,
  Button,
  useDisclosure,
  HStack,
  HTMLChakraProps,
  Icon,
  IconButton,
  Square,
  chakra,
  useColorMode,
  Image,
  Spacer
} from "@chakra-ui/react";

import { Link as ChakraLink, LinkProps } from '@chakra-ui/react'
import { Link as ReactRouterLink } from 'react-router-dom'
import {
  BsGithub,
  BsMoonFill,
  BsSunFill,
  BsTwitterX,
} from 'react-icons/bs'
import { useInternetIdentity } from "ic-use-internet-identity";
// import UserStore from "../store/index"
import {LoginButton}  from "./LoginButton"
 
const Header = () => {
  const { identity } = useInternetIdentity();
  // const { principal, setPrincipal } = UserStore();
  const { colorMode, toggleColorMode } = useColorMode()
  // useEffect(() => {
  //   if (!identity) {
  //     setPrincipal("");
  //   }else{
  //     setPrincipal(identity.getPrincipal().toString());
  //   }
      
  // }, [identity]);

  // Get the principal from the backend when an identity is available
  // useEffect(() => {
  //   if (identity  && !principal) {
  //     backend.whoami().then((p) => setPrincipal(p));
  //   }
  // }, [backend, identity, principal]);
  function ColorModeIcon() {

    const SwitchIcon = colorMode === 'light' ? BsMoonFill : BsSunFill
    return (
      <SwitchIcon />
    )
  }
  return (
    <Flex
      as="nav"
      align="center"
      justify="space-between"
      wrap="wrap"
      pt={6}
      pb={3}
      borderBottomWidth={1}
    >
      <Flex width='10%'>
        <Image
          height='50px' src="./logo1.png"></Image >
      </Flex>
      <Flex align="center" ml="10%" >
        <HStack gap='5' display={{ base: 'none', md: 'flex' }}>
          <ChakraLink as={ReactRouterLink} _hover={{ textDecoration: "none" }} to='/'>
            <Heading as="h1" size="lg" letterSpacing={"tighter"}>
              OSBTC
            </Heading>
          </ChakraLink>
          {/* <Spacer />
          <ChakraLink as={ReactRouterLink} _hover={{ textDecoration: "none" }} to='/wallet'>
            <Heading as="h1" size="lg" letterSpacing={"tighter"}>
            Wallets
            </Heading>
          </ChakraLink> */}
          <Spacer />
          <ChakraLink as={ReactRouterLink} _hover={{ textDecoration: "none" }} to='/wallet'>
            <Heading as="h1" size="lg" letterSpacing={"tighter"}>
              Portfolio
            </Heading>
          </ChakraLink>
          <Spacer />
          <ChakraLink as={ReactRouterLink} _hover={{ textDecoration: "none" }} to='/wallet'>
            <Heading as="h1" size="lg" letterSpacing={"tighter"}>
              Dashboard
            </Heading>
          </ChakraLink>
          <Spacer />
          <ChakraLink as={ReactRouterLink} _hover={{ textDecoration: "none" }} to='/wallet'>
            <Heading as="h1" size="lg" letterSpacing={"tighter"}>
              About
            </Heading>
          </ChakraLink>
        </HStack>
      </Flex>
      <Spacer />
      <Flex>
        <HStack gap='5' display={{ base: 'none', md: 'flex' }}>
          <ChakraLink
            isExternal
            aria-label='Go to Chakra UI GitHub page'
            href="https://github.com/bitcoinOS/bitcoinOS"
          >
            <Icon
              as={BsGithub}
              display='block'
              transition='color 0.2s'
              fontSize='md'
              _hover={{ color: 'gray.600' }}
            />
          </ChakraLink>
          <ChakraLink
            isExternal
            aria-label='Go to Chakra UI Discord page'
            href='/discord'
          >
            <Icon
              as={BsTwitterX}
              display='block'
              transition='color 0.2s'
              fontSize='md'
              _hover={{ color: 'gray.600' }}
            />
          </ChakraLink>

        </HStack>
        <HStack gap='5'>
          <IconButton
            size='md'
            aria-label='toggle color mode'
            variant='ghost'
            color='current'
            ml={{ base: '0', md: '3' }}
            onClick={toggleColorMode}
          >
            <ColorModeIcon />
          </IconButton>

        </HStack>


      </Flex>
      <Box
        // mt={{ base: 4, md: 0 }}
        mr={5}
        ml={5}
      >
         <LoginButton ></LoginButton>
      </Box>
    </Flex>
  );
};

export default Header;
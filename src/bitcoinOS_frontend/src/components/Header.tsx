
import { useEffect, useMemo, useState } from "react";

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
  Spacer,
  Menu,
  MenuButton,
  MenuList,
  MenuItem,
  useToast
} from "@chakra-ui/react";
import { Link as ChakraLink, LinkProps } from '@chakra-ui/react'
import { Link as ReactRouterLink } from 'react-router-dom'
import {
  BsChevronDown,
  BsCopy,
  BsGithub,
  BsMoonFill,
  BsPerson,
  BsPower,
  BsSunFill,
  BsTwitterX,
  BsBoxArrowUpRight,
  BsPersonSquare
} from 'react-icons/bs'
import { useNavigate } from 'react-router-dom';
import { useInternetIdentity } from "ic-use-internet-identity";
import { useSiwbIdentity } from 'ic-use-siwb-identity';
// import UserStore from "../store/index"
import { LoginButton } from "./LoginButton"
import { ConectedWallectStorageKey, ConnectWalletType, useConnectStore } from "../store/useConnectStore";
import ConnectModal from "./ConnectModal/ConnectModal";
import WalletList from "./ConnectModal/WalletList";
import useWalletConnector from "../hooks/useWalletConnector";
import { useStakeListStore } from "../store";

import { NetworkButton } from './NetworkButton'
import { getLastLoggedInWallet } from "../utils/utils";

const Header = () => {
  const navigate = useNavigate();

  const { identity } = useInternetIdentity();
  const { prepareLogin, clear: siwb_clear, identity: siwb_identity, isPrepareLoginIdle, identityAddress, prepareLoginError, loginError, login: siwb_login, setWalletProvider, getAddress, connectedBtcAddress } =
    useSiwbIdentity();
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

  const toast = useToast()
  const [currentAccount, setCurrentAccount] = useConnectStore((state) => [state.currentAccount, state.setCurrentAccount])
  const {
    connect,
    disconnect,
    installedWallets,
  } = useWalletConnector()

  const { poolDetail, setPoolDetail } = useStakeListStore();

  // for third party auto login
  useEffect(() => {
    const connectedWallet = getLastLoggedInWallet()
    if (connectedWallet && connectedWallet !== ConnectWalletType.INTERNET_IDENTITY) {
      connect(connectedWallet, true)
    }
  }, [installedWallets])

  useEffect(() => {
    const connectedWallet = getLastLoggedInWallet()
    if (connectedWallet === ConnectWalletType.INTERNET_IDENTITY && identity) {
      setCurrentAccount({
        address: identity.getPrincipal().toString(),
        type: ConnectWalletType.INTERNET_IDENTITY,
        balance: 0
      })
    }
  }, [identity])

  // useEffect(() => {
  //   const connectedWallet = getLastLoggedInWallet()
  //   if (connectedWallet === ConnectWalletType.UNISAT && siwb_identity) {
  //     setCurrentAccount({
  //       address: siwb_identity.getPrincipal().toString(),
  //       type: ConnectWalletType.UNISAT,
  //       balance: 0
  //     })
  //   }
  //   if (connectedWallet === ConnectWalletType.WIZZ && siwb_identity) {
  //     setCurrentAccount({
  //       address: siwb_identity.getPrincipal().toString(),
  //       type: ConnectWalletType.WIZZ,
  //       balance: 0
  //     })
  //   }
  // }, [siwb_identity])

  useEffect(() => {
    if (currentAccount?.address) {
      onCloseWalletList()
      // todo: detect if the user has any wallet, if so, close the modal
      // else open the modal for creating a wallet
    }
  }, [currentAccount])

  const handleCopy = () => {
    // copy address
    if (!currentAccount) return
    if (currentAccount.type === 'INTERNET_IDENTITY') {
      navigator.clipboard.writeText(currentAccount.address).then(() => {
        toast({
          title: "Address copied",
          status: "success",
          duration: 2000,
          isClosable: true,
        })
      })
    } else {
      navigator.clipboard.writeText(siwb_identity?.getPrincipal().toText()).then(() => {
        toast({
          title: "Account copied",
          status: "success",
          duration: 2000,
          isClosable: true,
        })
      })
    }
  }

  const handleToPortfolio = () => {
    toast({
      title: "Coming soon",
      status: "warning",
      duration: 2000,
      isClosable: true,
    })
  }

  const buttonText = (address = '') => {
    if (address) {
      return address.substring(0, 5) + "..." + address.substring(address.length - 3, address.length);
    } else {
      return "Login / Register";
    }
  }

  const { isOpen: isOpneWalletList, onOpen: onOpenWalletList, onClose: onCloseWalletList } = useDisclosure()

  function ColorModeIcon() {

    const SwitchIcon = colorMode === 'light' ? BsMoonFill : BsSunFill
    return (
      <SwitchIcon />
    )
  }

  const copyText = () => {
    if (currentAccount.type === 'INTERNET_IDENTITY') {
      return "Copy ii Account"
    } else {
      return "Copy Account"
    }
  }
  return (
    <Flex
      as="nav"
      align="center"
      justify="space-between"
      wrap="wrap"
      pt={3}
      pb={3}
      pl='3'
      bgColor='#FFFFFF'
    //borderBottomWidth={1}
    >
      <ConnectModal isOpen={isOpneWalletList} onClose={onCloseWalletList}>
        <WalletList />
      </ConnectModal>
      <Flex width='10%' justifyContent='center'>
        <ChakraLink as={ReactRouterLink} _hover={{ textDecoration: "none" }} to='/stake'>
          <Image
            height={{ md: '35px', lg: '41px', xl: '43px', '2xl': '45px', '3xl': '45px' }} src="./logo.png"></Image >
        </ChakraLink>
      </Flex>
      <Flex align="center" ml="10%" >
        <HStack gap='5' display={{ base: 'none', md: 'flex' }}>
          <ChakraLink as={ReactRouterLink} _hover={{ textDecoration: "none" }} to='/'>
            <Heading
              as="h1"
              //size="lg"
              //fontSize={{ md: '18px', lg: '20px', xl: '23px', '2xl': '26px', '3xl': '32px' }}
              fontSize='14px'
              color='gray.700'
              fontWeight='700'
              fontFamily='Ubuntu'
              css={{
                background: "linear-gradient(to right, #FF8800, #AA00FF, #3600FF, #33CCFF)",
                WebkitBackgroundClip: "text",
                WebkitTextFillColor: "transparent",
                backgroundClip: "text",
                color: "transparent"
              }}
            //letterSpacing={"tighter"}
            >
              BTC Marathon
            </Heading>
          </ChakraLink>
          <Spacer />
          <ChakraLink as={ReactRouterLink} _hover={{ textDecoration: "none" }} to='/leaderboard'>
            <Heading
              as="h1"
              //size="lg"
              //fontSize={{ md: '18px', lg: '20px', xl: '23px', '2xl': '26px', '3xl': '32px' }}
              fontSize='14px'
              color='gray.700'
              fontWeight='400'
              fontFamily='Ubuntu'
            //letterSpacing={"tighter"}
            >
              Leaderboard
            </Heading>
          </ChakraLink>
          <Spacer />
          <ChakraLink as={ReactRouterLink} _hover={{ textDecoration: "none" }} to='/reward'>
            <Flex direction='column' justifyContent='center'>
              <Text
                //size="lg"
                //letterSpacing={"tighter"}
                textAlign='center'
                fontFamily='Ubuntu'
              >
                <Text
                  fontSize={{ md: '14px', lg: '14px', xl: '14px', '2xl': '14px', '3xl': '14px' }}
                  color='gray.700'
                  fontWeight='400'
                >
                  Reward
                </Text>
              </Text>

            </Flex>
          </ChakraLink>
          <Spacer />
          <ChakraLink
            as="a"
            href="https://www.thefaucet.org/"
            target="_blank"
            rel="noopener noreferrer"
            _hover={{ textDecoration: "none" }}
          >
            <Flex direction='row' alignItems='center' justifyContent='center'>
              <Text
                //size="lg"
                //letterSpacing={"tighter"}
                textAlign='center'
                mr='2'
              >
                <Text
                  fontSize={{ md: '14px', lg: '14px', xl: '14px', '2xl': '14px', '3xl': '14px' }}
                  color='gray.700'
                  fontWeight='400'
                >
                  Faucet
                </Text>
              </Text>
              <BsBoxArrowUpRight />
            </Flex>

          </ChakraLink>
        </HStack>
      </Flex>
      <Spacer />
      {/*
      <Flex>
        <HStack gap='5' display={{ base: 'none', md: 'flex' }}>
          <ChakraLink
            isExternal
            aria-label='Go to bitcoinOS GitHub page'
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
            aria-label='Go to bitcoinOS X page'
            href='https://twitter.com/BitcoinOS_labs'
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
      */}
      <Box
        // mt={{ base: 4, md: 0 }}
        mr={5}
        ml={5}
      >
        <NetworkButton />
      </Box>
      <Box
        // mt={{ base: 4, md: 0 }}
        mr={5}
        zIndex='2'
      >
        {/* <LoginButton ></LoginButton> */}
        {
          !currentAccount
            ? <Button
              height="2.5rem"
              color="white"
              bgColor="orange.400"
              _hover={{ bg: "orange.200", borderColor: "orange.400" }}
              onClick={onOpenWalletList}
            >
              {buttonText(currentAccount?.address)}
            </Button>
            : <Menu>
              <MenuButton as={Button} rightIcon={<BsChevronDown />}>
                {currentAccount.type === 'INTERNET_IDENTITY'
                  ? buttonText(currentAccount.address)
                  : buttonText(siwb_identity?.getPrincipal().toText())}
              </MenuButton>
              <MenuList p={2} display={'flex'} flexDirection={'column'} gap={2}>
                {/*
                  <MenuItem onClick={handleToPortfolio} as={Button} justifyContent={'flex-start'} colorScheme='gray' variant='outline' leftIcon={<UserIcon />}>My Portfolio</MenuItem>
                */}
                <MenuItem onClick={() => { navigate('portfolio') }} as={Button} justifyContent={'flex-start'} colorScheme='gray' variant='outline' leftIcon={<BsPersonSquare />}>My Portfolio</MenuItem>
                <MenuItem onClick={handleCopy} as={Button} justifyContent={'flex-start'} colorScheme='gray' variant='outline' leftIcon={<CopyIcon />}>{copyText()}</MenuItem>
                <MenuItem onClick={disconnect} as={Button} justifyContent={'flex-start'} colorScheme='gray' variant='outline' leftIcon={<DisconnetIcon />}>Disconnet</MenuItem>
              </MenuList>
            </Menu>
        }
      </Box>
      {/* <Box
        // mt={{ base: 4, md: 0 }}
        mr={5}
        ml={5}
      >
        <LoginButton ></LoginButton>
      </Box> */}
    </Flex >
  );
};

const UserIcon = () => (
  <svg width="16" height="16" viewBox="0 0 16 16" fill="none" xmlns="http://www.w3.org/2000/svg">
    <circle cx="8" cy="5" r="4" stroke="black" strokeWidth="1.5" strokeLinecap="round" strokeLinejoin="round" />
    <path d="M14 15C14 12.349 12.2 9 8 9C3.8 9 2 12.349 2 15" stroke="black" strokeWidth="1.5" strokeLinecap="round" strokeLinejoin="round" />
  </svg>
)

const CopyIcon = () => (
  <svg width="16" height="16" viewBox="0 0 16 16" fill="none" xmlns="http://www.w3.org/2000/svg">
    <rect x="2" y="4" width="10" height="10" rx="2" stroke="black" strokeWidth="1.5" strokeLinecap="round" strokeLinejoin="round" />
    <path d="M6 1H12C13.6569 1 15 2.34315 15 4V10" stroke="black" strokeWidth="1.5" strokeLinecap="round" strokeLinejoin="round" />
  </svg>

)

const DisconnetIcon = () => (
  <svg width="16" height="16" viewBox="0 0 16 16" fill="none" xmlns="http://www.w3.org/2000/svg">
    <path d="M11.9687 3.5C13.2144 4.59942 14 6.20796 14 8C14 11.3137 11.3137 14 8 14C4.68629 14 2 11.3137 2 8C2 6.20796 2.78563 4.59942 4.03126 3.5M8 2V7" stroke="black" strokeWidth="1.5" strokeLinecap="round" strokeLinejoin="round" />
  </svg>
)

export default Header;

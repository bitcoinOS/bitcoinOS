import { useEffect, useState, useCallback } from "react"

import { useInterval } from '@chakra-ui/react';

import { ConnectAccountType, ConnectWalletType, useConnectStore } from "../store/useConnectStore"
import { useInternetIdentity } from "ic-use-internet-identity"
import { okxWalletAdapter, unisatWalletAdapter, walletAdapterMap } from "../utils/walletAdapter"

import { WalletStore } from "../store/useWalletStore"
import { usePoolrecordStore } from "../store/useStakePool"
import { userStore } from "../store/useMarathonStore"

import useOsActor from "../utils/osActor"

import { useOsBackend } from "../ic/OsActors"

import { useSiwbIdentity } from 'ic-use-siwb-identity';
import { getLastLoggedInWallet } from "../utils/utils";
import useGetMarathon from "../utils/marathonActor"

import { checkIdentityExpiration } from "../utils/utils"

export interface SelftConnector {
  installedWallets: ConnectWalletType[],
  connect: (type: ConnectWalletType, autologin: boolean) => Promise<void>
  disconnect: () => Promise<void>
  isInstalled: (type: ConnectWalletType) => boolean
  connectingWallet: ConnectWalletType | null
}

export default function useWalletConnector(): SelftConnector {
  const { actor: osBackend } = useOsBackend();

  const { identity, isLoggingIn, login, clear, isLoginSuccess } = useInternetIdentity();
  const { prepareLogin, clear: siwb_clear, identity: siwb_identity, isPrepareLoginIdle, identityAddress, prepareLoginError, loginError, login: siwb_login, setWalletProvider, getAddress, connectedBtcAddress } =
    useSiwbIdentity();

  const [currentAccount, setCurrentAccount] = useConnectStore((state) => [state.currentAccount, state.setCurrentAccount])
  const [connectingWallet, setConnectingWallet] = useState<ConnectWalletType | null>(null)
  const { userInfo } = userStore()

  const [isOsInited, setIsOsInited] = useState<boolean>(false)

  const [installedWallets, setInstalledWallets] = useState<ConnectWalletType[]>([])

  const [manually, setManually] = useState<boolean>(false);

  //ii walet info
  const { setWallet, setWalletList, setWalletSelect, setCurrentWallet, setBalance, setTotalBalance } = WalletStore()
  const { setStakeRecords } = usePoolrecordStore()
  const { os_login, os_logout } = useOsActor()
  const { get_info } = useGetMarathon()

  const fn = (address: string, type: ConnectWalletType) => {
    // nauxscript console

    if (type === ConnectWalletType.INTERNET_IDENTITY) return;
    adapter[type].getBalance(address).then((balance) => {
      setCurrentAccount({
        address: address,
        type: type,
        balance: balance === 'err' ? 0 : balance.total
      })
      setConnectingWallet(null)
      console.log('balance updated', balance);
    })
  }

  const initInstalledWallets = async () => {
    let list: ConnectWalletType[] = []
    // check if unisat is installed
    if (typeof globalThis.unisat !== 'undefined') {
      list.push(ConnectWalletType.UNISAT)
    }

    if (typeof globalThis.okxwallet !== 'undefined') {
      list.push(ConnectWalletType.OKX)
    }

    if (typeof globalThis.wizz !== 'undefined') {
      list.push(ConnectWalletType.WIZZ)
    }

    list.push(ConnectWalletType.INTERNET_IDENTITY)
    setInstalledWallets(list)
  }

  useEffect(() => {
    initInstalledWallets()
  }, [])

  useEffect(() => {
    if (osBackend) {
      setIsOsInited(true)
    }

  }, [osBackend])

  useEffect(() => {
    if (currentAccount && currentAccount.address) {
      setConnectingWallet(null)
    }
  }, [currentAccount])

  useEffect(() => {
    if (identity && osBackend && currentAccount &&
      typeof currentAccount === 'object' &&
      'type' in currentAccount &&
      currentAccount.type === 'INTERNET_IDENTITY') {
      if (!userInfo || (Array.isArray(userInfo) && userInfo.length === 0)) {
        get_info(osBackend)
      }
    }
    if (siwb_identity && osBackend && currentAccount &&
      typeof currentAccount === 'object' &&
      'type' in currentAccount &&
      currentAccount.type === 'UNISAT') {
      console.log(userInfo)
      if (!userInfo || (Array.isArray(userInfo) && userInfo.length === 0)) {
        get_info(osBackend)
      }
    }
    if (siwb_identity && osBackend && currentAccount &&
      typeof currentAccount === 'object' &&
      'type' in currentAccount &&
      currentAccount.type === 'WIZZ') {
      console.log(userInfo)
      if (!userInfo || (Array.isArray(userInfo) && userInfo.length === 0)) {
        get_info(osBackend)
      }
    }
  }, [currentAccount])

  useEffect(() => {
    const address = getAddress();
    if (currentAccount && currentAccount.type && !currentAccount.address) {
      const { type } = currentAccount
      if (type && identityAddress && siwb_identity) {
        fn(identityAddress, type)
      }
    }
  }, [currentAccount && currentAccount.address])

  const checkAndUpdateIdentity = useCallback(() => {
    if (siwb_identity && currentAccount && (currentAccount.type === 'UNISAT' || currentAccount.type === 'WIZZ')) {
      if (!checkIdentityExpiration(siwb_identity)) {
        siwb_clear();
        os_logout();
        setCurrentAccount(undefined);
      }
    }
    if (identity && currentAccount && currentAccount.type === 'INTERNET_IDENTITY') {
      if (!checkIdentityExpiration(identity)) {
        clear();
        os_logout();
        setCurrentAccount(undefined);
      }
    }
  }, [identity, siwb_identity, currentAccount]);

  // Initial inspection
  useEffect(() => {
    checkAndUpdateIdentity();
  }, [checkAndUpdateIdentity]);

  // Periodic checks (per minute)
  useInterval(checkAndUpdateIdentity, 60000);


  useEffect(() => {
    if (!isPrepareLoginIdle) return;
    const address = getAddress();
    if (address) {
      prepareLogin();
      if (connectedBtcAddress && !siwb_identity && manually) {
        (async () => {
          //setLoading(true);
          const res = await siwb_login().catch(() => {
            setConnectingWallet(null)
            setCurrentAccount(undefined)
            siwb_clear()
            os_logout()
            return Promise.reject()
          });
          const { type } = currentAccount
          if (type !== ConnectWalletType.INTERNET_IDENTITY) {
            adapter[type].getBalance(address).then((balance) => {
              setCurrentAccount({
                address: address,
                type: type,
                balance: balance === 'err' ? 0 : balance.total
              })
            })
            console.log(currentAccount)
          }
          //setLoading(false);
          if (res) {
            setManually(false);
          }
        })();
      }
    }
  }, [prepareLogin, isPrepareLoginIdle, getAddress, siwb_login, connectedBtcAddress, siwb_identity, manually]);

  useEffect(() => { }, [prepareLoginError]);
  /**
   * Show an error toast if the login call fails.
   */
  useEffect(() => { }, [loginError]);

  const adapter = {
    [ConnectWalletType.UNISAT]: {
      connect: async (autologin) => {
        //const [address] = 'test'
        //const [address] = await globalThis.unisat.requestAccounts()
        setManually(true);
        if (!autologin) {
          await setWalletProvider('unisat')
        }
        return {
          type: ConnectWalletType.UNISAT,
          address: ''
        }
      },
      disconnect: async () => {
        await siwb_clear()
        os_logout()
        // await globalThis.unisat.disconnect()
      },
      sendBitcoin: async (toAddress, satoshis, options) => {
        return globalThis.unisat.sendBitcoin(toAddress, satoshis, options)
      },
      getBalance: async () => {
        console.log('getbalance')
        const maxRetries = 1;
        for (let i = 0; i < maxRetries; i++) {
          try {
            const balance = await globalThis.unisat.getBalance();
            if (balance && balance.total) {
              return balance;
            } else {
              return 'err'
            }
          } catch (error) {
            console.error(`Attempt ${i + 1} failed:`, error);
            if (i === maxRetries - 1) {
              const result = 'err'
              return result
              throw error;
            }
            await new Promise(resolve => setTimeout(resolve, 1000));

          }
        }
        return 'err';
      }
    },
    [ConnectWalletType.WIZZ]: {
      connect: async (autologin) => {
        //const [address] = 'test'
        //const [address] = await globalThis.unisat.requestAccounts()
        setManually(true);
        if (!autologin) {
          await setWalletProvider('wizz')
        }
        return {
          type: ConnectWalletType.WIZZ,
          address: ''
        }
      },
      disconnect: async () => {
        await siwb_clear()
        os_logout()
        // await globalThis.unisat.disconnect()
      },
      sendBitcoin: async (toAddress, satoshis, options) => {
        return globalThis.wizz.sendBitcoin(toAddress, satoshis, options)
      },
      getBalance: async () => {
        console.log('getbalance')
        const maxRetries = 1;
        for (let i = 0; i < maxRetries; i++) {
          try {
            const balance = await globalThis.wizz.getBalance();
            console.log(balance);
            if (balance && balance.total) {
              return balance;
            } else {
              return 'err'
            }
          } catch (error) {
            console.error(`Attempt ${i + 1} failed:`, error);
            if (i === maxRetries - 1) {
              const result = 'err'
              return result
              throw error;
            }
            await new Promise(resolve => setTimeout(resolve, 1000));
          }
        }
        return 'err';
      }
    },
    //[ConnectWalletType.UNISAT]: unisatWalletAdapter,
    [ConnectWalletType.OKX]: okxWalletAdapter,
    [ConnectWalletType.INTERNET_IDENTITY]: {
      connect: async () => {
        await login().catch((error) => {
          if (error.message === "User is already authenticated") {
            return Promise.resolve()
          } else {
            console.error(error);
          }
        })
        return {
          type: ConnectWalletType.INTERNET_IDENTITY,
          address: ''
        }
      },
      disconnect: async () => {
        await clear()
        os_logout()
      }
    }
  }

  const isInstalled = (type: ConnectWalletType) => installedWallets.includes(type)

  const connect = async (type: ConnectWalletType, autologin = false) => {
    if (connectingWallet || !installedWallets.includes(type)) return
    // nauxscript console
    console.log('%c nausxcript connectiong', 'color: red')
    setConnectingWallet(type)
    console.log('connecting to wallet');
    const account = await adapter[type].connect(autologin)
    setCurrentAccount(account)
    if (account.address) {
      // setConnectingWallet(null)
      updateBalance(account)
    }
    console.log('connected!');
  }

  const disconnect = async () => {
    console.log(connectingWallet)
    if (connectingWallet || !isInstalled(currentAccount.type)) return
    console.log('disconnecting to unisat');
    await adapter[currentAccount.type].disconnect()
    setCurrentAccount(undefined)
    console.log('disconnected!');
  }

  const updateBalance = (account: ConnectAccountType) => {
    const { type, address } = account
    if (type !== ConnectWalletType.INTERNET_IDENTITY) {
      adapter[type].getBalance(address).then((balance) => {
        setCurrentAccount({
          ...account,
          balance: balance.total
        })
        console.log('balance updated', balance);
      })
    }
  }

  return {
    installedWallets,
    connect,
    disconnect,
    connectingWallet,
    isInstalled
  }
}

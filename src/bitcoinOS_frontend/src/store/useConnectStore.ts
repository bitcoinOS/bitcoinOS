import { create } from 'zustand'

export const ConectedWallectStorageKey = 'CONNECTED_WALLET'

export enum ConnectWalletType {
  OKX = 'OKX',
  UNISAT = 'UNISAT',
  WIZZ = 'WIZZ',
  INTERNET_IDENTITY = 'INTERNET_IDENTITY'
}

export interface ConnectAccountType {
  address: string
  type: ConnectWalletType,
  balance?: number
}

interface IState {
  currentAccount: ConnectAccountType | undefined
  setCurrentAccount: (currentAccount: ConnectAccountType | undefined) => void
}

export const useConnectStore = create<IState>((set) => ({
  currentAccount: undefined,
  setCurrentAccount: (currentAccount) => {
    localStorage.setItem(ConectedWallectStorageKey, currentAccount?.type || '')
    set({ currentAccount })
  }
}))
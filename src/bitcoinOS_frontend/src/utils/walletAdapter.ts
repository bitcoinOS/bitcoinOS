import { ConnectWalletType } from "../store/useConnectStore";

export interface WalletAdapter {
  connect: () => Promise<{
    type: ConnectWalletType;
    address: string;
  }>;
  disconnect: () => Promise<void>;
  sendBitcoin: (toAddress: string, satoshis: number, options?: { feeRate: number }) => Promise<string>;
  getBalance?: (address: string) => Promise<{
    confirmed: number;
    unconfirmed: number;
    total: number;
  }>;
}

export const okxWalletAdapter: WalletAdapter = {
  connect: async () => {
    const [address] = await globalThis.okxwallet.bitcoin.requestAccounts()
    return {
      type: ConnectWalletType.OKX,
      address
    }
  },
  disconnect: async () => {
    await globalThis.okxwallet.bitcoin.disconnect()
  },
  sendBitcoin: async (toAddress, satoshis, options) => {
    return globalThis.okxwallet.bitcoin.sendBitcoin(toAddress, satoshis, options)
  },
  getBalance: async () => {
    return globalThis.okxwallet.bitcoin.getBalance()
  }
}

export const unisatWalletAdapter: WalletAdapter = {
  connect: async () => {
    const [address] = await globalThis.unisat.requestAccounts()
    return {
      type: ConnectWalletType.UNISAT,
      address
    }
  },
  disconnect: async () => {
    // await globalThis.unisat.disconnect()
  },
  sendBitcoin: async (toAddress, satoshis, options) => {
    return globalThis.unisat.sendBitcoin(toAddress, satoshis, options)
  },
  getBalance: async () => {
    return globalThis.unisat.getBalance()
  }
}

export const wizzWalletAdapter: WalletAdapter = {
  connect: async () => {
    const [address] = await globalThis.wizz.requestAccounts()
    return {
      type: ConnectWalletType.WIZZ,
      address
    }
  },
  disconnect: async () => {
    // await globalThis.unisat.disconnect()
  },
  sendBitcoin: async (toAddress, satoshis, options) => {
    return globalThis.wizz.sendBitcoin(toAddress, satoshis, options)
  },
  getBalance: async () => {
    return globalThis.wizz.getBalance()
  }
}

export const walletAdapterMap = {
  [ConnectWalletType.OKX]: okxWalletAdapter,
  [ConnectWalletType.UNISAT]: unisatWalletAdapter,
  [ConnectWalletType.WIZZ]: wizzWalletAdapter,
}
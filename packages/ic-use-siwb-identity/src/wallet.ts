/* eslint-disable @typescript-eslint/no-unused-vars */

import type { EventEmitter } from 'stream';

/* eslint-disable @typescript-eslint/no-explicit-any */
export type SignMessageType = 'ecdsa' | 'bip322-simple';

export type WalletProviderKey =
  | 'wizz'
  | 'unisat'
  | 'atom'
  | 'xverse'
  | 'okxwallet.bitcoinTestnet'
  | 'okxwallet.bitcoin'
  | 'okxwallet.bitcoinSignet'
  | 'BitcoinProvider';

export type NetworkType = 'testnet' | 'testnet4' | 'livenet' | 'mainnet' | 'signet' | 'bitcoin';

export interface IBitcoinProvider extends EventEmitter {
  request(method: string, params: any): Promise<any>;
}

export type XverseAddressPurpose = 'ordinals' | 'payment';

export class BitcoinProviderMaker {
  private defaultAddress: string | undefined;
  private defaultPublickey: string | undefined;
  constructor(private _inner: IBitcoinProvider) {}
  static createProvider(providerKey: 'BitcoinProvider'): BitcoinProviderMaker {
    const provider = getPropByKey(window as any, providerKey);
    if (provider) {
      return new BitcoinProviderMaker(provider as IBitcoinProvider);
    }
    throw new Error(`Provider ${providerKey} not found`);
  }

  async signMessage(message: string, type?: string | SignMessageType): Promise<string> {
    if (this.defaultAddress === undefined) {
      throw new Error(`Connect Wallet first`);
    }

    const [addressType, _] = getAddressType(this.defaultAddress);

    if (type) {
      if (type.toLowerCase() === 'ecdsa') {
        if (addressType === AddressType.P2TR || addressType === AddressType.P2WPKH || addressType === AddressType.P2SH_P2WPKH) {
          throw new Error(`Wallet Type: ${addressType} not supoorted for sign message type: ${type}`);
        }
      } else {
        if (addressType === AddressType.P2PKH || addressType === AddressType.P2SH_P2WPKH) {
          throw new Error(`Wallet Type: ${addressType} not supoorted for sign message type: ${type}`);
        }
      }
    }

    // we only support ecdsa sig for login currently, so we use paymentAddress to request ECDSA signature instead of p2tr
    const { result: res } = await (this._inner.request('signMessage', { address: this.defaultAddress, message }) as Promise<{
      result: {
        address: string;
        messageHash: string;
        signature: string;
      };
    }>);
    return res.signature;
  }

  async requestAccounts(): Promise<string[]> {
    return this.getAccounts();
  }

  async getAccounts(): Promise<string[]> {
    const { result: addresses } = await (this._inner.request('getAccounts', { purposes: ['ordinals', 'payment'] }) as Promise<{
      result: { address: string; addressType: string; publicKey: string; purpose: XverseAddressPurpose }[];
    }>);
    this.defaultAddress = addresses.length > 0 ? addresses[0]!.address : undefined;
    this.defaultPublickey = addresses.length > 0 ? addresses[0]!.publicKey : undefined;
    return addresses.map(a => a.address);
  }

  async getPublicKey(): Promise<string> {
    if (this.defaultPublickey === undefined) {
      throw new Error(`Connect Wallet first`);
    }
    return this.defaultPublickey;
  }

  async getNetwork(): Promise<NetworkType> {
    if (this.defaultAddress === undefined) {
      throw new Error(`Connect Wallet first`);
    }
    return getAddressType(this.defaultAddress!)[1];
  }
  // on(event: 'accountsChanged' | 'networkChanged', listener: (data: any) => void) {
  //   this._inner.on(event, listener);
  // }
  // removeListener(event: 'accountsChanged' | 'networkChanged', listener: (data: any) => void) {
  //   this._inner.removeListener(event, listener);
  // }
}

export interface IWalletProvider {
  fetchAndValidateFile(url: string, filePath: string, expectSHA: string): Promise<string>;

  getProxy(): string | undefined;

  // Connect the current account.
  requestAccounts(): Promise<string[]>;

  getAccounts(): Promise<string[]>;

  getNetwork(): Promise<NetworkType>;

  // Get an address type, return null if the address is invalid
  getAddressType(address: string): Promise<string | null>;

  // Get current account PublicKey
  getPublicKey(): Promise<string>;

  // Sign message
  signMessage(message: string, type?: string | SignMessageType): Promise<string>;

  // // Sign Psbt(hex)
  // signPsbt(psbtHex: string, options?: SignOptions): Promise<string>;

  // // Sign Psbts(hexs)
  // signPsbts(psbtHexs: string[], options?: SignOptions): Promise<string[]>;

  getAppVersion(): Promise<string>;

  getSupportedMethods(): Promise<string[]>;

  pushTx({ rawtx }: { rawtx: string }): Promise<string>;

  pushPsbt(psbt: string): Promise<string>;

  on(event: 'accountsChanged' | 'networkChanged', listener: (data: any) => void): this;
  removeListener(event: 'accountsChanged' | 'networkChanged', listener: (data: any) => void): this;
}

export interface NetworkItem {
  type: string;
  network: NetworkType;
}

export const NETWORKS: { [key: string]: NetworkItem } = {
  mainnet: {
    type: 'livenet',
    network: 'bitcoin',
  },
  testnet: {
    type: 'testnet',
    network: 'testnet',
  },
  testnet4: {
    type: 'testnet4',
    network: 'testnet',
  },
  signet: {
    type: 'signet',
    network: 'testnet',
  },
};

export function getPropByKey(obj: any, key: string) {
  const keys = key.split('.');
  let result = obj;
  for (const key1 of keys) {
    if (result) {
      result = result[key1];
    }
  }
  return result;
}

export const getWalletProvider = (key: WalletProviderKey) => {
  if (key == 'BitcoinProvider' || key == 'xverse') {
    return BitcoinProviderMaker.createProvider('BitcoinProvider');
  } else {
    const provider = getPropByKey(window as any, key);
    console.log({ provider, key });
    if (provider) return provider as IWalletProvider;
  }
};

export function isPageHidden() {
  const doc = document as any;
  return doc.hidden || doc.msHidden || doc.webkitHidden || doc.mozHidden;
}

export async function getRegisterExtension(providerKey?: WalletProviderKey) {
  const provider = providerKey ? getWalletProvider(providerKey) : undefined;
  let address: string | undefined = undefined;
  let network: NetworkItem | undefined = undefined;
  const wp = provider;
  const accountChange = (accounts: string[]) => {
    if (isPageHidden()) {
      return;
    }
    address = accounts[0];
  };
  const networkChange = (_n: string) => {
    (async () => {
      if (isPageHidden()) {
        return;
      }
      if (_n === 'mainnet' || _n === 'livenet' || !_n) {
        network = NETWORKS.mainnet!;
      } else {
        network = NETWORKS[_n]!;
      }
    })();
  };

  const getNetwork = async () => {
    const network = await wp?.getNetwork();
    if (network) {
      networkChange(network);
    }
  };

  const requestAccounts = async () => {
    const accounts = await wp?.requestAccounts();
    if (accounts && accounts.length > 0) {
      address = accounts[0];
    }
  };
  if (wp) {
    if ((wp as IWalletProvider).on !== undefined) {
      (wp as IWalletProvider).on('accountsChanged', accountChange);
      (wp as IWalletProvider).on('networkChanged', networkChange);
    }

    await requestAccounts();
    await getNetwork();
    if ((wp as IWalletProvider).removeListener !== undefined) {
      (wp as IWalletProvider).removeListener('accountsChanged', accountChange);
      (wp as IWalletProvider).removeListener('networkChanged', networkChange);
    }
  }

  return { provider, providerKey, address, network };
}

export enum AddressType {
  P2PKH,
  P2WPKH,
  P2TR,
  P2SH_P2WPKH,
}

export function getAddressType(address: string): [AddressType, NetworkType] {
  if (address.startsWith('bc1q')) {
    return [AddressType.P2WPKH, 'mainnet'];
  } else if (address.startsWith('bc1p')) {
    return [AddressType.P2TR, 'mainnet'];
  } else if (address.startsWith('1')) {
    return [AddressType.P2PKH, 'mainnet'];
  } else if (address.startsWith('3')) {
    return [AddressType.P2SH_P2WPKH, 'mainnet'];
  }
  // testnet
  else if (address.startsWith('tb1q')) {
    return [AddressType.P2WPKH, 'testnet'];
  } else if (address.startsWith('m') || address.startsWith('n')) {
    return [AddressType.P2PKH, 'testnet'];
  } else if (address.startsWith('2')) {
    return [AddressType.P2SH_P2WPKH, 'testnet'];
  } else if (address.startsWith('tb1p')) {
    return [AddressType.P2TR, 'testnet'];
  }
  throw new Error(`Unknown address: ${address}`);
}

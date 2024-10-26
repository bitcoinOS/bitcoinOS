import { DelegationChain, DelegationIdentity } from '@dfinity/identity';
import type { LoginStatus, PrepareLoginStatus } from './state.type';
import type { NetworkItem, WalletProviderKey } from './wallet';

export type SiwbIdentityContextType = {
  /** Is set to `true` on mount until a stored identity is loaded from local storage or
   * none is found. */
  isInitializing: boolean;

  /** Load a SIWb message from the provider canister, to be used for login. Calling prepareLogin
   * is optional, as it will be called automatically on login if not called manually. */
  prepareLogin: () => void;

  /** Reflects the current status of the prepareLogin process. */
  prepareLoginStatus: PrepareLoginStatus;

  /** `prepareLoginStatus === "loading"` */
  isPreparingLogin: boolean;

  /** `prepareLoginStatus === "error"` */
  isPrepareLoginError: boolean;

  /** `prepareLoginStatus === "success"` */
  isPrepareLoginSuccess: boolean;

  /** `prepareLoginStatus === "idle"` */
  isPrepareLoginIdle: boolean;

  /** Error that occurred during the prepareLogin process. */
  prepareLoginError?: Error;

  /** Initiates the login process by requesting a SIWb message from the backend. */
  login: () => Promise<DelegationIdentity | undefined>;

  /** Reflects the current status of the login process. */
  loginStatus: LoginStatus;

  /** `loginStatus === "logging-in"` */
  isLoggingIn: boolean;

  /** `loginStatus === "error"` */
  isLoginError: boolean;

  /** `loginStatus === "success"` */
  isLoginSuccess: boolean;

  /** `loginStatus === "idle"` */
  isLoginIdle: boolean;

  /** Error that occurred during the login process. */
  loginError?: Error;

  /** Status of the SIWb message signing process. This is a re-export of the Wagmi
   * signMessage / status type. */
  signMessageStatus: 'error' | 'idle' | 'pending' | 'success';

  /** Error that occurred during the SIWb message signing process. This is a re-export of the
   * Wagmi signMessage / error type. */
  signMessageError: Error | null;

  /** The delegation chain is available after successfully loading the identity from local
   * storage or completing the login process. */
  delegationChain?: DelegationChain;

  /** The identity is available after successfully loading the identity from local storage
   * or completing the login process. */
  identity?: DelegationIdentity;

  /** The Bitcoin address associated with current identity. This address is not necessarily
   * the same as the address of the currently connected wallet - on wallet change, the addresses
   * will differ. */
  identityAddress?: string;

  /** Clears the identity from the state and local storage. Effectively "logs the user out". */
  clear: () => void;

  /** Network Identitfier, not adding bitcoinjs-lib network directly, simple string */

  network?: NetworkItem;

  /** We don't have things like rainbow kit right now, so we have to manually set provider key */
  setWalletProvider: (providerKey: WalletProviderKey) => Promise<void>;

  /** We don't have things like rainbow kit right now, so we have to manually return provider key */
  selectedProvider?: WalletProviderKey;

  /** We don't have things like rainbow kit right now, so we have to manually return btc address */
  connectedBtcAddress?: string;

  /** We don't have things like rainbow kit right now, so we have to manually return btc address */
  getAddress: () => string | undefined;
};

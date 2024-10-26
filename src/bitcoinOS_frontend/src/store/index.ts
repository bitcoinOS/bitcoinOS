import { create } from 'zustand';
// import { Principal } from "@dfinity/principal"

import { StakingPoolInfo } from "../ic/OsActors";

interface ExtendedStakingPoolInfo extends StakingPoolInfo {
  tvl?: any; // New tvl attribute, optional
}

interface ICurrentStatus {
  isLoading: boolean;
  setIsLoading: (isLoading: boolean) => void
}

interface NetworkInto {
  btcprice: number;
  network: string;
  setBtcprice: (btcprice: number) => void
  setNetwork: (network: string) => void
}

interface IWalletState {
  currentWallet: string;
  setCurrentWallet: (currentWallet: string) => void;
}

interface IStakepoolState {
  stakepoolCanister: string;
  setStakepoolCanister: (currentWallet: string) => void;
}

interface WalletList {
  walletList: [];
  setWalletList: (currentWallet: []) => void;
}

interface IStakeListState {
  stakeList: ExtendedStakingPoolInfo[];
  currentPool: ExtendedStakingPoolInfo;
  poolDetail: boolean;
  setStakeList: (stakeList: ExtendedStakingPoolInfo[]) => void;
  setCurrentPool: (currentPool: ExtendedStakingPoolInfo) => void;
  setPoolDetail: (poolDetail: boolean) => void;
}

interface ICurrentPoolState {
  currentPool: ExtendedStakingPoolInfo[];
  setCurrentPool: (currentPool: ExtendedStakingPoolInfo[]) => void;
}

interface AllInfo {
  tvl: number;
  tvl_24h: number;
  users: number;
  userslist: number[];
  tvllist: number[];
  setTvl: (tvl: number) => void;
  setTvl_24h: (tvl_24h: number) => void;
  setUsers: (user: number) => void;
  setUserslist: (userslist: number[]) => void;
  setTvllist: (tvllist: number[]) => void;
}

export const CurrentStatus = create<ICurrentStatus>((set) => ({
  isLoading: false,
  setIsLoading: (isLoading: boolean) => set({ isLoading })
}))

export const getNetworkInfo = create<NetworkInto>((set) => ({
  btcprice: 0,
  network: '',
  setBtcprice: (btcprice: number) => set({ btcprice }),
  setNetwork: (network: string) => set({ network })
}))

export const WalletStore = create<IWalletState>((set) => ({
  currentWallet: "",
  setCurrentWallet: (currentWallet: string) => set({ currentWallet })
}))


export const StakepoolStore = create<IStakepoolState>((set) => ({
  stakepoolCanister: '',
  setStakepoolCanister: (stakepoolCanister: string) => set({ stakepoolCanister })
}))

export const walletList = create<WalletList>((set) => ({
  walletList: [],
  setWalletList: (walletList: []) => set({ walletList })
}))

// Tvl、User Info
export const useAllInfo = create<AllInfo>((set) => ({
  tvl: 0,
  tvl_24h: 0,
  users: 0,
  userslist: [],
  tvllist: [],
  setTvl: (tvl: number) => set({ tvl }),
  setTvl_24h: (tvl_24h: number) => set({ tvl_24h }),
  setUsers: (users: number) => set({ users }),
  setUserslist: (userslist: number[]) => set({ userslist }),
  setTvllist: (tvllist: number[]) => set({ tvllist })
}))

// Pool Info
export const useStakeListStore = create<IStakeListState>((set) => ({
  stakeList: [],
  currentPool: undefined,
  poolDetail: false,
  setStakeList: (stakeList: ExtendedStakingPoolInfo[]) => set({ stakeList }),
  setCurrentPool: (currentPool: ExtendedStakingPoolInfo) => set({ currentPool }),
  setPoolDetail: (poolDetail: boolean) => set({ poolDetail }),
}));

export const useCurrentPoolStore = create<ICurrentPoolState>((set) => ({
  currentPool: [],
  setCurrentPool: (currentPool: ExtendedStakingPoolInfo[]) => set({ currentPool }),
}));
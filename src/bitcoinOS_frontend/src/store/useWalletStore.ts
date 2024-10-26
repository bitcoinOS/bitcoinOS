import { create } from 'zustand';

import { WalletInfo } from '../ic/OsActors';

interface StakedNFTItem {
    nft_id: number;
    amount: number;
    stake_at: bigint;
}


// Define the shape of the store state
interface IWalletState {
    wallet: string;
    walletList: WalletInfo[];
    walletSelect: WalletInfo | null;
    currentWallet: string;
    balance: number;
    totalBalance: number;
    // Methods to update the state
    setWallet: (wallet: string) => void;
    setWalletList: (walletList: WalletInfo[]) => void;
    setWalletSelect: (walletSelect: WalletInfo | null) => void;
    setCurrentWallet: (currentWallet: string) => void;
    setBalance: (balance: number) => void;
    setTotalBalance: (totalBalance: number) => void;
}

interface ICPWalletState {
    IcpWallet: string;
    BindIcpWallet: string;
    icpNft: number[];
    setIcpWallet: (IcpWallet: string) => void;
    setBindIcpWallet: (BindIcpWallet: string) => void;
    setIcpNft: (icpNft: number[]) => void;
}

interface UserInfo {
    userImageList: [];
    setUserImageList: (userImageList: []) => void;
}

interface StakeNftInfo {
    stakedNFTs: StakedNFTItem[];
    setStakedNFTs: (stakedNFTs: StakedNFTItem[]) => void;
}

// Create the store with Zustand
export const WalletStore = create<IWalletState>((set) => ({
    wallet: "",
    walletList: [],
    walletSelect: null,
    currentWallet: "",
    balance: 0,
    totalBalance: 0,
    setWallet: (wallet: "") => set({ wallet }),
    setWalletList: (walletList: WalletInfo[]) => set({ walletList }),
    setWalletSelect: (walletSelect: WalletInfo | null) => set({ walletSelect }),
    setCurrentWallet: (currentWallet: string) => set({ currentWallet }),
    setBalance: (balance: number) => set({ balance }),
    setTotalBalance: (totalBalance: number) => set({ totalBalance }),
}));

export const IcpWalletStore = create<ICPWalletState>((set) => ({
    IcpWallet: "",
    BindIcpWallet: "",
    icpNft: [],
    setIcpWallet: (IcpWallet: string) => set({ IcpWallet }),
    setBindIcpWallet: (BindIcpWallet: string) => set({ BindIcpWallet }),
    setIcpNft: (icpNft: number[]) => set({ icpNft }),
}));

export const UserInfo = create<UserInfo>((set) => ({
    userImageList: [],
    setUserImageList: (userImageList: []) => set({ userImageList })
}));

export const StakeNftInfo = create<StakeNftInfo>((set) => ({
    stakedNFTs: [],
    setStakedNFTs: (stakedNFTs: []) => set({ stakedNFTs })
}));
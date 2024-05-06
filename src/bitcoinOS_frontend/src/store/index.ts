import {create} from 'zustand';
// import { Principal } from "@dfinity/principal"
interface IWalletState {
  currentWallet: string;
  setCurrentWallet: (currentWallet:string) => void;
  }

  interface IStakepoolState {
    stakepoolCanister: string;
    setStakepoolCanister: (currentWallet:string) => void;
    }

 export  const WalletStore = create<IWalletState>((set) => ({
    currentWallet :"",
    setCurrentWallet:(currentWallet:string)=>set({currentWallet})
  }))


  export  const StakepoolStore = create<IStakepoolState>((set) => ({
    stakepoolCanister :'',
    setStakepoolCanister:(stakepoolCanister:string)=>set({stakepoolCanister})
  }))
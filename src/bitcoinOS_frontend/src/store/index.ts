import {create} from 'zustand';
interface IWalletState {
  currentWallet: string;
  setCurrentWallet: (currentWallet:string) => void;
  }

  const WalletStore = create<IWalletState>((set) => ({
    currentWallet :"",
    setCurrentWallet:(currentWallet:string)=>set({currentWallet})
  }))


  export default WalletStore;
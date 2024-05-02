import create from 'zustand';
interface IUserState {
    principal: string;
    setPrincipal: (principal:string) => void;
  }
  const UserStore = create<IUserState>((set) => ({
    principal :"",
    setPrincipal:(principal:string)=>set({principal})
  }))


  export default UserStore;
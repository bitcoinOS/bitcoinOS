import {

  Button,

}
  from '@chakra-ui/react'
import { useInternetIdentity } from "ic-use-internet-identity";
// import UserStore from "../store/index"
export function LoginButton() {
  // const { principal, setPrincipal } = UserStore();
  const { isLoggingIn, login, clear, identity } = useInternetIdentity();

  // If the user is logged in, clear the identity. Otherwise, log in.
  function handleClick() {
    if (identity) {
      clear();
    } else {
      login();
    }
  }


  const text = () => {
    if (identity) {
      const p = identity.getPrincipal().toString()
      return p.substring(0,5)+"..."+p.substring(p.length-3,p.length);
    } else if (isLoggingIn) {
      return (
        <>
          Logging in
        </>
      );
    }
    return "Connect Wallet";
  };

  return (
    <Button
      height="2.5rem"
      color="white"
      bgColor="orange.400"
      _hover={{ bg: "orange.200", borderColor: "orange.400" }}
      onClick={handleClick}
      >
      {text()}
    </Button>
  );
}

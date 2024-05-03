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
      return p.substring(0,5)+"..."+p.substring(p.length-5,p.length);
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
      bgColor="purple.500"
      _hover={{ bg: "purple.300", borderColor: "purple.500" }}
      onClick={handleClick}
      >
      {text()}
    </Button>
  );
}

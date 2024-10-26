import {
    Image,
    Button,

}
    from '@chakra-ui/react'
import { useState, useEffect } from 'react';
import { useInternetIdentity } from "ic-use-internet-identity";
import useGetStakePool from '../utils/poolActor'
import { getNetworkInfo } from '../store';
// import UserStore from "../store/index"
export function NetworkButton() {
    // const { principal, setPrincipal } = UserStore();
    const { isLoggingIn, login, clear, identity } = useInternetIdentity();
    const { get_network } = useGetStakePool()
    const { network } = getNetworkInfo()
    const [networkKey, setNetworkKey] = useState<string>("testnet");
    // If the user is logged in, clear the identity. Otherwise, log in.
    function handleClick() {
        if (identity) {
            clear();
        } else {
            login();
        }
    }


    const text = async () => {
        await get_network()
    };
    useEffect(() => {
        text()
    }, [])

    return (
        <Button
            height="2.5rem"
            color="#000000"
            bgColor="#F5F7FF"
            variant='outline'
            borderColor="#E6EAF5"
            leftIcon={<Image width='16px' src="./home/bitcoin-btc-logo.svg" />}
            _hover={{ bg: "orange.200", borderColor: "orange.400" }}
        >
            Bitcoin {network}
        </Button>
    );
}

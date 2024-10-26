import { ConectedWallectStorageKey, ConnectAccountType, ConnectWalletType } from "../store/useConnectStore";
import { useToast } from "@chakra-ui/react";
import React from 'react';
import { useConnectStore } from '../store/useConnectStore';
import { useInternetIdentity } from 'ic-use-internet-identity';
import { useSiwbIdentity } from 'ic-use-siwb-identity';

import { Principal } from '@dfinity/principal';

//export const icphost = 'http://localhost:4943'
export const icphost = 'https://identity.ic0.app'

export const backend_ip = 'https://api.bifipal.com/'

//export const nftCanisterId = 'asrmz-lmaaa-aaaaa-qaaeq-cai'
export const nftCanisterId = 'tfitq-wqaaa-aaaap-aa2xq-cai'

export function useIdentitySelect() {
    const {
        identity,
        isLoggingIn,
        login,
        clear,
        isLoginSuccess
    } = useInternetIdentity();

    const {
        identity: siwb_identity,
        identityAddress,
        clear: siwb_clear
    } = useSiwbIdentity();

    const [currentAccount, setCurrentAccount] = useConnectStore(
        (state) => [state.currentAccount, state.setCurrentAccount]
    );

    const identity_select = React.useMemo(() => {
        if (!currentAccount) {
            return undefined;
        }
        switch (currentAccount.type) {
            case 'UNISAT':
                return siwb_identity;
            case 'INTERNET_IDENTITY':
                return identity;
            default:
                return undefined;
        }
    }, [currentAccount, siwb_identity, identity]);

    return {
        identity,
        isLoggingIn,
        login,
        clear,
        isLoginSuccess,
        siwb_identity,
        identityAddress,
        siwb_clear,
        currentAccount,
        setCurrentAccount,
        identity_select
    };
}

// Abbreviated long strings
export const truncateMiddle = (str: string, startLength: number, endLength: number): string => {
    if (str.length <= startLength + endLength) {
        return str;
    }
    return `${str.substring(0, startLength)}...${str.substring(str.length - endLength)}`;
};


// Timestamp to date format (UTC)
export const formatDate = (timestamp): string => {
    const date = new Date(Number(timestamp) / 1000000);
    return `${date.getUTCFullYear()}-${String(date.getUTCMonth() + 1).padStart(2, '0')}-${String(date.getUTCDate()).padStart(2, '0')}`;
};

export const formatDateminute = (timestamp): string => {
    const date = new Date(Number(timestamp) / 1000000);

    const year = date.getUTCFullYear();
    const month = String(date.getUTCMonth() + 1).padStart(2, '0');
    const day = String(date.getUTCDate()).padStart(2, '0');
    const hours = String(date.getUTCHours()).padStart(2, '0');
    const minutes = String(date.getUTCMinutes()).padStart(2, '0');

    return `${year}-${month}-${day} ${hours}:${minutes}`;
};

// Check that ii's login has not expired.
export const checkIdentityExpiration = (identity: any) => {
    if (identity && identity._delegation) {
        const expiration = identity._delegation.delegations[0].delegation.expiration;
        const currentTime = BigInt(Date.now()) * 1000000n; // Convert current time to nanoseconds
        return expiration > currentTime;
    }
    return false;
};

export const getLastLoggedInWallet = () => {
    return localStorage.getItem(ConectedWallectStorageKey) as ConnectWalletType
};

export const satoshisToBTC = (satoshis: number): number => {
    // When the number of satoshi is relatively large, there may be precision problems, you can subsequently consider using bigNumber.js to convert the
    const btc = satoshis / 100000000; // 1 BTC = 100,000,000 Satoshis
    return btc;
}

export const isThirdPartyWallet = (account?: ConnectAccountType) => {
    return account?.type !== ConnectWalletType.INTERNET_IDENTITY;
};

export const handleCopy = (text) => {
    const toast = useToast()
    // copy address
    if (!text) return
    navigator.clipboard.writeText(text).then(() => {
        toast({
            title: "Address copied",
            status: "success",
            position: "bottom-right",
            duration: 2000,
            isClosable: true,
        })
    })
}

export async function computeExtTokenIdentifier(principal: Principal, index: number): Promise<string> {
    // Convert the prefix "tid" to Uint8Array
    const prefix = new TextEncoder().encode("\x0Atid");

    // Convert principal to Uint8Array
    const principalArray = principal.toUint8Array();

    // Combine prefix and principal
    let identifier = new Uint8Array([...prefix, ...principalArray]);

    // Convert index to 4 bytes (big-endian)
    const indexBytes = new Uint8Array(4);
    for (let i = 3; i >= 0; i--) {
        indexBytes[3 - i] = (index >> (i * 8)) & 0xff;
    }

    // Combine all parts
    identifier = new Uint8Array([...identifier, ...indexBytes]);

    // Convert back to Principal and then to text
    return Principal.fromUint8Array(identifier).toText();
}
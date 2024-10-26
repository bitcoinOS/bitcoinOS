/* eslint-disable react-refresh/only-export-components */
import React from 'react';
import {
    ActorProvider,
    InterceptorErrorData,
    InterceptorRequestData,
    InterceptorResponseData,
    createActorContext,
    createUseActorHook,
    isIdentityExpiredError,
} from "ic-use-actor";

import { SiwbIdentityProvider } from 'ic-use-siwb-identity';

import { canisterId, idlFactory } from "../../../declarations/ic_siwb_provider/index";

import { ReactNode } from "react";
import { _SERVICE } from "../../../declarations/ic_siwb_provider/ic_siwb_provider.did";
import toast from "react-hot-toast";
import { useInternetIdentity } from "ic-use-internet-identity";
import { useSiwbIdentity } from 'ic-use-siwb-identity';


import { WalletStore } from "../store/useWalletStore"
import { useConnectStore } from '../store/useConnectStore';

import { AnonymousIdentity } from "@dfinity/agent";


export default function siwbActors({ children }: { children: ReactNode }) {
    const { identity, clear } = useInternetIdentity();
    const noidentity = new AnonymousIdentity()
    const { identity: siwb_identity, identityAddress, clear: siwb_clear } = useSiwbIdentity();
    const [currentAccount, setCurrentAccount] = useConnectStore((state) => [state.currentAccount, state.setCurrentAccount])

    const { currentWallet, setCurrentWallet } = WalletStore();
    //const httpAgentOptions = import.meta.env.VITE_DFX_NETWORK === "local" ? {} : {
    //  "hsiwbt": "https://icp-api.io"
    //}
    const identity_select = React.useMemo(() => {
        if (!currentAccount) {
            return noidentity;
        }
        switch (currentAccount.type) {
            case 'UNISAT':
                return siwb_identity;
            case 'WIZZ':
                return siwb_identity;
            case 'INTERNET_IDENTITY':
                return identity;
            default:
                return noidentity;
        }
    }, [currentAccount, siwb_identity, identity, noidentity]);
    return (
        <SiwbIdentityProvider
            canisterId={canisterId}
            idlFactory={idlFactory}
            //httpAgentOptions={httpAgentOptions}
            httpAgentOptions={{ host: 'https://icp-api.io' }}
        >
            {children}
        </SiwbIdentityProvider>
    );
}

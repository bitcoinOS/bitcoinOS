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
import { canisterId, idlFactory } from "../../../declarations/os/index";

import { ReactNode } from "react";
import { _SERVICE } from "../../../declarations/os/os.did";
import toast from "react-hot-toast";
import { useInternetIdentity } from "ic-use-internet-identity";
import { useSiwbIdentity } from 'ic-use-siwb-identity';


import { WalletStore } from "../store/useWalletStore"
import { useConnectStore } from '../store/useConnectStore';

import { AnonymousIdentity } from "@dfinity/agent";

const actorContext = createActorContext<_SERVICE>();
export const useOsBackend = createUseActorHook<_SERVICE>(actorContext);

export { type UserProfileRequest, type BindRequest, type WalletInfo, type UserInfo, type Result, type StakingPoolInfo, type CreateStakingPoolRequest, type UserRequest } from "../../../declarations/os/os.did";

export default function OsActors({ children }: { children: ReactNode }) {
  const { identity, clear } = useInternetIdentity();
  const noidentity = new AnonymousIdentity()
  const { identity: siwb_identity, identityAddress, clear: siwb_clear } = useSiwbIdentity();
  const [currentAccount, setCurrentAccount] = useConnectStore((state) => [state.currentAccount, state.setCurrentAccount])

  const { currentWallet, setCurrentWallet } = WalletStore();
  const handleRequest = (data: InterceptorRequestData) => {
    console.log("onRequest", data.args, data.methodName);
    return data.args;
  };

  const handleResponse = (data: InterceptorResponseData) => {
    console.log("onResponse", data.args, data.methodName, data.response);
    return data.response;
  };

  const handleRequestError = (data: InterceptorErrorData) => {
    console.log("onRequestError", data.args, data.methodName, data.error);
    toast.error("Request error" as string, {
      position: "bottom-right",
    });
    return data.error;
  };

  const handleResponseError = (data: InterceptorErrorData) => {
    console.log("onResponseError", data.args, data.methodName, data.error);
    if (isIdentityExpiredError(data.error)) {
      toast.error("Login expired.", {
        id: "login-expired",
        position: "bottom-right",
      });
      setTimeout(() => {
        clear(); // Clears the identity from the state and local storage. Effectively "logs the user out".
        window.location.reload(); // Reloads the page to reset the UI.
      }, 1000);
      return;
    }

    if (
      typeof data === "object" &&
      data !== null &&
      "message" in data &&
      typeof data.message === "string"
    ) {
      toast.error(data.message, {
        position: "bottom-right",
      });
    }
  };
  //const httpAgentOptions = import.meta.env.VITE_DFX_NETWORK === "local" ? {} : {
  //  "host": "https://icp-api.io"
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
    <ActorProvider<_SERVICE>
      canisterId={canisterId}
      context={actorContext}
      identity={identity_select}
      idlFactory={idlFactory}
      //httpAgentOptions={httpAgentOptions}
      onRequest={handleRequest}
      onResponse={handleResponse}
      onRequestError={handleRequestError}
      onResponseError={handleResponseError}
    >
      {children}
    </ActorProvider>
  );
}

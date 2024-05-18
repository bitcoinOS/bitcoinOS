/* eslint-disable react-refresh/only-export-components */
import {
  ActorProvider,
  InterceptorErrorData,
  InterceptorRequestData,
  InterceptorResponseData,
  createActorContext,
  createUseActorHook,
  isIdentityExpiredError,
} from "ic-use-actor";
import { canisterId, idlFactory } from "../../../declarations/stakingpool/index";

import { ReactNode } from "react";
import { _SERVICE } from "../../../declarations/stakingpool/stakingpool.did";
import toast from "react-hot-toast";
import { useInternetIdentity } from "ic-use-internet-identity";
import { StakepoolStore } from "../store/index"

const actorContext = createActorContext<_SERVICE>();
export const useSatkePoolBackend = createUseActorHook<_SERVICE>(actorContext);

export { type RedeemRequest, type Result_3 as RedeemResponse } from "../../../declarations/stakingpool/stakingpool.did";

export default function StakePoolActors({ children }: { children: ReactNode }) {
  const { identity, clear } = useInternetIdentity();
  const { stakepoolCanister, setStakepoolCanister } = StakepoolStore();
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

  return (
    <ActorProvider<_SERVICE>
      canisterId={stakepoolCanister}
      context={actorContext}
      identity={identity}
      idlFactory={idlFactory}
      onRequest={handleRequest}
      onResponse={handleResponse}
      onRequestError={handleRequestError}
      onResponseError={handleResponseError}
    >
      {children}
    </ActorProvider>
  );
}

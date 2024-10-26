import React from 'react';
import ReactDOM from 'react-dom/client';
import App from './App';
import './index.scss';
import { InternetIdentityProvider } from "ic-use-internet-identity";
import {
  BrowserRouter
} from "react-router-dom";


import WalletActors from "./ic/WalletActors";
import OsActors from "./ic/OsActors";
import StakePoolActors from "./ic/StakePoolActors";
import PointActors from './ic/PointActors'
import StakeActors from './ic/StakeActors';

import { idlFactory as siwbIdl } from './idls/ic_siwb_provider.idl.ts';
import { SiwbIdentityProvider } from 'ic-use-siwb-identity';

ReactDOM.createRoot(document.getElementById('root')).render(
  <React.StrictMode>

    <SiwbIdentityProvider
      canisterId="45rbu-zqaaa-aaaah-qdeeq-cai"
      idlFactory={siwbIdl}
      //canisterId="by6od-j4aaa-aaaaa-qaadq-cai"
      //httpAgentoptions={{ host: 'http://127.0.0.1:4943' }}
      httpAgentOptions={{ host: 'https://icp-api.io' }}
    >
      <InternetIdentityProvider>
        <WalletActors>
          <OsActors>
            <StakePoolActors>
              <PointActors>
                <StakeActors>
                  <App />
                </StakeActors>
              </PointActors>
            </StakePoolActors>
          </OsActors>
        </WalletActors>
      </InternetIdentityProvider>
    </SiwbIdentityProvider>

  </React.StrictMode>
);

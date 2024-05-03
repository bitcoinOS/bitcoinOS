import React from 'react';
import ReactDOM from 'react-dom/client';
import App from './App';
import './index.scss';
import { InternetIdentityProvider } from "ic-use-internet-identity";
import {
  BrowserRouter
} from "react-router-dom";


import  WalletActors  from "./ic/WalletActors";
import  OsActors  from "./ic/OsActors";
import StakePoolActors  from "./ic/StakePoolActors";


ReactDOM.createRoot(document.getElementById('root')).render(
  <React.StrictMode>

    <InternetIdentityProvider>
      <WalletActors>
        <OsActors>
          <StakePoolActors>
            <App />
          </StakePoolActors>
        </OsActors>
      </WalletActors>
    </InternetIdentityProvider>

  </React.StrictMode>
);

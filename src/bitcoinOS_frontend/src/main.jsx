import React from 'react';
import ReactDOM from 'react-dom/client';
import App from './App';
import './index.scss';
import { InternetIdentityProvider } from "ic-use-internet-identity";
import {
  BrowserRouter
} from "react-router-dom";

 
import Stake from "./pages/stake/index"

 


 



ReactDOM.createRoot(document.getElementById('root')).render(
  <React.StrictMode>
     
    <InternetIdentityProvider>
    {/* <BrowserRouter  > */}
      <App />
     {/* </BrowserRouter> */}
    </InternetIdentityProvider>
 
  </React.StrictMode>
);

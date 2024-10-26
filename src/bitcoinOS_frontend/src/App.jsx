import { useState } from 'react';
import { ChakraProvider, Flex } from '@chakra-ui/react'
import { extendTheme } from '@chakra-ui/react'
import {
  BrowserRouter as Router,
  Routes, Route, useLocation
} from "react-router-dom";
import { ColorModeScript } from '@chakra-ui/react';

import { useEffect } from 'react';

import { initializeAnalytics, trackPageView } from './utils/analytics';

import Stake from "./pages/stake/index"
import Wallet from "./pages/wallet/index"
import Marathon from './pages/Leaderboard/index'
import Reward from './pages/Leaderboard/Reward'
import Leaderboard from './pages/Leaderboard/Leaderboard'
import Pools from './pages/Pools'
import Market from "./pages/market/index"
import Portfolio from './pages/portfolio';
import Docs from './pages/Docs'
import Header from "./components/Header"
import Footer from './components/Footer'

// import "/public/fonts/Ubuntu-font/Ubuntu-font.css";

const config = {
  initialColorMode: 'light',
  useSystemColorMode: false,
}

// Responsive Adaptive Breakpoints
const breakpoints = {
  base: '0px',
  sm: '500px',
  md: '960px',
  lg: '1280px',
  xl: '1560px',
  '2xl': '1920px',
  '3xl': '2500px',
}
// 3. extend the theme
const theme = extendTheme({
  config,
  breakpoints,
  fonts: {
    body: "Ubuntu",
    heading: "Ubuntu",
  },
})
const AppContent = () => {
  const location = useLocation();

  useEffect(() => {
    console.log('------------test')
    console.log(location.pathname)
    trackPageView(location.pathname);
  }, [location]);

  const ConditionalFooter = () => {
    const exactPathsWithoutFooter = ['/'];
    const prefixPathsWithoutFooter = ['/reward', '/leaderboard'];

    const shouldHideFooter =
      exactPathsWithoutFooter.includes(location.pathname) ||
      prefixPathsWithoutFooter.some(path => location.pathname.startsWith(path));

    return shouldHideFooter ? null : <Footer />;
  };

  return (
    <Flex direction="column" minH="100vh" bgGradient="linear(to-b, #F5F7FF 100%, #FFFFFF 100%)">
      <Header />
      <Flex direction="column" minH="78vh">
        <Routes>
          <Route path='/pools' element={<Pools />} />
          <Route path="/market" element={<Market />} />
          <Route path="/marathon" element={<Marathon />} />
          <Route path="/portfolio" element={<Portfolio />} />
          <Route path="/docs" element={<Docs />} />
          <Route path="/stake" element={<Stake />} />
          <Route path="/" element={<Marathon />} />
          <Route path="/reward" element={<Reward />} />
          <Route path="/leaderboard" element={<Leaderboard />} />
        </Routes>
      </Flex>
      <ConditionalFooter />
    </Flex>
  );
};

function App() {
  useEffect(() => {
    initializeAnalytics();
  }, []);

  useEffect(() => {
    const link = document.createElement('link');
    link.rel = 'stylesheet';
    link.href = '/fonts/Ubuntu-font/Ubuntu-font.css';
    document.head.appendChild(link);
  }, []);

  return (
    <ChakraProvider theme={theme}>
      <ColorModeScript initialColorMode={theme.config.initialColorMode} />
      <div className="App">
        <Router>
          <AppContent />
        </Router>
      </div>
    </ChakraProvider>
  );
}



export default App;

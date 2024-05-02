import { useState } from 'react';
import { ChakraProvider } from '@chakra-ui/react'
import { extendTheme } from '@chakra-ui/react'
import {
  BrowserRouter as Router,
  Routes, Route
} from "react-router-dom";
import Stake from "./pages/stake/index"
import Wallet from "./pages/wallet/index"
import  Header from "./components/Header"
 

const config = {
  initialColorMode: 'dark',
  useSystemColorMode: false,
}
// 3. extend the theme
const theme = extendTheme({ config })
function App() {
  const [greeting, setGreeting] = useState('');

  function handleSubmit(event) {
    event.preventDefault();
    const name = event.target.elements.name.value;
    // hello_icp2_backend.greet(name).then((greeting) => {
    //   setGreeting(greeting);
    // });
    setGreeting(name);
    return false;
  }

  return (
      <>
      <ChakraProvider theme={theme}>
      
      <div className="App">
      <Router>
      <Header />
        <Routes>
          <Route path="/wallet" element={<Wallet />}>
          </Route>
          <Route path="/" element={<Stake />}>
          </Route>
        </Routes>
       </Router>
    </div>
      </ChakraProvider>
      </>
  );
}

export default App;

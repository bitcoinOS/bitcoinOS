import { useState } from 'react';
// import { hello_icp2_backend } from 'declarations/hello_icp2_backend';
import { ChakraProvider } from '@chakra-ui/react'
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
    <ChakraProvider>
    <main>
      <img src="/logo2.svg" alt="DFINITY logo" />
      <br />
      <br />
      <form action="#" onSubmit={handleSubmit}>
        <label htmlFor="name">Enter your name: &nbsp;</label>
        <input id="name" alt="Name" type="text" />
        <button type="submit">Click Me!</button>
      </form>
      <section id="greeting">{greeting}</section>
    </main>
    </ChakraProvider>
  );
}

export default App;

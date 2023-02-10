import "./App.css";
import init, { greet, greet_json } from "wasm-game-of-life";
import { useEffect, useState } from "react";

function App() {
  const [isLoading, setIsLoading] = useState(true);
  const [name, setName] = useState("");

  useEffect(() => {
    init().then(() => {
      setIsLoading(false);
    });
  }, []);

  function handleGreet(name) {
    if (isLoading) return;

    // greet(name);
    greet_json(JSON.stringify({ name }));
  }

  return (
    <div className="App">
      <input
        value={name}
        onChange={(e) => setName(e.target.value)}
        type="text"
      />
      <button onClick={() => handleGreet(name)}>Greet</button>
    </div>
  );
}

export default App;

import "./App.css";
import init, { greet, greet_json } from "wasm-game-of-life";
import { useEffect, useState } from "react";

const TEXT_AREA_NAME = "input_box";

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

  function handleSubmit(e) {
    // Prevent the browser from reloading the page
    e.preventDefault();

    // Read the form data
    const data = Object.fromEntries(new FormData(e.target));
    const value = data[TEXT_AREA_NAME];

    // Or you can work with it as a plain object:
    console.log(value);
  }

  return (
    <div className="App">
      <input
        value={name}
        onChange={(e) => setName(e.target.value)}
        type="text"
      />
      <button onClick={() => handleGreet(name)}>Greet</button>
      <br></br>
      <br></br>
      <br></br>
      <div>
        <h1>day 4</h1>
        <form method="post" onSubmit={handleSubmit}>
          <textarea
            name={TEXT_AREA_NAME}
            rows="40"
            cols="50"
            defaultValue="input here"
          />
          <br></br>
          <button type="reset">Reset edits</button>
          <button type="submit">Save post</button>
        </form>
      </div>
    </div>
  );
}

export default App;

import "./App.css";
import init, { greet, greet_json, build_greeting } from "wasm-game-of-life";
import { useEffect, useState } from "react";

const TEXT_AREA_NAME = "input_box";
const SELECT_NAME = "select_box";

const DAYS = {
  day3: "day3",
  day4: "day4",
  day5: "day5",
};

const DAY_FUNC_MAP = {
  [DAYS.day3]: greet,
};

function App() {
  const [isLoading, setIsLoading] = useState(true);
  const [answer, setAnswer] = useState(null);

  useEffect(() => {
    init().then(() => {
      setIsLoading(false);
    });
  }, []);

  function handleSubmit(e) {
    // Prevent the browser from reloading the page
    e.preventDefault();

    if (isLoading) return;

    // Read the form data
    const data = Object.fromEntries(new FormData(e.target));
    console.log(data);
    const value = data[TEXT_AREA_NAME];

    console.log(greet(value));
  }

  return (
    <div className="App">
      <div>
        <form method="post" onSubmit={handleSubmit}>
          <h2>day:</h2>
          <select name={SELECT_NAME}>
            {Object.entries(DAYS).map(([key, val]) => (
              <option value={key}>{val}</option>
            ))}
          </select>

          <h2>input.txt:</h2>
          <textarea
            name={TEXT_AREA_NAME}
            rows="30"
            cols="50"
            defaultValue="input here"
          />
          <br></br>
          <button type="reset">Reset edits</button>
          <button type="submit">Save post</button>
        </form>
        {answer ? <h2>ANSWER: {answer}</h2> : <p>no answer</p>}
      </div>
    </div>
  );
}

export default App;

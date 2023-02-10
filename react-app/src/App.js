import "./App.css";
import init, { day3 } from "wasm-game-of-life";
import { useEffect, useState } from "react";

const TEXT_AREA_NAME = "input_box";
const SELECT_NAME = "select_box";

const DAYS = {
  day3: "day3",
  day4: "day4",
  day5: "day5",
};

const DAY_FUNC_MAP = {
  [DAYS.day3]: day3,
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
    e.preventDefault();

    if (isLoading) return;

    const data = Object.fromEntries(new FormData(e.target));
    console.log(data);
    const day = data[SELECT_NAME];
    const inputText = data[TEXT_AREA_NAME];

    const ans = DAY_FUNC_MAP[day](inputText);

    console.log(ans);

    setAnswer(ans);
  }

  const answerMarkup = (() => {
    if (!answer) {
      return <p>no answer</p>;
    }

    if (answer["pt1"] && answer["pt2"]) {
      return (
        <h2>
          ANSWER - a: {answer.pt1}, b: {answer.pt2}
        </h2>
      );
    }

    return <p>invalid input</p>;
  })();

  return (
    <div className="App">
      <div>
        <form method="post" onSubmit={handleSubmit}>
          <h2>day:</h2>
          <select name={SELECT_NAME}>
            {Object.entries(DAYS).map(([key, val]) => (
              <option key={key} value={key}>
                {val}
              </option>
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
          <button type="reset">reset</button>
          <button type="submit">calculate</button>
        </form>
        {answerMarkup}
      </div>
    </div>
  );
}

export default App;

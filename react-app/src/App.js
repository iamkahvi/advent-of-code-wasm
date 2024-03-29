import "./App.css";
import init, { day3, day4, day6, console_log } from "wasm-game-of-life";
import { useEffect, useState } from "react";

const TEXT_AREA_NAME = "input_box";
const SELECT_NAME = "select_box";

const DAYS = {
  day3: "day3",
  day4: "day4",
  day6: "day6",
};

const DAY_FUNC_MAP = {
  [DAYS.day3]: day3,
  [DAYS.day4]: day4,
  [DAYS.day6]: day6,
};

function App() {
  const [isLoading, setIsLoading] = useState(true);
  const [answer, setAnswer] = useState(null);
  const [textArea, setTextArea] = useState("");
  const [selectedDay, setSelectedDay] = useState(DAYS.day3);

  if (!isLoading) {
    console_log("HELLO RUST");
  }

  useEffect(() => {
    init().then(() => {
      setIsLoading(false);
    });

    const day = window.location.pathname.match("/(.*)")[1];

    if (day !== "") {
      console.log(day);
      setSelectedDay(day);
    }
  }, []);

  function handleSubmit(e) {
    e.preventDefault();

    if (isLoading) return;

    const data = Object.fromEntries(new FormData(e.target));
    console.log(data);
    // const day = data[SELECT_NAME];
    const inputText = data[TEXT_AREA_NAME];

    const ans = DAY_FUNC_MAP[selectedDay](inputText);

    console.log(ans);

    setAnswer(ans);
  }

  const answerMarkup = (() => {
    if (!answer) {
      return <p>no answer</p>;
    }

    if (answer["pt1"] === undefined || answer["pt2"] === undefined) {
      return <p>invalid input</p>;
    }

    return (
      <h2>
        ANSWER - a: {answer.pt1}, b: {answer.pt2}
      </h2>
    );
  })();

  function handleDayChange(e) {
    const day = e.target.value;

    setSelectedDay(day);
    setTextArea("");
  }

  return (
    <div className="App">
      <div>
        <form method="post" onSubmit={handleSubmit}>
          <h2>day:</h2>
          <select
            name={SELECT_NAME}
            onChange={handleDayChange}
            value={selectedDay}
          >
            {Object.entries(DAYS).map(([key, val]) => (
              <option key={key} value={key}>
                {val}
              </option>
            ))}
          </select>

          <h2>input.txt:</h2>
          <textarea
            name={TEXT_AREA_NAME}
            onChange={(e) => setTextArea(e.target.value)}
            value={textArea}
            placeholder={`${selectedDay} input here`}
            rows="30"
            cols="50"
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

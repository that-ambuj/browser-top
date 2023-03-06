import { useRef, useState } from "react";

// use localhost:8000 when in dev environment
const BASE_URL = import.meta.env.DEV ? "http://localhost:8000" : "";

function App() {
  const [data, setData] = useState<number[]>([]);
  const timerRef = useRef<any>();

  timerRef.current = setInterval(async () => {
    const response = await fetch(`${BASE_URL}/api/info`);

    if (response.status === 200) setData(await response.json());
  }, 1000);

  return (
    <>
      <div className="App">
        {data.map((usage, index) => {
          return (
            <div key={index}>
              Core #{index + 1}: {usage.toFixed(2)}
            </div>
          );
        })}
      </div>
    </>
  );
}

export default App;

import { useEffect, useState } from "react";

const BASE_URL = import.meta.env.DEV ? "http://localhost:8000" : "";

export function CpuInfo() {
  const [data, setData] = useState<number[]>(
    Array.from({ length: 8 }, () => 0.0)
  );

  let url = new URL(`${BASE_URL}/ws/cpu`, window.location.href);
  url.protocol = url.protocol.replace("http", "ws");

  useEffect(() => {
    let ws = new WebSocket(url);

    ws.onmessage = (event) => {
      setData(JSON.parse(event.data));
    };
  }, []);

  return (
    <>
      <h1 className="text-(4xl neutral-8) font-bold">CPU Usage:</h1>
      {data.map((usage, index) => {
        return (
          <div
            key={index}
            className="relative my-2 rounded-2 bg-green-1 max-w-lg z-0 h-10 flex items-center justify-between"
          >
            <div
              className="absolute rounded-2 bg-green-5 left-0 top-0 bottom-0 z-10 transition-width transition-duration-500"
              style={{ width: `${usage}%` }}
            ></div>
            <div
              className="relative z-999 m-auto font-medium transition-color transition-duration-300"
              style={{ color: usage > 50 ? "white" : "black" }}
            >
              {usage.toFixed(2)} %
            </div>
          </div>
        );
      })}
    </>
  );
}

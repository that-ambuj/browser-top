import { CpuInfo } from "./views";

// use localhost:8000 when in dev environment
export const BASE_URL = import.meta.env.DEV ? "http://localhost:8000" : "";

function App() {
  return (
    <>
      <CpuInfo />
    </>
  );
}

export default App;

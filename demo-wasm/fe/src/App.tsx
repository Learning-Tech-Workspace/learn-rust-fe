import { useEffect, useState } from "react";
import reactLogo from "./assets/react.svg";
import viteLogo from "/vite.svg";
import "./App.css";

function App() {
  const [count, setCount] = useState(0);

  async function loadWebAssembly(fileName: string) {
    const resp: Response = await fetch(fileName);
    const buffer: ArrayBuffer = await resp.arrayBuffer();
    const webAssembly = await WebAssembly.instantiate(buffer);
    console.log(webAssembly);
    return webAssembly.instance;
  }

  useEffect(() => {
    async function run() {
      const instance = await loadWebAssembly("/rust-test.wasm");
      console.log(instance);
      console.log(instance.exports.add);
      const add = instance.exports.add as (a: bigint, b: bigint) => bigint;
      const mul = instance.exports.mul as (a: bigint, b: bigint) => bigint;
      const sum = add(BigInt(40), BigInt(2));
      const diff = mul(BigInt(40), BigInt(2));
      console.log(sum);
      console.log(diff);
    }
    run();
  }, []);

  return (
    <>
      <div>
        <a href="https://vitejs.dev" target="_blank">
          <img src={viteLogo} className="logo" alt="Vite logo" />
        </a>
        <a href="https://react.dev" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>
      <h1>Vite + React</h1>
      <div className="card">
        <button onClick={() => setCount((count) => count + 1)}>
          count is {count}
        </button>
        <p>
          Edit <code>src/App.tsx</code> and save to test HMR
        </p>
      </div>
      <p className="read-the-docs">
        Click on the Vite and React logos to learn more
      </p>
    </>
  );
}

export default App;

import { useState } from "react";
// 從 Tauri API 導入 invoke 函數
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
  const [greetingMessage, setGreetingMessage] = useState("");
  const [name, setName] = useState("World");

  async function greetUser() {
    console.log("準備呼叫後端命令...");
    // 呼叫後端名為 'greet' 的命令，並傳遞 'name' 參數
    try {
      const message = await invoke("greet", { name: name });
      setGreetingMessage(message);
    } catch (error) {
      console.error("Failed to invoke greet command:", error);
    }
  }

  return (
    <div className='container'>
      <h1>歡迎使用 Tauri 2 + React！</h1>

      <div className='row'>
        <div>
          <input
            id='greet-input'
            onChange={(e) => setName(e.currentTarget.value)}
            placeholder='輸入你的名字...'
            value={name}
          />
          <button type='button' onClick={greetUser}>
            發送至 Rust 後端
          </button>
        </div>
      </div>

      {greetingMessage && <p>{greetingMessage}</p>}
    </div>
  );
}

export default App;

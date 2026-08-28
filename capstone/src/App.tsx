import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";

// Mirror of the Rust `Todo` struct (serde-serialized across the IPC bridge).
type Todo = { id: number; text: string; done: boolean };

export default function App() {
  const [todos, setTodos] = useState<Todo[]>([]);
  const [text, setText] = useState("");

  async function refresh() {
    setTodos(await invoke<Todo[]>("list_todos"));
  }

  useEffect(() => {
    refresh();
  }, []);

  async function add() {
    if (!text.trim()) return;
    await invoke("add_todo", { text }); // call Rust command
    setText("");
    await refresh();
  }

  async function toggle(id: number) {
    await invoke("toggle_todo", { id });
    await refresh();
  }

  return (
    <main className="min-h-screen bg-slate-900 text-slate-100 flex items-center justify-center p-6">
      <div className="w-full max-w-md rounded-2xl bg-slate-800 shadow-xl p-6">
        <h1 className="text-2xl font-bold mb-4">Tauri Todos</h1>
        <p className="text-sm text-slate-400 mb-4">
          React 19 + Tailwind 4 + Vite frontend, Rust backend.
        </p>

        <div className="flex gap-2 mb-4">
          <input
            className="flex-1 rounded-lg bg-slate-700 px-3 py-2 outline-none focus:ring-2 ring-indigo-400"
            value={text}
            onChange={(e) => setText(e.target.value)}
            onKeyDown={(e) => e.key === "Enter" && add()}
            placeholder="Add a todo…"
          />
          <button
            className="rounded-lg bg-indigo-500 hover:bg-indigo-400 px-4 py-2 font-medium"
            onClick={add}
          >
            Add
          </button>
        </div>

        <ul className="space-y-2">
          {todos.map((t) => (
            <li
              key={t.id}
              className="flex items-center gap-3 rounded-lg bg-slate-700/60 px-3 py-2 cursor-pointer"
              onClick={() => toggle(t.id)}
            >
              <span
                className={`h-4 w-4 rounded-full border ${
                  t.done ? "bg-emerald-400 border-emerald-400" : "border-slate-400"
                }`}
              />
              <span className={t.done ? "line-through text-slate-400" : ""}>
                {t.text}
              </span>
            </li>
          ))}
          {todos.length === 0 && (
            <li className="text-slate-500 text-sm">No todos yet.</li>
          )}
        </ul>
      </div>
    </main>
  );
}

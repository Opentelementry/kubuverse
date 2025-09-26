import { useState, useEffect } from "react";

export default function Home() {
  const [items, setItems] = useState([]);
  const [newName, setNewName] = useState("");

  useEffect(() => {
    fetch("/api/items")
      .then(res => res.json())
      .then(setItems)
      .catch(console.error);
  }, []);

  const addItem = async () => {
    if (!newName) return;
    const res = await fetch("/api/items", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ name: newName })
    });
    const data = await res.json();
    setItems([...items, data]);
    setNewName("");
  };

  return (
    <main style={{ padding: 20 }}>
      <h1>Shared Items</h1>
      <input
        value={newName}
        onChange={(e) => setNewName(e.target.value)}
        placeholder="Enter item name"
      />
      <button onClick={addItem}>Add</button>
      <ul>
        {items.map(i => (
          <li key={i.id}>{i.name}</li>
        ))}
      </ul>
    </main>
  );
}

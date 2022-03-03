import { For } from "solid-js";
import { createStore, produce } from "solid-js/store";

interface Todo {
    id: number;
    text: string;
    completed: boolean;
}

interface Store {
    todos: Todo[];
}

const MutableStoreExample = () => {
    let input: HTMLInputElement;
    let todoId = 0;
    const [store, setStore] = createStore<Store>({todos: []});

    const addTodo = (text: string) => {
        setStore(
            "todos",
            produce(todos => todos.push({id: ++todoId, text, completed: false}))
        );
    }

    const toggleTodo = (id: number) => {
        setStore(
            "todos",
            todo => todo.id === id,
            produce(todo => todo.completed = !todo.completed)
        )
    }

    return (
        <>
          <div>
            <input ref={input!} />
            <button
              onClick={(e) => {
                if (!input.value.trim()) return;
                addTodo(input.value);
                input.value = "";
              }}
            >
              Add Todo
            </button>
          </div>
          <For each={store.todos}>
            {(todo) => {
              const { id, text } = todo;
              console.log(`Creating ${text}`)
              return <div>
                <input
                  type="checkbox"
                  checked={todo.completed}
                  onchange={[toggleTodo, id]}
                />
                <span
                  style={{ "text-decoration": todo.completed ? "line-through" : "none"}}
                >{text}</span>
              </div>
            }}
          </For>
        </>
      );
}

export default MutableStoreExample;
import { Accessor, createSignal, For, Setter } from "solid-js";

interface Todo {
    id: number;
    text: string;
    completed: Accessor<boolean>;
    setCompleted: Setter<boolean>;
}

const NestedReactivityExample = () => {
    const [todos, setTodos] = createSignal<Todo[]>([]);
    let input: HTMLInputElement;
    let todoId = 0;

    const addTodo = (text: string) => {
        const [completed, setCompleted] = createSignal(false);
        setTodos([...todos(), { id: ++todoId, text, completed, setCompleted}]);
    }

    const toggleTodo = (id: number) => {
        const index = todos().findIndex(todo => todo.id === id);
        const todo = todos()[index];

        if (todo) {
            todo.setCompleted(!todo.completed())
        }
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
          <For each={todos()}>
            {(todo) => {
              const { id, text } = todo;
              console.log(`Creating ${text}`)
              return <div>
                <input
                  type="checkbox"
                  checked={todo.completed()}
                  onchange={[toggleTodo, id]}
                />
                <span
                  style={{ "text-decoration": todo.completed() ? "line-through" : "none"}}
                >{text}</span>
              </div>
            }}
          </For>
        </>
      );
}

export default NestedReactivityExample;
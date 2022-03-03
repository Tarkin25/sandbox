import { createEffect, createSignal } from "solid-js";

const Counter = () => {
    const [count, setCount] = createSignal(0);

    setInterval(() => setCount(count() + 1), 1000);

    return <p>Count: {count()}</p>;
}

export default Counter;
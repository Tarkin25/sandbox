import { createSignal, onCleanup } from "solid-js";

const OnCleanupExamaple = () => {
    const [count, setCount] = createSignal(0);

    const interval = setInterval(() => setCount(count() + 1), 1000);

    onCleanup(() => clearInterval(interval));

    return (
        <div>Count: {count()}</div>
    )
}

export default OnCleanupExamaple;
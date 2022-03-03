import { createSignal, onCleanup } from "solid-js";

const StyleExample = () => {
    const [num, setNum] = createSignal(0);

    const interval = setInterval(() => setNum((num() + 1) % 255), 30);

    onCleanup(() => clearInterval(interval));

    return (
        <h2
            style={{
                color: `rgb(${num()}, 180, ${num()})`,
                "font-weight": 800,
            }}
        >
            Some Text
        </h2>
    )
}

export default StyleExample;
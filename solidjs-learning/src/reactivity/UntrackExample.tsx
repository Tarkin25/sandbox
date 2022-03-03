import { createEffect, createSignal, untrack } from "solid-js";

const UntrackExample = () => {
    const [a, setA] = createSignal(1);
    const [b, setB] = createSignal(1);

    createEffect(() => {
        console.log(a(), untrack(b));
    })

    return (
        <>
            <button onClick={() => setA(a() + 1)}>Increment A</button>
            <button onClick={() => setB(b() + 1)}>Increment B</button>
        </>
    )
}

export default UntrackExample;
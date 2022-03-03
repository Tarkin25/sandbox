import { Accessor, Component, createContext, createSignal, useContext } from "solid-js";

interface CounterActions {
    increment(): void;
    decrement(): void;
}

type CounterStore = [Accessor<number>, CounterActions];

const CounterContext = createContext<CounterStore>();

const CounterProvider: Component = (props) => {
    const [count, setCount] = createSignal(0);

    const increment = () => setCount(count => count + 1);
    const decrement = () => setCount(count => count - 1);

    const store: CounterStore = [
        count,
        {
            increment,
            decrement
        }
    ]

    return (
        <CounterContext.Provider value={store}>
            {props.children}
        </CounterContext.Provider>
    )
}

const useCounter = () => useContext(CounterContext)!;

const Nested = () => {
    const [count, { increment, decrement }] = useCounter();

    return (
        <>
            <div>{count()}</div>
            <button onClick={increment}>+</button>
            <button onClick={decrement}>-</button>
        </>
    )
}

const ContextExample = () => {
    return (
        <CounterProvider>
            <h1>Counter with Context</h1>
            <Nested />
        </CounterProvider>
    )
}

export default ContextExample;
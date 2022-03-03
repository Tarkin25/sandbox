import { Component, createSignal, mergeProps } from "solid-js";

const Greeting: Component<{name?: string; greeting?: string;}> = (props) => {
    const merged = mergeProps({greeting: "Hi", name: "John"}, props);

    // IMPORTANT: don't destructure props, as their attributes are reactive getters which only work with the dot syntax
    return <h3>{merged.greeting} {merged.name}</h3>
}

const DefaultPropsExample = () => {
    const [name, setName] = createSignal<string>();

    return (
        <>
            <Greeting greeting="Hello" />
            <Greeting name="Jeremy" />
            <Greeting name={name()} />
            <button onClick={() => setName("Jarod")}>Set Name</button>
        </>
    )
}

export default DefaultPropsExample;
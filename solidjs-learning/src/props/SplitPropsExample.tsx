import { Component, createSignal, JSX, splitProps } from "solid-js";

const Greeting: Component<{greeting: string; name: string;} & JSX.HTMLAttributes<HTMLHeadingElement>> = (props) => {
    const [local, others] = splitProps(props, ["greeting", "name"]);

    return <h3 {...others}>{local.greeting} {local.name}</h3>
}

const SplitPropsExample = () => {
    const [name, setName] = createSignal("Jakob");

    return (
        <>
            <Greeting greeting="Yo" name={name()} style="color: teal;" />
            <button onClick={() => setName("Jarod")}>Set Name</button>
        </>
    )
}

export default SplitPropsExample;
import { Accessor, children, Component, createEffect, createSignal, For } from "solid-js";

const ColoredList: Component<{color: string;}> = (props) => {
    const _children = children(() => props.children) as Accessor<HTMLDivElement[]>;

    createEffect(() => {
        _children().forEach(child => child.style.color = props.color);
    })

    return <>{_children()}</>
}

const ChildrenExample = () => {
    const [color, setColor] = createSignal("purple");

    return (
        <>
            <ColoredList color={color()}>
                <For each={["Most", "Interesting", "Thing"]}>
                    {item => <div>{item}</div>}
                </For>
            </ColoredList>
            <button onClick={() => setColor("teal")}>Set Color</button>
        </>
    )
}

export default ChildrenExample;
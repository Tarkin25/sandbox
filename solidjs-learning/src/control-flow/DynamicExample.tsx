import { createSignal, For } from "solid-js";
import { Dynamic } from "solid-js/web";

const RedThing = () => <strong style="color: red">Red Thing</strong>;
const GreenThing = () => <strong style="color: green">Green Thing</strong>;
const BlueThing = () => <strong style="color: blue">Blue Thing</strong>;

const colors = {
    red: RedThing,
    green: GreenThing,
    blue: BlueThing,
};

type Color = keyof typeof colors;

const DynamicExample = () => {
    const [color, setColor] = createSignal<Color>("red");

    return (
        <>
            <select value={color()} onInput={e => setColor(e.currentTarget.value as Color)}>
                <For each={Object.keys(colors)}>
                    {color => <option value={color}>{color}</option>}
                </For>
            </select>
            <Dynamic component={colors[color()]} />
        </>
    )
}

export default DynamicExample;
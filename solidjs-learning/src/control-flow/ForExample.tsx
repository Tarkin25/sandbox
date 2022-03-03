import { createSignal, For } from "solid-js";

const ForExample = () => {
    const [cats] = createSignal(["Lucky", "Cheeky", "Baboule", "Leo", "Mia"]);

    return (
        <div>
            <ul>
                <For each={cats()}>{(cat) => <li>{cat}</li>}</For>
            </ul>
        </div>
    );
};

export default ForExample;

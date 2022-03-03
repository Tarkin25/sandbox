import { createSignal, Index } from "solid-js";

const IndexExample = () => {
    const [cats] = createSignal(["Cheeky", "Lucky", "Baboule", "Leo"]);

    return (
        <ul>
            <Index each={cats()}>
                {(cat, index) => (
                    <li>
                        {index}: {cat()}
                    </li>
                )}
            </Index>
        </ul>
    )
}

export default IndexExample;
import { Component, For } from "solid-js";
import asyncExamples from "./async";
import bindingsExamples from "./bindings";
import controlFlowExamples from "./control-flow";

import Counter from "./counter/Counter";
import lifecycleExamples from "./lifecycles";
import propsExamples from "./props";
import reactivityExamples from "./reactivity";
import storesExamples from "./stores";

const App: Component = () => {
    const components = [
        <Counter />,
        ...controlFlowExamples,
        ...lifecycleExamples,
        ...bindingsExamples,
        ...propsExamples,
        ...storesExamples,
        ...reactivityExamples,
        ...asyncExamples,
    ];

    return (
        <div>
            <For each={components}>
                {(component) => (
                    <>
                        {component}
                        <hr />
                    </>
                )}
            </For>
        </div>
    );
};

export default App;

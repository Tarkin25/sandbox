import { createSignal, Match, Switch } from "solid-js";

const MatchExample = () => {
    const [number] = createSignal(7);

    return (
        <p>
            <Switch fallback={`${number()} is between 5 and 10`}>
                <Match when={number() > 10}>{number()} is greater than 10</Match>
                <Match when={5 > number()}>{number()} is less than 5</Match>
            </Switch>
        </p>
    )
}

export default MatchExample;
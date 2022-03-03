import { createSignal, Show } from "solid-js";

const ShowExample = () => {
    const [loggedIn, setLoggedIn] = createSignal(false);

    const toggle = () => setLoggedIn(!loggedIn());

    return (
        <button onClick={toggle}>
            <Show when={loggedIn()} fallback="Log in">
                Log out
            </Show>
        </button>
    )
}

export default ShowExample;
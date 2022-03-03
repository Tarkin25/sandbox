import { lazy } from "solid-js";

const Greeting = lazy(async () => {
    await new Promise(resolve => setTimeout(resolve, 1000));
    return import("./Greeting");
})

const LazyExample = () => {
    return (
        <>
            <h1>Lazy Example</h1>
            <Greeting />
        </>
    )
}

export default LazyExample;
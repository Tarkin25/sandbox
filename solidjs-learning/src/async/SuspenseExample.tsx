import { lazy, Suspense } from "solid-js";

const Greeting = lazy(async () => {
    await new Promise(resolve => setTimeout(resolve, 1000));
    return import("./Greeting");
})

const SuspenseExample = () => {
    return (
        <>
            <h1>Suspense Example</h1>
            <Suspense fallback={<p>Loading...</p>}>
                <Greeting />
            </Suspense>
        </>
    )
}

export default SuspenseExample;
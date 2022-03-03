import { ErrorBoundary } from "solid-js";

const Broken = () => {
    throw new Error("Oh No!");

    return <>Never getting here...</>
}

const ErrorBoundaryExample = () => {
    return (
        <>
            <div>Before</div>
            <ErrorBoundary fallback={error => error}>
                <Broken />
            </ErrorBoundary>
            <div>After</div>
        </>
    )
}

export default ErrorBoundaryExample;
import { batch, createSignal } from "solid-js";

const BatchExample = () => {
    const [firstname, setFirstname] = createSignal("John");
    const [lastname, setLastname] = createSignal("Doe");

    const fullname = () => {
        console.log("Running fullname");
        return `${firstname()} ${lastname()}`;
    };

    const updateNames = () => {
        console.log("Running updateNames");
        batch(() => {
            setFirstname(firstname() + "n");
            setLastname(lastname() + "!");
        });
    };

    return <button onClick={updateNames}>My name is {fullname()}</button>;
};

export default BatchExample;

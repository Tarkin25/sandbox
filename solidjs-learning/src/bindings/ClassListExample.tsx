import { createSignal } from "solid-js";
import styles from "./ClassListExample.module.css";

const ClassListExample = () => {
    const [current, setCurrent] = createSignal("foo");

    const Button = ({name}: {name: string}) => (
        <button classList={{[styles.button]: true, [styles.selected]: current() === name}} onClick={() => setCurrent(name)}>{name}</button>
    )

    return (
        <>
            <Button name="foo" />
            <Button name="bar" />
            <Button name="baz" />
        </>
    )
};

export default ClassListExample;

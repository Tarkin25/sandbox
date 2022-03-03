import { createSignal, For } from "solid-js";
import styles from "./EventExample.module.css";

const EventExample = () => {
    const [pos, setPos] = createSignal({ x: 0, y: 0 });

    const handleMouseMove = (event: MouseEvent) => {
        setPos({ x: event.clientX, y: event.clientY });
    };
    const names = ["John", "Anna", "Kevin"];

    const handleNameClick = (name: string, _event: MouseEvent) => {
        alert(`You clicked "${name}"`);
    }

    return (
        <>
            <div class={styles.area} onMouseMove={handleMouseMove}>
                The mouse position is {pos().x} x {pos().y} y
            </div>
            <For each={names}>
                {name => (
                    <button onClick={[handleNameClick, name]}>{name}</button>
                )}
            </For>
        </>
    );
};

export default EventExample;

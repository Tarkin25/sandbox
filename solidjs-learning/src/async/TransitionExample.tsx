import {
    Component,
    createResource,
    createSignal,
    Match,
    Suspense,
    Switch,
    useTransition,
} from "solid-js";
import "./TransitionExample.css";

const CONTENT = {
    Uno: `Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.`,
    Dos: `Sed ut perspiciatis unde omnis iste natus error sit voluptatem accusantium doloremque laudantium, totam rem aperiam, eaque ipsa quae ab illo inventore veritatis et quasi architecto beatae vitae dicta sunt explicabo. Nemo enim ipsam voluptatem quia voluptas sit aspernatur aut odit aut fugit, sed quia consequuntur magni dolores eos qui ratione voluptatem sequi nesciunt. Neque porro quisquam est, qui dolorem ipsum quia dolor sit amet, consectetur, adipisci velit, sed quia non numquam eius modi tempora incidunt ut labore et dolore magnam aliquam quaerat voluptatem. Ut enim ad minima veniam, quis nostrum exercitationem ullam corporis suscipit laboriosam, nisi ut aliquid ex ea commodi consequatur? Quis autem vel eum iure reprehenderit qui in ea voluptate velit esse quam nihil molestiae consequatur, vel illum qui dolorem eum fugiat quo voluptas nulla pariatur?`,
    Tres: `On the other hand, we denounce with righteous indignation and dislike men who are so beguiled and demoralized by the charms of pleasure of the moment, so blinded by desire, that they cannot foresee the pain and trouble that are bound to ensue; and equal blame belongs to those who fail in their duty through weakness of will, which is the same as saying through shrinking from toil and pain. These cases are perfectly simple and easy to distinguish. In a free hour, when our power of choice is untrammelled and when nothing prevents our being able to do what we like best, every pleasure is to be welcomed and every pain avoided. But in certain circumstances and owing to the claims of duty or the obligations of business it will frequently occur that pleasures have to be repudiated and annoyances accepted. The wise man therefore always holds in these matters to this principle of selection: he rejects pleasures to secure other greater pleasures, or else he endures pains to avoid worse pains.`,
};

function createDelay() {
    return new Promise<number>((resolve) => {
        const delay = Math.random() * 420 + 160;
        setTimeout(() => resolve(delay), delay);
    });
}

const Child: Component<{ page: keyof typeof CONTENT }> = (props) => {
    const [time] = createResource(createDelay);

    return (
        <div class="tab-content">
            This content is for page "{props.page}" after {time()?.toFixed()}ms.
            <p>{CONTENT[props.page]}</p>
        </div>
    );
};

const TransitionExample = () => {
    const [tab, setTab] = createSignal(0);
    const [pending, start] = useTransition();

    const updateTab = (index: number) => start(() => setTab(index));

    return (
        <>
            <ul class="inline">
                <li
                    classList={{ selected: tab() === 0 }}
                    onClick={[updateTab, 0]}
                >
                    Uno
                </li>
                <li
                    classList={{ selected: tab() === 1 }}
                    onClick={[updateTab, 1]}
                >
                    Dos
                </li>
                <li
                    classList={{ selected: tab() === 2 }}
                    onClick={[updateTab, 2]}
                >
                    Tres
                </li>
            </ul>
            <div class="tab" classList={{ "pending": pending() }}>
                <Suspense fallback={<div class="loader">Loading...</div>}>
                    <Switch>
                        <Match when={tab() === 0}>
                            <Child page="Uno" />
                        </Match>
                        <Match when={tab() === 1}>
                            <Child page="Dos" />
                        </Match>
                        <Match when={tab() === 2}>
                            <Child page="Tres" />
                        </Match>
                    </Switch>
                </Suspense>
            </div>
        </>
    );
};

export default TransitionExample;

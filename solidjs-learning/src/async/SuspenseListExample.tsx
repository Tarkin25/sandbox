import {
    Component,
    createResource,
    For,
    Suspense,
    SuspenseList,
} from "solid-js";

const fetchUserProfile = async (id: number) => {
    await new Promise((resolve) => setTimeout(resolve, id * 1000));

    return await fetch(`https://jsonplaceholder.typicode.com/users/${id}`).then(
        (res) => res.json()
    );
};

const UserProfile: Component<{ id: number }> = (props) => {
    const [user] = createResource(props.id, fetchUserProfile);

    return (
        <div>
            {user()?.id}: {user()?.name}
        </div>
    );
};

const SuspenseListExample = () => {
    return (
        <SuspenseList revealOrder="forwards">
            <For each={[1, 2, 3, 4]}>{(id) => (
                <Suspense fallback={<p>Loading user {id}...</p>}>
                    <UserProfile id={id} />
                </Suspense>
            )}</For>
        </SuspenseList>
    );
};

export default SuspenseListExample;

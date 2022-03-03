import { createResource, For, Suspense } from "solid-js";

const fetchUsers = async () => {
    await new Promise(resolve => setTimeout(resolve, 2000));
    return await fetch("https://jsonplaceholder.typicode.com/users").then(res => res.json());
};

const UserList = () => {
    const [users] = createResource(fetchUsers);

    return (
        <div>
            <For each={users()}>
                {(user: any) => (
                    <div>{user.name}</div>
                )}
            </For>
        </div>
    )
}

const SuspendWithResourceExample = () => {
    return (
        <>
            <h1>Resource with Suspense</h1>
            <Suspense fallback={<p>Loading users...</p>}>
                <UserList />
            </Suspense>
        </>
    )
}

export default SuspendWithResourceExample;
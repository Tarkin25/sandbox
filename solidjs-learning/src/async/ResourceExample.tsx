import { createResource, createSignal } from "solid-js";

const fetchUser = async (id: string) => {
    const res = await fetch(`https://swapi.dev/api/people/${id}`);
    return res.json();
}

const ResourceExample = () => {
    const [userId, setUserId] = createSignal("1");

    const [user] = createResource(userId, fetchUser);

    return (
        <>
          <input
            type="number"
            min="1"
            placeholder="Enter Numeric Id"
            onInput={(e) => setUserId(e.currentTarget.value)}
          />
          <span>{user.loading && "Loading..."}</span>
          <div>
            <pre>{JSON.stringify(user(), null, 2)}</pre>
          </div>
        </>
      );
}

export default ResourceExample;
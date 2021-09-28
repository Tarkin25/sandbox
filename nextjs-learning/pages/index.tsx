import Head from "next/head";
import { useSelector } from "react-redux";
import { RootState } from "../app/store";

export interface Todo {
  id: number;
  title: string;
  completed: boolean;
}

const Home = () => {
  const count = useSelector((state: RootState) => state);

  return (
    <>
      <Head>
        <title>Counter</title>
      </Head>
      <h2>Counter</h2>
      <section>
        {count}
      </section>
    </>
  )
}

export default Home;



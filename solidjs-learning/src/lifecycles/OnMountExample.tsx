import { createSignal, For, onMount } from "solid-js";
import styles from "./OnMountExample.module.css";

interface Photo {
    thumbnailUrl: string;
    title: string;
}

const OnMountExample = () => {
    const [photos, setPhotos] = createSignal<Photo[]>([]);

    onMount(async () => {
        await new Promise(resolve => setTimeout(resolve, 1000))

        const photos: Photo[] = await fetch(
            "https://jsonplaceholder.typicode.com/photos?_limit=20"
        ).then((res) => res.json());

        setPhotos(photos);
    });

    return (
        <>
            <h1>Photo Album</h1>

            <div class={styles.photos}>
                <For each={photos()} fallback={<p>Loading...</p>}>
                    {(photo) => (
                        <figure>
                            <img src={photo.thumbnailUrl} alt={photo.title} />
                            <figcaption>{photo.title}</figcaption>
                        </figure>
                    )}
                </For>
            </div>
        </>
    );
};

export default OnMountExample;

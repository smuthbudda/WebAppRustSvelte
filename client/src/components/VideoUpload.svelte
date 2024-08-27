<!-- <script lang="ts">
    import { FFmpeg } from "@ffmpeg/ffmpeg";
    import { fetchFile, toBlobURL } from "@ffmpeg/util";
    import { onMount } from "svelte";
    import { fade } from "svelte/transition";

    enum state {
        loading,
        loaded,
        start,
        error,
        done,
    }

    let appState: state = state.loading;
    let error = "";
    let ffmpeg: FFmpeg;
    const fileTypes: string[] = ["video/mp4", "video/quicktime", "video/webm"];

    async function convertVideo(video: File) {
        appState = state.start;
        const videoData = await readFile(video);

        await ffmpeg.writeFile("input.mov", videoData);
        await ffmpeg.exec(["-1", "input.mov", "output.mp4"]);
        const data = await ffmpeg.readFile("output.mp4");
        appState = state.done;
        return data as Uint8Array;
    }

    async function handleDrop(event: DragEvent) {
        console.log(event.dataTransfer);

        if (!event.dataTransfer) return;

        if (event.dataTransfer.files.length > 1) {
            error = "Upload one file";
        }
        console.log(event.dataTransfer.files[0].type);
        if (fileTypes.includes(event.dataTransfer.files[0].type)) {
            error = "";
            const [file] = event.dataTransfer.files;
        } else {
            error = "File type incorrect";
        }

        console.log(error);
    }

    async function readFile(file: File): Promise<Uint8Array> {
        return new Promise((resolve) => {
            const fileReader = new FileReader();

            fileReader.onload = () => {
                const { result } = fileReader;
                if (result instanceof ArrayBuffer) {
                    resolve(new Uint8Array(result));
                }
            };

            fileReader.onerror = () => {
                error = "Could not read the file";
            };

            fileReader.readAsArrayBuffer(file);
        });
    }

    async function loadFFmpeg() {
        const baseURL = "https://unpkg.com/@ffmpeg/core@0.12.6/dist/esm";
        ffmpeg = new FFmpeg();
        await ffmpeg.load({
            coreURL: await toBlobURL(
                `${baseURL}/ffmpeg-core.js`,
                "text/javascript",
            ),
            wasmURL: await toBlobURL(
                `${baseURL}/ffmpeg-core.wasm`,
                "application/wasm",
            ),
        });
        appState = state.loaded;
    }

    onMount(() => {
        loadFFmpeg();
    });
</script>

<h1 class="title">Track and Field Splits</h1>

<!-- svelte-ignore a11y-no-static-element-interactions -->
<div
    on:drop|preventDefault={handleDrop}
    on:dragover|preventDefault={() => {}}
    data-state={appState}
    id="drop_zone"
>
    {#if appState === state.loading}
        <p in:fade>Loading FFmpeg</p>
    {/if}

    {#if appState === state.loaded}
        <p in:fade>Drag video here</p>
    {/if}

    {#if appState === state.start}
        <p in:fade>Converting Video</p>
    {/if}

    {error}
</div>

<style>
    #drop_zone {
        border: 5px solid blue;
        width: 100%;
        height: 100px;
    }
</style> -->

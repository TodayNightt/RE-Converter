import { createStore } from "solid-js/store";
import type { IoStore } from "./io_store";

export const [lastSavedStore, setLastSavedStore] = createStore<IoStore>({
    inputFolder: null,
    outputFolder: null,
    resolution: null,
    audioBitrate: null,
    videoBitrate: null,
    audioCodec: null,
    videoCodec: null,
    pictureFormat: null,
    outputExtension: null,
    needSorting: true,
});


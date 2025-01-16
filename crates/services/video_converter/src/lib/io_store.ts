
import { createStore } from "solid-js/store";
import type { AudioCodec, ConverterOptions, OutputExtension, PictureFormat, Resolution, VideoCodec } from "./types-backend";

export type IoStore = {
    inputFolder: string;
    outputFolder: string;
    resolution: Resolution | null;
    audioBitrate: number | null;
    videoBitrate: number | null;
    audioCodec: AudioCodec | null;
    videoCodec: VideoCodec | null;
    pictureFormat: PictureFormat | null;
    outputExtension: OutputExtension | null;
    needSorting: boolean;
}


export const [ioStore, setIoStore] = createStore<IoStore>({
    inputFolder: "",
    outputFolder: "",
    resolution: null,
    audioBitrate: null,
    videoBitrate: null,
    audioCodec: null,
    videoCodec: null,
    pictureFormat: null,
    outputExtension: null,
    needSorting: true,
});

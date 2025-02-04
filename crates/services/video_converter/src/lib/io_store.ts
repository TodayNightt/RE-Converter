
import { createStore } from "solid-js/store";
import type { AudioCodec, ConverterOptions, OutputExtension, PictureFormat, Resolution, VideoCodec } from "./types-backend";

export type IoStore = {
    inputFolder: string | null;
    outputFolder: string | null;
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

export function converterOptionsToIoStore(options: ConverterOptions): IoStore {
    return {
        audioCodec: options.ffmpegOptions.audioCodec.type === "custom"
            ? options.ffmpegOptions.audioCodec.content
            : null,
        videoCodec: options.ffmpegOptions.videoCodec.type === "custom"
            ? options.ffmpegOptions.videoCodec.content
            : null,
        resolution: options.ffmpegOptions.resolution.type === "custom"
            ? options.ffmpegOptions.resolution.content
            : null,
        pictureFormat: options.ffmpegOptions.pictureFormat.type === "custom"
            ? options.ffmpegOptions.pictureFormat.content
            : null,
        outputExtension: options.ffmpegOptions.outputExtension,
        audioBitrate: options.ffmpegOptions.audioBitrate.type === "custom"
            ? options.ffmpegOptions.audioBitrate.content
            : null,
        videoBitrate: options.ffmpegOptions.videoBitrate.type === "custom"
            ? options.ffmpegOptions.videoBitrate.content
            : null,
        inputFolder: options.inputDir,
        outputFolder: options.outputDir,
        needSorting: options.needSorting
    };
}
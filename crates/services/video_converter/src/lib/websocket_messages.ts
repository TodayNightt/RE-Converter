import { ioStore } from "./io_store";
import type WebSocket from "@tauri-apps/plugin-websocket";
import { Resolution, VideoCodec, AudioCodec, OutputExtension, type Message, type ConverterOptions, type ArgsType, type PictureFormat } from "./types-backend";

//region convert
export function convert(ws: WebSocket) {
    // Check resolution
    let r: ArgsType<Resolution> = { type: "matchSource" };
    if (ioStore.resolution) {
        r = {
            type: "custom",
            content: ioStore.resolution ?? Resolution.R1080P,
        };
    }

    // Check videoCodec
    let vc: ArgsType<VideoCodec> = { type: "matchSource" };
    if (ioStore.videoCodec) {
        // FIXME : This default to H264
        vc = { type: "custom", content: ioStore.videoCodec ?? VideoCodec.H264 };
    }

    // Check audioCodec
    let ac: ArgsType<AudioCodec> = { type: "matchSource" };
    if (ioStore.audioCodec) {
        // FIXME : This default to aac
        ac = { type: "custom", content: ioStore.audioCodec ?? AudioCodec.Aac };
    }

    // Check videoBitrate
    let vb: ArgsType<number> = { type: "matchSource" };
    if (ioStore.videoBitrate) {
        // FIXME : This default to 0
        vb = {
            type: "custom",
            content: ioStore.videoBitrate ? (ioStore.videoBitrate as number) : 0,
        };
    }

    // Check audioBitrate
    let ab: ArgsType<number> = { type: "matchSource" };
    if (ioStore.audioBitrate) {
        // FIXME : This default to 0
        ab = {
            type: "custom",
            content: ioStore.audioBitrate ? (ioStore.audioBitrate as number) : 0,
        };
    }

    // Check pictureFormat
    let pf: ArgsType<PictureFormat> = { type: "matchSource" };
    const pff = ioStore.pictureFormat;
    if (pff) {
        pf = {
            type: "custom",
            content: pff,
        };
    }

    const options: ConverterOptions = {
        inputDir: ioStore.inputFolder ?? "",
        outputDir: ioStore.outputFolder ?? "",
        needSorting: ioStore.needSorting,
        ffmpegOptions: {
            resolution: r,
            videoCodec: vc,
            audioCodec: ac,
            videoBitrate: vb,
            audioBitrate: ab,
            pictureFormat: pf,
            outputExtension: ioStore.outputExtension ?? OutputExtension.Default,
        },
    };


    const message: Message = {
        method: "convert",
        data: options,
    };

    ws.send({
        type: "Text",
        data: JSON.stringify(message),
    });

}
//endregion

//region cancel
export function cancel(ws: WebSocket) {
    const message: Message = {
        method: "cancel",
    };

    ws.send({
        type: "Text",
        data: JSON.stringify(message),
    });
};
//endregion

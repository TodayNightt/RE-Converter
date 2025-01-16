import { createEffect, createSignal, For } from "solid-js";
import {
  RadioGroup,
  RadioGroupItem,
  RadioGroupItemLabel,
} from "./ui/radio-group";
import SwitchToggle from "./switchToggle";
import { type ArgsType, VideoCodec } from "~/lib/types-backend";
import type { DefaultValueProps, ValueStoreSetter } from "~/lib/types";

type VideoCodecProps = DefaultValueProps<ArgsType<VideoCodec>> &
  ValueStoreSetter;

function VideoCodecComponent(props: VideoCodecProps) {
  const codecs = Object.values(VideoCodec).map(mapValue);
  const [switcher, setSwitcher] = createSignal<boolean>(
    !!(props.defaultValue && props.defaultValue.type === "custom")
  );

  const defaultValue = props.defaultValue;
  const videoCodec =
    defaultValue && defaultValue.type === "custom"
      ? defaultValue.content
      : null;
  const [value, setValue] = createSignal<VideoCodec | null>(videoCodec);

  createEffect(() => {
    if (!switcher()) {
      props.valueSetter(props.storeIdentifier, null);
      setValue(null);
    }
    props.valueSetter(props.storeIdentifier, reverseMapValue(value()));
  });
  return (
    <SwitchToggle
      reactiveSwitch={[switcher, setSwitcher]}
      additionalCss={"overflow-y-scroll"}
    >
      <RadioGroup
        class="flex gap-10"
        onChange={setValue}
        value={value()?.toString()}
      >
        <For each={codecs}>
          {(item) => (
            <RadioGroupItem value={item}>
              <RadioGroupItemLabel>{item}</RadioGroupItemLabel>
            </RadioGroupItem>
          )}
        </For>
      </RadioGroup>
    </SwitchToggle>
  );
}

export default VideoCodecComponent;

function mapValue(codec: VideoCodec): string {
  switch (codec) {
    case VideoCodec.H264:
      return "h264";
    case VideoCodec.H264NVENC:
      return "h264 (nvenc)";
    case VideoCodec.H264AMF:
      return "h264 (amf)";
    case VideoCodec.H264QSV:
      return "h264 (qsv)";
    case VideoCodec.H265:
      return "h265";
    case VideoCodec.H265NVENC:
      return "h265 (nvenc)";
    case VideoCodec.H265AMF:
      return "h265 (amf)";
    case VideoCodec.H265QSV:
      return "h265 (qsv)";
    case VideoCodec.CineForm:
      return "cineform";
    case VideoCodec.Prores:
      return "prores";
    default:
      throw null;
  }
}

function reverseMapValue(codecStr: string | null): VideoCodec | null {
  switch (codecStr) {
    case "h264":
      return VideoCodec.H264;
    case "h264 (nvenc)":
      return VideoCodec.H264NVENC;
    case "h264 (amf)":
      return VideoCodec.H264AMF;
    case "h264 (qsv)":
      return VideoCodec.H264QSV;
    case "h265":
      return VideoCodec.H265;
    case "h265 (nvenc)":
      return VideoCodec.H265NVENC;
    case "h265 (amf)":
      return VideoCodec.H265AMF;
    case "h265 (qsv)":
      return VideoCodec.H265QSV;
    case "cineform":
      return VideoCodec.CineForm;
    case "prores":
      return VideoCodec.Prores;
    default:
      return null;
  }
}

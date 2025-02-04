import { createEffect, createSignal } from "solid-js";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "./ui/select";
import { OutputExtension } from "~/lib/types-backend";
import type { DefaultValueProps, ValueStoreSetter } from "~/lib/types";
import { getValueFromLastSaved } from "~/lib/utils";

type OutputExtensionProps = DefaultValueProps & ValueStoreSetter;

function OutputExtensionComponent(props: OutputExtensionProps) {
  const extensions = Object.values(OutputExtension).map(mapValue);
  const [value, setValue] = createSignal<string | null>(
    mapValue(OutputExtension.Default)
  );

  createEffect(() => {
    props.valueSetter(props.storeIdentifier, reverseMapValue(value()));
  });

  createEffect(() => {
    if (props.redraw()) {
      const val = getValueFromLastSaved<OutputExtension>(
        props.defaultValueIden
      );

      if (val) {
        setValue(mapValue(val));
      }
    }
  });
  return (
    <Select
      defaultValue={mapValue(OutputExtension.Default)}
      options={extensions}
      value={value()}
      onChange={setValue}
      itemComponent={(props) => (
        <SelectItem item={props.item}>{props.item.rawValue}</SelectItem>
      )}
    >
      <SelectTrigger class="w-[180px] font-bold">
        <SelectValue<string>>{(state) => state.selectedOption()}</SelectValue>
      </SelectTrigger>
      <SelectContent />
    </Select>
  );
}

export default OutputExtensionComponent;

function mapValue(val: OutputExtension): string {
  switch (val) {
    case OutputExtension.Mkv:
      return "mkv";
    case OutputExtension.Default:
      return "Default (mkv)";
    case OutputExtension.Mp3:
      return "mp3";
    case OutputExtension.Mp4:
      return "mp4";
    case OutputExtension.Mov:
      return "mov";
  }
}

function reverseMapValue(resStr: string | null): OutputExtension | null {
  switch (resStr) {
    case "mkv":
      return OutputExtension.Mkv;
    case "Default (mkv)":
      return OutputExtension.Default;
    case "mp3":
      return OutputExtension.Mp3;
    case "mp4":
      return OutputExtension.Mp4;
    case "mov":
      return OutputExtension.Mov;
    default:
      return null;
  }
}

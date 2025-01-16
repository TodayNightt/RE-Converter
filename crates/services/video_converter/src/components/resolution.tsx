import { createEffect, createSignal, For } from "solid-js";
import SwitchToggle from "./switchToggle";
import { type ArgsType, Resolution } from "~/lib/types-backend";
import {
  RadioGroup,
  RadioGroupItem,
  RadioGroupItemLabel,
} from "./ui/radio-group";
import type { DefaultValueProps, ValueStoreSetter } from "~/lib/types";

type ResolutionProps = DefaultValueProps<ArgsType<Resolution>> &
  ValueStoreSetter;

function ResolutionComponent(props: ResolutionProps) {
  const resolutions = Object.values(Resolution).map(mapValue);
  const [switcher, setSwitcher] = createSignal<boolean>(
    !!(props.defaultValue && props.defaultValue.type === "custom")
  );

  const defaultValue = props.defaultValue;
  const resolution =
    defaultValue && defaultValue.type === "custom"
      ? defaultValue.content
      : null;
  const [value, setValue] = createSignal<Resolution | null>(resolution);

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
      additionalCss="flex gap-4"
    >
      <RadioGroup
        class="flex gap-10"
        onChange={setValue}
        value={value()?.toString()}
      >
        <For each={resolutions}>
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

export default ResolutionComponent;

function mapValue(val: Resolution): string {
  switch (val) {
    case Resolution.R1080P:
      return "1080p";
    case Resolution.R720P:
      return "720p";
    case Resolution.R1440P:
      return "1440p";
    case Resolution.R4K:
      return "4k";
  }
}

// export enum Resolution {
//   R720P = "r720p",
//   R1080P = "r1080p",
//   R1440P = "r1440p",
//   R4K = "r4k",
// }

function reverseMapValue(resStr: string | null): Resolution | null {
  switch (resStr) {
    case "1080p":
      return Resolution.R1080P;
    case "720p":
      return Resolution.R720P;
    case "1440p":
      return Resolution.R1440P;
    case "4k":
      return Resolution.R4K;
    default:
      return null;
  }
}

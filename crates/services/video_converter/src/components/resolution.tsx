import { createEffect, createSignal, For } from "solid-js";
import SwitchToggle from "./switchToggle";
import { Resolution } from "~/lib/types-backend";
import {
  RadioGroup,
  RadioGroupItem,
  RadioGroupItemLabel,
} from "./ui/radio-group";
import type { DefaultValueProps, ValueStoreSetter } from "~/lib/types";
import { getValueFromLastSaved } from "~/lib/utils";

type ResolutionProps = DefaultValueProps & ValueStoreSetter;

function ResolutionComponent(props: ResolutionProps) {
  const resolutions = Object.values(Resolution).map(mapValue);
  const [switcher, setSwitcher] = createSignal<boolean>(false);

  const [value, setValue] = createSignal<Resolution | null>(null);

  createEffect(() => {
    if (!switcher()) {
      props.valueSetter(props.storeIdentifier, null);
      setValue(null);
      return;
    }

    props.valueSetter(props.storeIdentifier, reverseMapValue(value()));
  });

  createEffect(() => {
    if (props.redraw()) {
      const val = getValueFromLastSaved<Resolution>(props.defaultValueIden);

      if (val) {
        setValue(val);
        setSwitcher(true);
      }
    }
  });

  return (
    <SwitchToggle
      reactiveSwitch={[switcher, setSwitcher]}
      additionalCss="flex gap-4"
    >
      <RadioGroup
        class="flex gap-10"
        onChange={setValue}
        value={mapValue(value() ?? Resolution.R1080P)}
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

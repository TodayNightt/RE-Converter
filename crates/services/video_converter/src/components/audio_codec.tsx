import { createEffect, createSignal, For } from "solid-js";
import SwitchToggle from "./switchToggle";
import {
  RadioGroup,
  RadioGroupItem,
  RadioGroupItemLabel,
} from "./ui/radio-group";
import { AudioCodec } from "~/lib/types-backend";
import type { DefaultValueProps, ValueStoreSetter } from "~/lib/types";
import { getValueFromLastSaved } from "~/lib/utils";

type AudioCodecProps = DefaultValueProps & ValueStoreSetter;

function AudioCodecComponent(props: AudioCodecProps) {
  const codecs = Object.values(AudioCodec);
  const [switcher, setSwitcher] = createSignal<boolean>(false);

  const [value, setValue] = createSignal<AudioCodec | null>(null);

  createEffect(() => {
    if (!switcher()) {
      props.valueSetter(props.storeIdentifier, null);
      setValue(null);
    }
    props.valueSetter(props.storeIdentifier, value());
  });

  createEffect(() => {
    if (props.redraw()) {
      const val = getValueFromLastSaved<AudioCodec>(props.defaultValueIden);
      if (val) {
        setSwitcher(true);
        setValue(val);
      }
    }
  });

  return (
    <SwitchToggle reactiveSwitch={[switcher, setSwitcher]}>
      <RadioGroup
        class="flex"
        defaultValue={value()?.toString()}
        onChange={setValue}
        value={value()?.toString()}
      >
        <For each={codecs}>
          {(fruit) => (
            <RadioGroupItem value={fruit}>
              <RadioGroupItemLabel class="text-sm">{fruit}</RadioGroupItemLabel>
            </RadioGroupItem>
          )}
        </For>
      </RadioGroup>
    </SwitchToggle>
  );
}

export default AudioCodecComponent;

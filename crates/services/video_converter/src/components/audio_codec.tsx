import { createEffect, createSignal, For } from "solid-js";
import SwitchToggle from "./switchToggle";
import {
  RadioGroup,
  RadioGroupItem,
  RadioGroupItemLabel,
} from "./ui/radio-group";
import { type ArgsType, AudioCodec } from "~/lib/types-backend";
import type { DefaultValueProps, ValueStoreSetter } from "~/lib/types";
import { updateStillDefault } from "~/lib/utils";

type AudioCodecProps = DefaultValueProps<ArgsType<AudioCodec>> &
  ValueStoreSetter;

function AudioCodecComponent(props: AudioCodecProps) {
  const codecs = Object.values(AudioCodec);
  const [switcher, setSwitcher] = createSignal<boolean>(
    !!(props.defaultValue && props.defaultValue.type === "custom")
  );

  const defaultValue = props.defaultValue;
  const audioCodec =
    defaultValue && defaultValue.type === "custom"
      ? defaultValue.content
      : null;
  const [value, setValue] = createSignal<AudioCodec | null>(audioCodec);

  createEffect(() => {
    updateStillDefault(
      props.stillDefault,
      props.defaultIndex,
      value(),
      defaultValue?.content ?? null
    );
    if (!switcher()) {
      props.valueSetter(props.storeIdentifier, null);
      setValue(null);
    }
    props.valueSetter(props.storeIdentifier, value());
  });

  return (
    <SwitchToggle reactiveSwitch={[switcher, setSwitcher]}>
      <RadioGroup
        class="flex"
        defaultValue={defaultValue ? defaultValue.toString() : undefined}
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

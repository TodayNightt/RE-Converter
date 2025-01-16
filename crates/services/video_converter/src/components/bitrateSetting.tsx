import { createEffect, createSignal } from "solid-js";
import SwitchToggle from "./switchToggle";
import { TextField, TextFieldInput, TextFieldLabel } from "./ui/text-field";
import type { DefaultValueProps, ValueStoreSetter } from "~/lib/types";
import type { ArgsType } from "~/lib/types-backend";
import { updateStillDefault } from "~/lib/utils";

type BitrateSettingProps = DefaultValueProps<ArgsType<number>> &
  ValueStoreSetter;

function BitrateSetting(props: BitrateSettingProps) {
  const [switcher, setSwitcher] = createSignal<boolean>(
    !!(props.defaultValue && props.defaultValue.type === "custom")
  );

  const defaultValue = props.defaultValue;
  const bitrate =
    defaultValue && defaultValue.type === "custom"
      ? defaultValue.content
      : null;
  const [value, setValue] = createSignal<string | null>(
    bitrate?.toString() ?? null
  );

  createEffect(() => {
    updateStillDefault(
      props.stillDefault,
      props.defaultIndex,
      value(),
      defaultValue?.content?.toString() ?? null
    );
    if (!switcher()) {
      props.valueSetter(props.storeIdentifier, null);
      setValue(null);
      return;
    }
    props.valueSetter(props.storeIdentifier, Number.parseInt(value() ?? "0"));
  });

  return (
    <SwitchToggle reactiveSwitch={[switcher, setSwitcher]}>
      <TextField
        defaultValue={
          (props.defaultValue ? props.defaultValue : null) as unknown as string
        }
        class="flex items-center gap-4"
        onChange={setValue}
      >
        <TextFieldInput
          id="bitrate"
          value={value() ?? undefined}
          type="number"
        />
        <TextFieldLabel class="text-sm">kbps</TextFieldLabel>
      </TextField>
    </SwitchToggle>
  );
}

export default BitrateSetting;

import { createEffect, createSignal } from "solid-js";
import SwitchToggle from "./switchToggle";
import { TextField, TextFieldInput, TextFieldLabel } from "./ui/text-field";
import type { DefaultValueProps, ValueStoreSetter } from "~/lib/types";
import { getValueFromLastSaved } from "~/lib/utils";

type BitrateSettingProps = DefaultValueProps & ValueStoreSetter;

function BitrateSetting(props: BitrateSettingProps) {
  const [switcher, setSwitcher] = createSignal<boolean>(false);

  const [value, setValue] = createSignal<string | null>(null);

  createEffect(() => {
    if (!switcher()) {
      props.valueSetter(props.storeIdentifier, null);
      setValue(null);
      return;
    }

    props.valueSetter(props.storeIdentifier, Number.parseInt(value() ?? "0"));
  });

  createEffect(() => {
    if (props.redraw()) {
      const val = getValueFromLastSaved<number>(props.defaultValueIden);

      if (val) {
        setSwitcher(true);
        setValue(val.toString());
      }
    }
  });

  return (
    <SwitchToggle reactiveSwitch={[switcher, setSwitcher]}>
      <TextField
        defaultValue={value() as string}
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

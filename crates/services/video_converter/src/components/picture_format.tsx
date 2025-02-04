import { createEffect, createSignal, For } from "solid-js";
import { PictureFormat } from "~/lib/types-backend";
import SwitchToggle from "./switchToggle";
import {
  RadioGroup,
  RadioGroupItem,
  RadioGroupItemLabel,
} from "./ui/radio-group";
import type { DefaultValueProps, ValueStoreSetter } from "~/lib/types";
import { getValueFromLastSaved } from "~/lib/utils";

type PictureFormatProps = DefaultValueProps & ValueStoreSetter;
function PictureFormatComponent(props: PictureFormatProps) {  
  const pictureFormats = Object.values(PictureFormat).map(mapValue);
  const [switcher, setSwitcher] = createSignal<boolean>(false);

  const [value, setValue] = createSignal<PictureFormat | null>(null);

  createEffect(() => {
    if (!switcher()) {
      props.valueSetter(props.storeIdentifier, null);
      setValue(null);
      return;
    }

    console.log("picture format", value());

    props.valueSetter(props.storeIdentifier, reverseMapValue(value()));
  });

  createEffect(() => {
    if (props.redraw()) {
      const val = getValueFromLastSaved<PictureFormat>(props.defaultValueIden);

      if (val) {
        console.log("picture format redraw");
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
        defaultValue={mapValue(PictureFormat.Pf4208B)}
        value={mapValue(value() ?? PictureFormat.Pf4208B)}
      >
        <For each={pictureFormats}>
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

export default PictureFormatComponent;

function mapValue(val: PictureFormat): string {
  switch (val) {
    case PictureFormat.Pf42010B:
      return "4:2:0 10bit";
    case PictureFormat.Pf4208B:
      return "4:2:0 8bit";
    case PictureFormat.Pf4228B:
      return "4:2:2 8bit";
    case PictureFormat.Pf42210B:
      return "4:2:2 10bit";
  }
}

function reverseMapValue(resStr: string | null): PictureFormat | null {
  switch (resStr) {
    case "4:2:0 10bit":
      return PictureFormat.Pf42010B;
    case "4:2:0 8bit":
      return PictureFormat.Pf4208B;
    case "4:2:2 8bit":
      return PictureFormat.Pf4228B;
    case "4:2:2 10bit":
      return PictureFormat.Pf42210B;
    default:
      return null;
  }
}

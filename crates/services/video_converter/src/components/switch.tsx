import type { Accessor, Setter } from "solid-js";
import { Switch, SwitchLabel, SwitchControl, SwitchThumb } from "./ui/switch";

type SwitchWProps = {
  reactiveSwitch: [Accessor<boolean>, Setter<boolean>];
};
function SwitchW(props: SwitchWProps) {
  return (
    <Switch
      class="flex items-center gap-4"
      onChange={props.reactiveSwitch[1]}
      checked={props.reactiveSwitch[0]()}
    >
      <SwitchLabel>Match Source</SwitchLabel>
      <SwitchControl>
        <SwitchThumb />
      </SwitchControl>
      <SwitchLabel>Custom</SwitchLabel>
    </Switch>
  );
}

export default SwitchW;

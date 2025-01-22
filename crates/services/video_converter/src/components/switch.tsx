import type { Accessor, Setter } from "solid-js";
import { Switch, SwitchLabel, SwitchControl, SwitchThumb } from "./ui/switch";
import { useAppState } from "~/appContext";

type SwitchWProps = {
  reactiveSwitch: [Accessor<boolean>, Setter<boolean>];
};
function SwitchW(props: SwitchWProps) {
  const { t } = useAppState();
  return (
    <Switch
      class="flex items-center gap-4"
      onChange={props.reactiveSwitch[1]}
      checked={props.reactiveSwitch[0]()}
    >
      <SwitchLabel>{t("matchSource")}</SwitchLabel>
      <SwitchControl>
        <SwitchThumb />
      </SwitchControl>
      <SwitchLabel>{t("custom")}</SwitchLabel>
    </Switch>
  );
}

export default SwitchW;

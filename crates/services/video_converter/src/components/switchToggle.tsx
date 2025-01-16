import { type Accessor, type FlowProps, type Setter, Show } from "solid-js";
import SwitchW from "./switch";

type SwitchToggleProps = FlowProps & {
  additionalCss?: string;
  reactiveSwitch: [Accessor<boolean>, Setter<boolean>];
};

function SwitchToggle(props: SwitchToggleProps) {
  return (
    <div class="flex flex-col gap-4">
      <SwitchW reactiveSwitch={props.reactiveSwitch} />
      <Show when={props.reactiveSwitch[0]()}>
        <div
          class={`w-full min-h-[30px] flex items-center ${props.additionalCss}`}
        >
          {props.children}
        </div>
      </Show>
    </div>
  );
}

export default SwitchToggle;

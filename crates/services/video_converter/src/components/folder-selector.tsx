import { createSignal } from "solid-js";
import { Label } from "./ui/label";
import { Button } from "./ui/button";
import { TextField, TextFieldInput } from "./ui/text-field";
import { open } from "@tauri-apps/plugin-dialog";
import { useAppState } from "~/appContext";
import type { DefaultValueProps, ValueStoreSetter } from "~/lib/types";

type FolderSelectorProps = DefaultValueProps<string> &
  ValueStoreSetter & {
    label: string;
  };

function FolderSelector(props: FolderSelectorProps) {
  const { t } = useAppState();
  const [value, setValue] = createSignal<string>(
    props.defaultValue ? props.defaultValue : ""
  );
  const valueSetter = async () => {
    try {
      const selectedFolder = await open({
        directory: true, // Set to allow folder selection
      });

      if (selectedFolder) {
        setValue(selectedFolder as string);
        props.valueSetter(props.storeIdentifier, value());
      }
    } catch (error) {
      console.error("Folder selection failed:", error);
    }
  };
  return (
    <div class="flex flex-col gap-4 p-4 bg-slate-300 rounded-lg">
      <Label class="text-lg font-bold">{props.label}</Label>
      <TextField value={value() as string} disabled>
        <TextFieldInput
          class="font-medium bg-slate-100 text-slate-900 text-ellipsis"
          type="text"
        />
      </TextField>
      <Button onClick={valueSetter} class="w-2/3 self-center">
        {t("selectFolder")}
      </Button>
    </div>
  );
}

export default FolderSelector;

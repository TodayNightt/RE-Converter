import "./App.css";
import MainComponent from "./main_component";
import { emit, once, type UnlistenFn } from "@tauri-apps/api/event";
import { createSignal, onCleanup, onMount } from "solid-js";

import {
  AlertDialog,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogTitle,
} from "~/components/ui/alert-dialog";
import { Button } from "./components/ui/button";
import { exit } from "@tauri-apps/plugin-process";

function App() {
  let unlisten: UnlistenFn;
  const [ffmpegNotInstalled, setFfmpegNotInstalled] =
    createSignal<boolean>(false);
  onMount(async () => {
    emit("tauri://window-created");
    unlisten = await once("no-ffmpeg", (_) => {
      console.log("ffmpeg not install");
      setFfmpegNotInstalled(true);
    });
  });

  onCleanup(() => {
    unlisten();
  });
  return (
    <>
      <MainComponent />
      <AlertDialog open={ffmpegNotInstalled()}>
        <AlertDialogContent>
          <AlertDialogTitle>Ffmpeg is not installed</AlertDialogTitle>
          <AlertDialogDescription>
            Ffmpeg is not installed in your pc. Kindly install it then open the
            application
            <br />
            <pre class="p-4 bg-slate-100 rounded-lg m-4">
              <h1 class="font-bold underline">Windows</h1>
              <br />
              <code>powershell &gt;&gt; winget install ffmpeg</code>
            </pre>
          </AlertDialogDescription>
          <Button
            onClick={async () => {
              await exit(1);
            }}
          >
            Close the application
          </Button>
        </AlertDialogContent>
      </AlertDialog>
    </>
  );
}

export default App;

import { FiChevronsRight, FiFilm, FiFolder } from "solid-icons/fi";
import {
  Accordion,
  AccordionContent,
  AccordionItem,
  AccordionTrigger,
} from "./components/ui/accordion";
import FolderSelector from "./components/folder-selector";
import type { Message, ProgressInfo } from "~/lib/types-backend";
import { Checkbox } from "~/components/ui/checkbox";
import WebSocket from "@tauri-apps/plugin-websocket";
import {
  createEffect,
  createMemo,
  createSignal,
  For,
  onCleanup,
  onMount,
  Show,
} from "solid-js";
import { CgArrowsExchangeAlt, CgSpinnerTwo } from "solid-icons/cg";
import { TbProgressCheck } from "solid-icons/tb";
import AudioCodecComponent from "./components/audio_codec";
import BitrateSetting from "./components/bitrateSetting";
import PictureFormatComponent from "./components/picture_format";
import ResolutionComponent from "./components/resolution";
import { Label } from "./components/ui/label";
import {
  Progress,
  ProgressLabel,
  ProgressValueLabel,
} from "./components/ui/progress";
import { Table, TableBody, TableRow, TableCell } from "./components/ui/table";
import VideoCodecComponent from "./components/video_codec";
import { Button } from "./components/ui/button";
import { useAppState } from "~/appContext";
import {
  RadioGroup,
  RadioGroupItem,
  RadioGroupItemLabel,
} from "./components/ui/radio-group";
import { type Locale, locales } from "~/lib/types";
import OutputExtensionComponent from "./components/output_extension";
import { showToast, Toaster } from "./components/ui/toast";
import { Badge } from "./components/ui/badge";
import { setIoStore } from "./lib/io_store";
import { cancel, convert } from "./lib/websocket_messages";
import { getLocaleFontClass } from "./lib/utils";

function MainComponent() {
  const ctx = useAppState();
  const { t, locale, setLocale } = ctx;
  let ws!: WebSocket;
  const [needSorting, setNeedSorting] = createSignal<boolean>(true);
  const [progress, updateProgress] = createSignal<ProgressInfo[]>([]);

  const [converting, setConverting] = createSignal<boolean>(false);
  const accordionPanel = createMemo(() =>
    converting() ? ["output"] : ["io-folder", "advanced-settings"]
  );

  const [stillDefaults, setStillDefaults] = createSignal<Array<boolean>>(
    Array(10).fill(true)
  );

  const stillDefault = createMemo(() => stillDefaults().every((v) => v));

  // Create memo for font class
  const fontClass = createMemo(() => getLocaleFontClass(locale()));

  onMount(async () => {
    try {
      ws = await WebSocket.connect("ws://127.0.0.1:8080").then((r) => {
        console.log("Connected");
        return r;
      });
    } catch (e) {}

    ws.addListener((msg) => {
      if (msg.type === "Text") {
        const data: Message = JSON.parse(msg.data);
        switch (data.method) {
          case "error":
            console.log(data);
            // console.error(data.data);
            showToast({
              title: "Error",
              description: data.data,
              variant: "error",
            });
            setConverting(false);
            break;
          case "acknowledge":
            setConverting(true);
            break;
          case "progress":
            console.log(data.data);
            updateProgress(data.data);
            break;
          case "cancelAcknowledge":
            setConverting(false);
            break;
        }
      }
    });
  });

  onCleanup(() => {
    if (ws) {
      cancel(ws);
      ws.disconnect()
        .then(() => {
          console.log("Disconnected");
        })
        .catch(console.log);
    }
  });

  return (
    <Show when={ctx.config?.getConfig()}>
      <main class={`container h-full ${fontClass()}`}>
        <div class="grid grid-rows-section-llm  grid-cols-3 h-full w-full">
          <nav class="flex justify-end items-center col-span-3 px-4">
            <RadioGroup
              class="flex justify-end w-full col-span-3 p-4"
              defaultValue={locale()}
              onChange={(val: string) => setLocale(val as Locale)}
            >
              <For each={locales}>
                {(locake: Locale) => (
                  <RadioGroupItem
                    value={locake}
                    class="flex justify-center items-center"
                  >
                    <RadioGroupItemLabel
                      class={`${getLocaleFontClass(
                        locake
                      )} flex justify-center items-center`}
                    >
                      {locake === "ja"
                        ? "日本語"
                        : locake === "zh-tw"
                        ? "中文"
                        : "English"}
                    </RadioGroupItemLabel>
                  </RadioGroupItem>
                )}
              </For>
            </RadioGroup>
            <Show when={stillDefault()}>
              <div class="flex justify-center items-center">
                <Badge round class="w-[6rem] h-6 justify-center p-4">
                  {t("lastSaved")}
                </Badge>
              </div>
            </Show>
          </nav>

          <div class="container h-full box-border col-span-3 row-span-2 overflow-y-scroll">
            <Accordion
              multiple={true}
              collapsible
              class="col-span-3 row-span-2 h-full p-4"
              defaultValue={accordionPanel()}
            >
              <AccordionItem value="io-folder" class="rounded-lg">
                <AccordionTrigger class="hover:no-underline font-bold text-xl justify-start gap-4 ui-expanded:animate-none">
                  <FiFolder />
                  {t("inputnOutputFolder")}
                </AccordionTrigger>
                <AccordionContent>
                  <div class="grid grid-cols-3 p-2">
                    <FolderSelector
                      valueSetter={setIoStore}
                      storeIdentifier={"inputFolder"}
                      label={t("inputFolder")}
                    />
                    <div class="flex justify-center items-center">
                      <FiChevronsRight class="size-40 animate-bounce" />
                    </div>
                    <FolderSelector
                      valueSetter={setIoStore}
                      storeIdentifier={"outputFolder"}
                      label={t("outputFolder")}
                    />
                  </div>
                </AccordionContent>
              </AccordionItem>
              <AccordionItem value="advanced-settings">
                <AccordionTrigger class="hover:no-underline justify-start gap-4 font-bold text-xl">
                  <FiFilm class="" />
                  {t("advancedSettings")}
                </AccordionTrigger>
                <AccordionContent>
                  {/* <div class="flex w-full gap-4 p-4"></div> */}
                  <Table>
                    <TableBody>
                      <TableRow>
                        <TableCell class="h-[150px] flex gap-5 items-center justify-center">
                          <Label class="w-[10rem] text-lg">
                            {t("needSorting")}
                          </Label>
                        </TableCell>
                        <TableCell>
                          <Checkbox
                            checked={needSorting()}
                            onChange={setNeedSorting}
                          />
                        </TableCell>
                        <TableCell class="h-[150px] flex gap-5 items-center justify-center">
                          <Label class="text-lg w-[10rem]">
                            {t("outputExtension")}
                          </Label>
                        </TableCell>
                        <TableCell>
                          <OutputExtensionComponent
                            valueSetter={setIoStore}
                            storeIdentifier={"outputExtension"}
                          />
                        </TableCell>
                      </TableRow>
                      <TableRow>
                        <TableCell class="w-[10rem] h-[150px] flex gap-5 items-center justify-center">
                          <Label class="text-lg">{t("resolution")}</Label>
                        </TableCell>
                        <TableCell class="w-full" colSpan={3}>
                          <ResolutionComponent
                            valueSetter={setIoStore}
                            storeIdentifier={"resolution"}
                          />
                        </TableCell>
                      </TableRow>
                      <TableRow>
                        <TableCell class="w-[10rem] h-[150px] flex flex-col gap-5 items-center justify-center">
                          <Label class="text-lg">{t("videoCodec")}</Label>
                        </TableCell>
                        <TableCell class="w-full" colSpan={3}>
                          <VideoCodecComponent
                            valueSetter={setIoStore}
                            storeIdentifier={"videoCodec"}
                          />
                        </TableCell>
                      </TableRow>
                      <TableRow>
                        <TableCell class="w-[10rem] h-[150px] flex flex-col gap-5 items-center justify-center">
                          <Label class="text-lg">{t("video")}</Label>
                        </TableCell>
                        <TableCell class="w-full" colSpan={3}>
                          <BitrateSetting
                            valueSetter={setIoStore}
                            storeIdentifier={"videoBitrate"}
                          />
                        </TableCell>
                      </TableRow>
                      <TableRow>
                        <TableCell class="w-[10rem] h-[150px] flex flex-col gap-5 items-center justify-center">
                          <Label class="text-lg">{t("audioCodec")}</Label>
                        </TableCell>
                        <TableCell class="w-full" colSpan={3}>
                          <AudioCodecComponent
                            valueSetter={setIoStore}
                            storeIdentifier={"audioCodec"}
                          />
                        </TableCell>
                      </TableRow>
                      <TableRow>
                        <TableCell class="w-[10rem] h-[150px] flex flex-col gap-5 items-center justify-center">
                          <Label class="text-lg">{t("audioBitrate")}</Label>
                        </TableCell>
                        <TableCell class="w-full" colSpan={3}>
                          <BitrateSetting
                            valueSetter={setIoStore}
                            storeIdentifier={"audioBitrate"}
                          />
                        </TableCell>
                      </TableRow>
                      <TableRow>
                        <TableCell class="w-[10rem] h-[150px] flex flex-col gap-5 items-center justify-center">
                          <Label class="text-lg">{t("pictureFormat")}</Label>
                        </TableCell>
                        <TableCell class="w-full" colSpan={3}>
                          <PictureFormatComponent
                            stillDefault={stillDefaults}
                            defaultIndex={9}
                            defaultValue={
                              ctx.config?.getConfig()?.last_saved?.ffmpegOptions
                                .pictureFormat
                            }
                            valueSetter={setIoStore}
                            storeIdentifier={"pictureFormat"}
                          />
                        </TableCell>
                      </TableRow>
                    </TableBody>
                  </Table>
                </AccordionContent>
              </AccordionItem>
              <AccordionItem value="output">
                <AccordionTrigger class="hover:no-underline justify-start gap-4 font-bold text-xl">
                  <TbProgressCheck />
                  {t("output")}
                </AccordionTrigger>
                <AccordionContent>
                  <div class="grid">
                    <For each={progress()}>
                      {({
                        totalProgress,
                        currentProgress,
                        fileName,
                        folderName,
                      }) => (
                        <Progress
                          value={currentProgress - 1}
                          minValue={0}
                          maxValue={totalProgress}
                          getValueLabel={({ value, max }) =>
                            `Currently on task ${value} of ${max}`
                          }
                          class="w-[300px] space-y-1 h-[5rem]"
                        >
                          <div class="flex justify-between ">
                            <ProgressLabel>{`Folder [${folderName}]`}</ProgressLabel>
                            <ProgressLabel>{`Currently converting ${fileName}`}</ProgressLabel>
                            <ProgressValueLabel />
                          </div>
                        </Progress>
                      )}
                    </For>
                  </div>
                </AccordionContent>
              </AccordionItem>
            </Accordion>
          </div>
          <div class="col-start-3 flex flex-col items-center justify-center p-4 lg:w-4/5">
            <Button
              class="w-full"
              onClick={() => convert(ws)}
              disabled={converting()}
            >
              <Show when={!converting()}>
                <div class="flex gap-4 justify-center items-center">
                  {t("convert")}
                  <CgArrowsExchangeAlt class="size-6" />
                </div>
              </Show>
              <Show when={converting()}>
                <CgSpinnerTwo class="animate-spin size-6" />
                {t("converting")}
              </Show>
            </Button>
            <Show when={converting()}>
              <Button
                class="underline bg-white text-black m-0 hover:bg-white h-fit p-4 size-2"
                onClick={() => cancel(ws)}
              >
                {t("cancel")}
              </Button>
            </Show>
          </div>
          <Toaster class="poppins" />
        </div>
      </main>
    </Show>
  );
}

export default MainComponent;

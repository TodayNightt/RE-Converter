import {
  createContext,
  useContext,
  createSignal,
  type ParentComponent,
  createResource,
  type ParentProps,
} from "solid-js";
import * as i18n from "@solid-primitives/i18n";
import type { Dict } from "./i18n/en";
import ConfigSingleton from "./lib/config_singleton";
import type { Locale, AppState, Dictionary } from "./lib/types";
import { setLastSavedStore } from "./lib/ls_store";

async function fetchDictionary(locale: Locale): Promise<Dictionary> {
  const dict: Dict = (await import(`./i18n/${locale}.ts`)).dict;
  return i18n.flatten(dict); // flatten the dictionary to make all nested keys available top-level
}

const AppContext = createContext<AppState>({} as AppState);

export const useAppState = () => useContext(AppContext);

export const AppContextProvider: ParentComponent = (props: ParentProps) => {
  const [locale, setLocale] = createSignal<Locale>("en");
  const [config, _setConfig] = createSignal<ConfigSingleton>(
    ConfigSingleton.getInstance()
  );

  const [dict] = createResource(locale, fetchDictionary);

  const t = i18n.translator(dict);

  setLastSavedStore(config().getConfig()?.last_saved);

  const state: AppState = {
    get locale() {
      return locale();
    },
    setLocale(value) {
      console.log(value);
      setLocale(value);
    },
    t: (key: string) => t(key) || "",
    get config() {
      return config();
    },
  };

  return (
    <AppContext.Provider value={state}>{props.children}</AppContext.Provider>
  );
};

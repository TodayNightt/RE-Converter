import type { Accessor, Setter } from "solid-js";
import type ConfigSingleton from "./config_singleton";
import type { Part, SetStoreFunction } from "solid-js/store";
import type { IoStore } from "./io_store";
import type { lastSavedStore } from "./ls_store";

export type Locale = "en" | "zh-tw" | "ja" | string;

export const locales: Locale[] = ["en", "zh-tw", "ja"];

export type Dictionary = {
    [key: string]: string;
};

export type AppState = {
    locale: Accessor<Locale>;
    setLocale: (value: Locale) => void;
    t: (key: string) => string;
    config: ConfigSingleton;
};

export type DefaultValueProps = {
    redraw: Accessor<boolean>;
    // defaultValueIden : keyof typeof lastSavedStore
    defaultValueIden: keyof typeof lastSavedStore;
    stillDefault?: Setter<Array<boolean>>;
    defaultIndex?: number;
}

export type ValueStoreSetter = {
    valueSetter: SetStoreFunction<IoStore>
    storeIdentifier: Part<IoStore, keyof IoStore>
}

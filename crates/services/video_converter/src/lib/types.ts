import type { Setter } from "solid-js";
import type ConfigSingleton from "./config_singleton";
import type { Part, SetStoreFunction } from "solid-js/store";
import type { IoStore } from "./io_store";

export type Locale = "en" | "zh-tw" | "ja" | string;

export const locales: Locale[] = ["en", "zh-tw", "ja"];

export type Dictionary = {
    [key: string]: string;
};

export type AppState = {
    locale: Locale;
    setLocale: (value: Locale) => void;
    t: (key: string) => string;
    config: ConfigSingleton | undefined;
};

export type DefaultValueProps<T> = {
    defaultValue?: T;
    stillDefault?: Setter<Array<boolean>>;
    defaultIndex?: number;
}

export type ValueStoreSetter = {
    valueSetter: SetStoreFunction<IoStore>
    storeIdentifier: Part<IoStore, keyof IoStore>
}

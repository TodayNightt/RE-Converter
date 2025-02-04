import { invoke } from "@tauri-apps/api/core";
import type { ConverterOptions } from "./types-backend";
import { type Accessor, createSignal, type Setter } from "solid-js";

class ConfigSingleton {
    private static instance: ConfigSingleton;
    private ready: [Accessor<boolean>, Setter<boolean>];
    private config: ConverterOptions | undefined;


    private constructor() {
        this.ready = createSignal(false);
        this.loadConfig().then(() => { this.ready[1](true) });
    }

    private async loadConfig() {
        this.config = await invoke<ConverterOptions>("get_last_saved");
    }

    public static getInstance(): ConfigSingleton {
        if (!ConfigSingleton.instance) {
            ConfigSingleton.instance = new ConfigSingleton();
        }
        return ConfigSingleton.instance;
    }

    public isReady(): Accessor<boolean> {
        return this.ready[0];
    }

    // Transform it to a Config when it's ready
    public getConfig(): ConverterOptions | null {
        if (!this.ready) {
            return null;
        }
        return this.config as ConverterOptions;
    }

    public updateConfig() {
        this.ready[1](false);
        this.loadConfig().then(() => {
            this.ready[1](true);
        });
    }
}

export default ConfigSingleton;
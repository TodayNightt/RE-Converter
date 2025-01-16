import { invoke } from "@tauri-apps/api/core";
import { createResource } from "solid-js";
import type { Config } from "./types-backend";

class ConfigSingleton {
    private static instance: ConfigSingleton;
    private configResource: ReturnType<typeof createResource<Config>>;

    private constructor() {
        this.configResource = createResource<Config>(this.loadConfig.bind(this));
    }

    private async loadConfig(): Promise<Config> {
        const config = await invoke<Config>("get_last_saved");
        return config;
    }

    public static getInstance(): ConfigSingleton {
        if (!ConfigSingleton.instance) {
            ConfigSingleton.instance = new ConfigSingleton();
        }
        return ConfigSingleton.instance;
    }

    public getConfig(): Config | undefined {
        return this.configResource[0]();
    }

    public updateConfig() {
        this.configResource[1].refetch();
    }
}

export default ConfigSingleton;
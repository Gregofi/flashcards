import type { config } from './types/config';
import { appConfigDir } from '@tauri-apps/api/path';
import { createDir, BaseDirectory, writeTextFile, readTextFile, exists } from '@tauri-apps/api/fs';

const CONFIG_FILE = 'config.json';

const DEFAULT_CONFIG: config = {
    algorithm: {
        name: 'Naive Exponential',
        description: 'Repeats questions every x days.',
        start: 2,
        cap: 128
    },
    syncedFolders: [],
    syncOnStartup: false,
    randomShuffle: false
};

/// Tries to fetch config from local file, if not exists
/// then a default config is created and saved to file.
/// Not used for now.
// eslint-disable-next-line @typescript-eslint/no-unused-vars
const getConfigFromFile = async (): Promise<config> => {
    const dir = await appConfigDir();
    await createDir(dir, {
        recursive: true
    });

    if (!(await exists(CONFIG_FILE, { dir: BaseDirectory.AppConfig }))) {
        await writeTextFile(CONFIG_FILE, JSON.stringify(DEFAULT_CONFIG), {
            dir: BaseDirectory.AppConfig
        });
    }

    const config = await readTextFile(CONFIG_FILE, {
        dir: BaseDirectory.AppConfig
    });

    return JSON.parse(config);
};

export const getConfig = (): config => {
    const config = localStorage.getItem('config');
    if (!config) {
        localStorage.setItem('config', JSON.stringify(DEFAULT_CONFIG));
        return DEFAULT_CONFIG;
    }
    try {
        const result = JSON.parse(config);
        return result;
    } catch (e) {
        localStorage.setItem('config', JSON.stringify(DEFAULT_CONFIG));
        return DEFAULT_CONFIG;
    }
};

export const saveConfig = async (config: config) => {
    localStorage.setItem('config', JSON.stringify(config));
};

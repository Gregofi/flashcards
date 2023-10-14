<script lang="ts">
    import { open } from '@tauri-apps/api/dialog';
    import { syncFolder } from '@api/commands';
    import { successToast, errorToast } from '@api/toasts';
    import { getConfig, saveConfig } from '@api/preferences';
    import { pendingSync } from '@stores/pendingSync';
    import { algorithmTypes } from '@api/types/algorithm';

    let cfg = getConfig();

    let loadingSync = false;
    pendingSync.subscribe((value) => {
        loadingSync = value;
    });

    const addFolder = () => {
        open({
            directory: true,
            multiple: false
        }).then((selected) => {
            if (selected === null) {
                return;
            }
            if (cfg.syncedFolders.includes(selected as string)) {
                errorToast('This folder is already in list');
                return;
            }
            cfg.syncedFolders = [...cfg.syncedFolders, selected as string];
            saveConfig(cfg);
        });
    };

    const removeFolder = (folder: string) => {
        cfg.syncedFolders = cfg.syncedFolders.filter((f) => f !== folder);
        saveConfig(cfg);
    };

    const sync = async () => {
        for (const folder of cfg.syncedFolders) {
            syncFolder(folder);
        }

        pendingSync.set(true);
        const promises = cfg.syncedFolders.map((folder) => syncFolder(folder));
        Promise.all(promises)
            .then(() => {
                successToast('Synchronization completed');
            })
            .catch((err) => {
                errorToast(`Synchronization failed: ${err}`);
            })
            .finally(() => {
                pendingSync.set(false);
            });
    };

    const updateShuffle = (event: Event) => {
        cfg.randomShuffle = (event.target as HTMLInputElement).checked;
        saveConfig(cfg);
    };

    const updateSync = (event: Event) => {
        cfg.syncOnStartup = (event.target as HTMLInputElement).checked;
        saveConfig(cfg);
    };
</script>

<div class="text-left">
    <div>Folders to synchronize cards from</div>
    <ul>
        {#each cfg.syncedFolders as folder}
            <li class="flex flex-row items-center">
                <button
                    class="m-1 ml-0 w-6
            h-6 bg-red-600 text-white flex items-center justify-center
            rounded"
                    on:click={removeFolder(folder)}>-</button
                ><span class="max-w-min"><code>{folder}</code></span>
            </li>
        {/each}
        <li class="flex flex-row items-center">
            <button
                class="m-2 ml-0 p-2 h-8 bg-blue-500 text-white flex items-center justify-center rounded"
                on:click={addFolder}>Add new folder</button
            >
            {#if loadingSync}
                <button
                    class="ml-4 bg-purple-300 text-white h-8 px-8 rounded hover:cursor-not-allowed"
                    disabled
                    on:click={sync}>Synchronize cards</button
                >
                <div
                    class="ml-4 border-gray-300 h-6 w-6 animate-spin rounded-full border-4 border-t-blue-600"
                />
            {:else}
                <button class="ml-4 bg-purple-800 text-white h-8 px-8 rounded" on:click={sync}
                    >Synchronize cards</button
                >
            {/if}
        </li>
    </ul>
</div>

<hr class="my-4" />
<div class="flex items-center mb-4">
    <input
        id="random-shuffle"
        type="checkbox"
        bind:checked={cfg.randomShuffle}
        class="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 rounded
           focus:ring-blue-500 dark:focus:ring-blue-600 dark:ring-offset-gray-800
           focus:ring-2 dark:bg-gray-700 dark:border-gray-600"
        on:change={updateShuffle}
    />
    <label
        for="random-shuffle"
        class="ml-2 text-sm font-medium text-gray-900
           dark:text-gray-300"
    >
        Shuffle cards randomly when starting a review.
    </label>
</div>
<div class="flex items-center mb-4">
    <input
        id="auto-sync"
        type="checkbox"
        bind:checked={cfg.syncOnStartup}
        class="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 rounded
           focus:ring-blue-500 dark:focus:ring-blue-600 dark:ring-offset-gray-800
           focus:ring-2 dark:bg-gray-700 dark:border-gray-600"
        on:change={updateSync}
    />
    <label
        for="auto-sync"
        class="ml-2 text-sm font-medium text-gray-900
           dark:text-gray-300"
    >
        Automatically synchronize cards on app startup.
    </label>
</div>
<hr class="my-4" />

<div class="flex flex-row">
    <label for="algorithm" class="mr-2">Spaced repetition algorithm: </label>

    <select name="algorithm">
        {#each algorithmTypes as alg}
            <option value={alg} selected={cfg.algorithm.name === alg}>{alg}</option>
        {/each}
    </select>
</div>

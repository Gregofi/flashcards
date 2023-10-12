<script lang="ts">
    import { open } from '@tauri-apps/api/dialog';
    import { syncFolder } from '@api/commands';
    import { toast } from '@zerodevx/svelte-toast';

    const sync = async () => {
        let selected = await open({
            directory: true,
            multiple: false
        });
        if (selected === null) {
            return;
        }

        syncFolder(selected as string)
            .then(() => {
                toast.push('Synchronized successfully', {
                    theme: {
                        '--toastBackground': '#4caf50',
                        '--toastProgressBackground': '#81c784',
                        '--toastColor': '#fff'
                    }
                });
            })
            .catch((err) => {
                toast.push(`Synchronization failed: ${err}`, {
                    theme: {
                        '--toastBackground': '#f44336',
                        '--toastProgressBackground': '#e57373',
                        '--toastColor': '#fff'
                    }
                });
            });
    };
</script>

<div class="text-left">
    <button
        class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 border border-blue-700 rounded"
        on:click={sync}>Synchronize flashcards from folder</button
    >
</div>

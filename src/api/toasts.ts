import { toast } from '@zerodevx/svelte-toast';

export const successToast = (message: string) => {
    toast.push(message, {
        theme: {
            '--toastBackground': '#4caf50',
            '--toastProgressBackground': '#81c784',
            '--toastColor': '#fff'
        }
    });
};

export const errorToast = (message: string) => {
    toast.push(message, {
        theme: {
            '--toastBackground': '#f44336',
            '--toastProgressBackground': '#e57373',
            '--toastColor': '#fff'
        }
    });
};

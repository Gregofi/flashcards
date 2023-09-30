import { open } from '@tauri-apps/api/dialog';
import { syncFolder } from './commands';

const sync = async () => {
  let selected = await open({
    directory: true,
  });
  if (selected === null) {
    return;
  }

  await syncFolder(selected as string);
}

window.addEventListener('DOMContentLoaded', () => {
  document.querySelector('#sync-dir')?.addEventListener('click', () => {
    console.log("Hi");
    // TODO: Show the messages in the UI.
    sync().then(() => {
      console.log("Sync completed!");
    }).catch((err) => {
      console.log("Sync failed!", err);
    });
  });
});

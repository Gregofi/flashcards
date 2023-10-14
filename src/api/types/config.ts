import type { Algorithm } from './algorithm';

export interface config {
    /// The algorithm to use for spaced repetition.
    algorithm: Algorithm;
    /// The folders to synchronize.
    syncedFolders: string[];
    /// Synchronize on app start.
    syncOnStartup: boolean;
    /// Shuffle the cards before reviewing.
    randomShuffle: boolean;
}

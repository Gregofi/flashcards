import { invoke } from '@tauri-apps/api/tauri';
import type { Card } from './types/card';

export const getCard = async (id: number): Promise<Card | null> => invoke('get_card', { id });

export async function getCards(): Promise<Card[]> {
    const cardsJson = (await invoke('get_all_cards')) as Card[];
    return cardsJson;
}

export const getCardsToReview = async (shuffle: boolean) =>
    invoke('get_cards_to_review', { shuffle }) as Promise<Card[]>;

export const saveAnswer = async (flashcardId: number, answerRating: number) =>
    invoke('answer_question', { flashcardId, answerRating });

export const syncFolder = async (folder: string) => invoke('sync_flashcards', { folder });

import { invoke } from "@tauri-apps/api/tauri";
import { Card } from "./types/card";

export async function getCards(): Promise<Card[]> {
  const cardsJson = await invoke("getAllCards") as Card[];
  return cardsJson;
}

export const getCardsToReview = async () => invoke("get_cards_to_review") as Promise<Card[]>;

export const saveAnswer = async (flashcardId: number, answerRating: number) =>
  invoke("answer_question", { flashcardId, answerRating });

export const syncFolder = async (folder: string) => invoke("sync_flashcards", { folder });

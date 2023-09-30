import { Card } from './types/card.ts';
import { getCardsToReview, saveAnswer } from './commands';

let toReview: Card[] = [];
let flipped = false;

enum AnswerKind {
  BEST = "BEST",
  GOOD = "GOOD",
  BAD = "BAD",
  WORST = "WORST",
}

const getBtnForAnswer = (answer: AnswerKind) => {
  switch (answer) {
    case AnswerKind.BEST:
      return "btn-answer-3";
    case AnswerKind.GOOD:
      return "btn-answer-2";
    case AnswerKind.BAD:
      return "btn-answer-1";
    case AnswerKind.WORST:
      return "btn-answer-0";
  }
}

const getAnswerRating = (answer: AnswerKind): number => {
  switch (answer) {
    case AnswerKind.BEST: return 100;
    case AnswerKind.GOOD: return 66;
    case AnswerKind.BAD: return 33;
    case AnswerKind.WORST: return 0;
  }
}

const renderCurrentCard = () => {
  let cardText = document.querySelector("#card-text")!;
  let card = toReview[0];
  if (flipped) {
    cardText.textContent = card.answer;
  } else {
    cardText.textContent = card.question;
  }
}

window.addEventListener("DOMContentLoaded", () => {
  getCardsToReview().then((cards) => {
    toReview = cards;
    renderCurrentCard();
  });

  let flipButton = document.querySelector("#btn-flip")!;
  flipButton.addEventListener("click", () => {
    flipped = !flipped;
    renderCurrentCard();
  });

  Object.keys(AnswerKind).forEach((answer) => {
    const btn = document.querySelector(`#${getBtnForAnswer(answer as AnswerKind)}`)!;
    btn.addEventListener("click", () => {
      const currentCardId = toReview[0].id;
      const rating = getAnswerRating(answer as AnswerKind);
      saveAnswer(currentCardId, rating).then(() => {
        console.log("Answer saved successfully");
      }).catch((e) => {
        console.error("Error saving answer", e);
      });

      toReview.shift();
      if (toReview.length === 0) {
        // TODO: Show some text like "You're done for today!"
        window.location.href = "index.html";
      }

      flipped = false;
      renderCurrentCard();
    });
  });
});

import { getCardsToReview } from './commands';

window.addEventListener("DOMContentLoaded", () => {
  getCardsToReview().then((cards) => {
    let p = document.querySelector("#loaded-cards")
    console.log(cards);
    if (p !== null) {
      p.textContent = `${cards.length.toString()} cards to review today`;
    }
  });
});

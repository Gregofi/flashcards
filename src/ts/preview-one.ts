import { Card } from "./types/card";
import { getCard } from "./commands";
import "./mathjax";

const renderCard = (card: Card): HTMLDivElement => {
  const cardContainer = document.createElement("div");
  cardContainer.id = "card-container";
  
  const cardInfo = `
<span><strong>Question text</strong></span>
<p>${card.question}</p>
<span><strong>Answer text</strong></span>
<p>${card.answer}</p>
`;

  cardContainer.innerHTML = cardInfo;
  return cardContainer;
};

const urlParams = new URLSearchParams(window.location.search);
const flashcardId = urlParams.get("flashcard_id");

(() => {
  if (flashcardId) {
    const id = parseInt(flashcardId);
    if (!id) {
      console.log("Couldn't parse ID", flashcardId);
      return;
    }
    const card = getCard(id);
    card.then((card) => {
      if (!card) {
        console.log("Couldn't find card with given ID");
        return;
      }

      const container = document.getElementById("card")!;
      container.appendChild(renderCard(card));
    });
  }
})();

import { Card } from "./types/card";
import { getCards } from './commands';
import './mathjax';

const createElement = (element: string, className: string, text?: string): HTMLSpanElement => {
  const cardText = document.createElement(element);
  cardText.classList.add(className);
  if (text) {
    cardText.innerText = text;
  }
  return cardText;
};

const renderCards = (cards: Card[]): HTMLDivElement => {
  const cardContainer = createElement("div", "card-container") as HTMLDivElement;
  const cardDivs = cards.reduce((acc, card) => {
    const cardDiv = createElement("div", "card-one");

    const cardId = createElement("span", "card-id");
    const link = createElement("a", "card-link", card.id.toString()) as HTMLLinkElement;
    link.href = `preview-one.html?flashcard_id=${card.id}`;
    cardId.appendChild(link);
    cardDiv.appendChild(cardId);

    cardDiv.appendChild(createElement("span", "card-text", card.question));

    acc.appendChild(cardDiv);
    return acc;
  }, cardContainer);
  return cardDivs;
};

getCards().then((cards) => {
  console.log("Here");
  const cardsContainer = renderCards(cards);
  const container = document.getElementById("cards")!;
  container.appendChild(cardsContainer);
  window.MathJax.typesetPromise();
});

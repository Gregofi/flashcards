<script lang="ts">
    import { getCardsToReview, saveAnswer } from '@api/commands';
    import { HtmlToMarkdown } from '@api/markdown';
    import type { Card } from '@api/types/card';
    import '@api/mathjax';

    let cards: Card[] | null;
    $: cards = null;
    $: flipped = false;
    getCardsToReview().then((cards_) => {
        cards = cards_.toReversed();
        setTimeout(window.MathJax.typeset, 0);
    });

    const updateState = (cards: Card[], score: number) => {
        const last_card = cards.pop();
        if (!last_card) {
            return [];
        }
        saveAnswer(last_card.id, score);

        flipped = false;

        return cards;
    };
    const flip = () => {
        flipped = !flipped;
        // Run after DOM settles.
        setTimeout(window.MathJax.typeset, 0);
    };
</script>

<div class="review-container">
    <div class="review-main">
        <h1>Review</h1>
        {#if cards !== null}
            {#if cards.length === 0}
                <div>Nothing to review</div>
            {:else if !flipped}
                <div class="question">{@html HtmlToMarkdown(cards.slice(-1)[0].question)}</div>
            {:else}
                <div class="answer">{@html HtmlToMarkdown(cards.slice(-1)[0].answer)}</div>
            {/if}
        {/if}
    </div>
    <div>
        {#if cards !== null}
            <div class="answer-buttons">
                <button
                    class="btn-answer"
                    on:click={() => {
                        cards = updateState(cards ?? [], 100);
                    }}>Good</button
                >
                <button
                    class="btn-answer"
                    on:click={() => {
                        cards = updateState(cards ?? [], 66);
                    }}>Decent</button
                >
                <button
                    class="btn-answer"
                    on:click={() => {
                        cards = updateState(cards ?? [], 33);
                    }}>Bad</button
                >
                <button
                    class="btn-answer"
                    on:click={() => {
                        cards = updateState(cards ?? [], 0);
                    }}>Worst</button
                >
            </div>
        {/if}
        <button id="btn-flip" on:click={flip}>Flip</button>
        <div>
            <a href="/">Back to home</a>
        </div>
    </div>
</div>

<style>
    .review-container {
        margin: 0;
        padding-top: 10vh;
        display: flex;
        flex-direction: column;
        justify-content: space-between;
        text-align: center;
        height: -webkit-fill-available;
    }

    .answer-buttons {
        margin: 0 auto;
        display: flex;
        flex-direction: row;
        flex-grow: 1;
        justify-content: center;
    }

    .question,
    .answer {
        text-align: justify;
    }

    .btn-answer {
        margin: 2px;
    }

    #btn-flip {
        margin: 1em auto;
        width: 30%;
    }

    button {
        border: 1px solid transparent;
        padding: 0.6em 1.2em;
        font-size: 1em;
        font-weight: 500;
        font-family: inherit;
        color: #0f0f0f;
        background-color: #ffffff;
        transition: border-color 0.25s;
        box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
    }

    button {
        cursor: pointer;
    }

    button:hover {
        border-color: #396cd8;
    }
    button:active {
        border-color: #396cd8;
        background-color: #e8e8e8;
    }

    button {
        outline: none;
    }
</style>

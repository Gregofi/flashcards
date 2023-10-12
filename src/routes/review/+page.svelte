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

        setTimeout(window.MathJax.typeset, 0);
        return cards;
    };
    const flip = () => {
        flipped = !flipped;
        // Run after DOM settles.
        setTimeout(window.MathJax.typeset, 0);
    };

    // TODO: Lift buttons into separate component.
    const buttons = [
        { score: 100, text: 'Good', style: 'from-green-700 via-green-600 to-green-600' },
        { score: 66, text: 'Decent', style: 'from-green-600 via-lime-600 to-lime-600' },
        { score: 33, text: 'Bad', style: 'from-amber-400 via-amber-500 to-amber-600' },
        { score: 0, text: 'Worst', style: 'from-red-600 via-red-700 to-red-800' }
    ];
</script>

<div class="review-container">
    <div class="review-main">
        <h1 class="text-3xl mb-4">Review</h1>
        {#if cards !== null}
            {#if cards.length === 0}
                <div>Nothing to review</div>
            {:else if !flipped}
                <div class="text-justify">{@html HtmlToMarkdown(cards.slice(-1)[0].question)}</div>
            {:else}
                <div class="text-justify">{@html HtmlToMarkdown(cards.slice(-1)[0].answer)}</div>
            {/if}
        {/if}
    </div>
    <div class="bottom-container mx-auto mt-24">
        {#if cards !== null}
            <div class="mx-auto flex flex-row flex-grow justify-center">
                {#each buttons as { score, text, style }}
                    <button
                        class="m-1 w-32 h-10 text-white bg-gradient-to-r {style} hover:scale-110 transition"
                        on:click={() => {
                            cards = updateState(cards ?? [], score);
                        }}>{text}</button
                    >
                {/each}
            </div>
            <button
                class="mt-5 w-52 h-12 m-1 mx-auto text-white bg-blue-500 font-semibold text-lg hover:scale-110 transition"
                on:click={flip}>Flip card</button
            >
        {/if}
    </div>
</div>

<style>
</style>

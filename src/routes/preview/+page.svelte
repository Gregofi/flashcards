<script lang="ts">
    import { getCards } from '@api/commands';
    import { onMount } from 'svelte';
    import type { Card } from '@api/types/card';
    import Showdown from 'showdown';
    import '@api/mathjax';

    let cards: Card[] = [];
    const converter = new Showdown.Converter();

    onMount(async () => {
        // We want to get cards before
        cards = await getCards();
        setTimeout(window.MathJax.typeset, 0);
    });
</script>

<h1>Card Preview</h1>
<div class="cards-container">
    {#each cards as card}
        <div class="card-container">
            <div class="card-id"><p><a href="/preview/{card.id}">{card.id}</a></p></div>
            <div class="card-text">{@html converter.makeHtml(card.question)}</div>
        </div>
    {/each}
</div>

<style>
    .cards-container {
        text-align: left;
    }

    .card-container {
        display: flex;
        flex-direction: row;
        border: 1px solid gray;
    }

    .card-id {
        border-right: 1px solid gray;
        width: 50px;
        text-align: center;
    }

    .card-text {
        margin-left: 10px;
    }
</style>

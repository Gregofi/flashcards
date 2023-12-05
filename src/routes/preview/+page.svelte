<script lang="ts">
    import { getCards } from '@api/commands';
    import { onMount } from 'svelte';
    import type { Card } from '@api/types/card';
    import katex from 'katex';
    import { markdownToHtml } from '@api/markdown';

    let cards: Card[] = [];

    onMount(async () => {
        // We want to get cards before
        cards = await getCards();
        setTimeout(katex.renderMathInElement(document.body), 0);
    });
</script>

<div>
    <table class="w-full">
        <thead class="uppercase text-sm">
            <tr>
                <th class="px-6 py-3">id</th>
                <th class="px-6 py-3">question</th>
            </tr>
        </thead>
        <tbody>
            {#each cards as card}
                <tr
                    class="border-b h-8 min-h-full hover:bg-gray-50 hover:cursor-pointer"
                    on:click={(window.location = `/preview/${card.id}`)}
                >
                    <td>{card.id}</td>
                    <td class="text-left">{@html markdownToHtml(card.question)}</td>
                </tr>
            {/each}
        </tbody>
    </table>
</div>

<style>
</style>

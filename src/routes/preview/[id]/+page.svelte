<script lang="ts">
    import { page } from '$app/stores';
    import { getCard } from '@api/commands';
    import { markdownToHtml } from '@api/markdown';

    const id = parseInt($page.params.id);
    const card = getCard(id);
    setTimeout(window.MathJax.typeset, 0);
</script>

{#await card}
    <p>loading...</p>
{:then card}
    <div class="card-container">
        <p class="text-sm">Card at <code>{card.path}</code></p>
        <h2 class="text-3xl mt-8">Question</h2>
        <div class="text-justify">{@html markdownToHtml(card.question)}</div>
        <h2 class="text-3xl mt-3">Answer</h2>
        <div class="text-justify">{@html markdownToHtml(card.answer)}</div>
    </div>
    <a href="/preview"
        ><div class="w-48 h-8 bg-blue-600 mx-auto text-white text-lg mt-10">go back</div></a
    >
{:catch error}
    <p style="color: red">{error.message}</p>
{/await}

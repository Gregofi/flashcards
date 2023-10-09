<script lang="ts">
    import { page } from '$app/stores';
    import { getCard } from '@api/commands';
    import { HtmlToMarkdown } from '@api/markdown';

    const id = parseInt($page.params.id);
    const card = getCard(id);
    setTimeout(window.MathJax.typeset, 0);
</script>

{#await card}
    <p>loading...</p>
{:then card}
    <div class="card-container">
        <p>Card at <code>{card.path}</code></p>
        <h2>Question</h2>
        <p>{@html HtmlToMarkdown(card.question)}</p>
        <h2>Answer</h2>
        <p>{@html HtmlToMarkdown(card.answer)}</p>
    </div>
    <a href="/preview">go back</a>
{:catch error}
    <p style="color: red">{error.message}</p>
{/await}

<style>
    .card-container {
        text-align: justify;
    }
</style>

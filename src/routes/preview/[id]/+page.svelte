<script lang="ts">
    import { page } from '$app/stores';
    import { getCard } from '@api/commands';
    import Showdown from 'showdown';

    const converter = new Showdown.Converter();
    const id = parseInt($page.params.id);
    const card = getCard(id);
    setTimeout(window.MathJax.typeset, 0);
</script>

{#await card}
    <p>loading...</p>
{:then card}
    <div class="card-container">
        <h2>Question</h2>
        <p>{@html converter.makeHtml(card.question)}</p>
        <h2>Answer</h2>
        <p>{@html converter.makeHtml(card.answer)}</p>
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

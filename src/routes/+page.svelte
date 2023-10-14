<script lang="ts">
    import { getCardsToReview } from '@api/commands';

    let cards = [];
    getCardsToReview(false).then((_cards) => {
        cards = _cards;
    });
</script>

<div>
    <h1 class="text-3xl font-bold">Flashcards</h1>
    <p>{cards.length} cards to review</p>
    <div>
        <a href="/review">
            <div
                class="bg-blue-500 mx-auto mt-5 w-48 hover:bg-blue-700 text-white font-bold py-2 px-4 border border-blue-700 rounded"
            >
                Review cards
            </div></a
        >
    </div>
</div>

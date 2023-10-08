<script lang="ts">
    import { getCardsToReview } from '@api/commands';
    
    let cards = [];
    getCardsToReview().then((_cards) => {
        cards = _cards;
    });
</script>

<div>
    <h1>Flashcards</h1>
    <p>{ cards.length } cards loaded</p>
    <div>
        <a href="/review">Review cards</a>
    </div>
    <div>
        <a href="/preview">Card preview</a>
    </div>
</div>

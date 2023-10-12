<script lang="ts">
    import { SvelteToast } from '@zerodevx/svelte-toast';

    const options = {};
</script>

<nav>
    <ul>
        <li>
            <a href="/">Home</a>
        </li>
        <li>
            <a href="/settings">Settings</a>
        </li>
    </ul>
</nav>

<style>
    .container {
        margin: 0 auto;
        padding-top: 10vh;
        width: 700px;
        display: flex;
        flex-direction: column;
        text-align: center;
    }
</style>

<div class="container">
    <slot />
</div>

<SvelteToast {options} />


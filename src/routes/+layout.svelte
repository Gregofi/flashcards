<script lang="ts">
    import '../app.css';
    import '../katex.css';
    import { SvelteToast } from '@zerodevx/svelte-toast';
    const options = {};
</script>

<nav>
    <ul class="border-b flex flex-row text-xl">
        <li class="w-32 h-12 flex justify-center items-center">
            <a class="inline-block text-blue-500" href="/">Home</a>
        </li>
        <li class="w-32 h-12 flex justify-center items-center">
            <a class="inline-block text-blue-500" href="/review">Review</a>
        </li>
        <li class="w-32 h-12 flex justify-center items-center">
            <a class="inline-block text-blue-500" href="/preview">Preview</a>
        </li>
        <li class="w-32 h-12 flex justify-center items-center">
            <a class="inline-block text-blue-500" href="/settings">Settings</a>
        </li>
    </ul>
</nav>

<div class="flex mx-auto w-700 text-center flex-col mt-16">
    <slot />
</div>

<SvelteToast {options} />

<style>
    .w-700 {
        width: 700px;
    }
</style>

<script>
  import { navigating } from "$app/stores";
  import Nav from "$lib/Nav.svelte";
  import PreloadingIndicator from "$lib/PreloadingIndicator.svelte";
</script>

{#if $navigating}
  <PreloadingIndicator />
{/if}

<Nav />

<div class="container">
<main>
  <slot />
</main>
</div>
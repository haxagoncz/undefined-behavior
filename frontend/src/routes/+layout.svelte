<script lang="ts">
import '@unocss/reset/tailwind.css'
import 'uno.css'
import Header from "$lib/Header.svelte";
</script>


<div class="min-h-screen w-full bg-gray-700 text-white">
  <Header />

  <main class="flex flex-col gap-2 px-4 py-4 md:px-32 md:py-8">
    <slot />
  </main>
</div>

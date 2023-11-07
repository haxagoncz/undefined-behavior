<script lang="ts">
  import type { PageData } from "./$types"


  export let data: PageData

</script>

<h2 class="font-bold text-2xl">About</h2>
<p>We are a cat cafe located at Preslova 72/25, 150 21 Praha 5-Smíchov.</p>
<p>Open from 8 AM till 5 PM every Monday - Friday. You can make a reservation on +420 693 042 333 or admin@myapp.io</p>

<div class="w-full flex flex-col items-center pt-8"><img src="/owner.webp" alt="Owner-chan" class="h-100 w-100"/></div>

<h4 class="font-bold text-xl">Web info</h4>
<p>NotAWordpressApp { data.version }-{ data.sha }</p>

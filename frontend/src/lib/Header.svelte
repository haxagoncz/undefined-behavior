<script>
  import { page } from "$app/stores";

</script>
<header class="flex flex-col gap-2 py-2 sm:flex-row">
  <a href="/" class="text-4xl uppercase font-bold inline-flex items-center gap-2 px-2 group">
    Neko <span class="i-bx:coffee inline-block group-hover:text-yellow-800 transition-colors duration-300"></span>
  </a>

  <nav class="flex flex-col w-full items-center px-2 gap-x-4 bg-gray-600 sm:bg-transparent sm:flex-row sm:justify-end">
    <a href="/" class="
      uppercase font-bold text-lg
      {$page.url.pathname === '/' ? 'text-yellow-800' : ''}
      hover:text-yellow-500 duration-300 transition-colors
    ">
      Home
    </a>
    <a href="/about" class="
      uppercase font-bold text-lg
      {$page.url.pathname === '/about' ? 'text-yellow-800' : ''}
      hover:text-yellow-500 duration-300 transition-colors
    ">
      About
    </a>
    <a href="/admin" class="
      uppercase font-bold text-lg
      {$page.url.pathname === '/admin' ? 'text-yellow-800' : ''}
      hover:text-yellow-500 duration-300 transition-colors
    ">
      admin
    </a>
  </nav>
</header>
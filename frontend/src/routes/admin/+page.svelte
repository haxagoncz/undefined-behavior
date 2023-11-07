<script lang="ts">

  let loading = false
  let email = ''
  let password = ''
  let error = ''

  async function login() {
    if (loading) return
    loading = true
    const result = await fetch('/api/login', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify({ email, password }),
    })
    loading = false

    password = ''
    if (result.ok) {
      error = await result.text()
    } else {
      error = 'Login failed'
    }
  }

</script>

<form on:submit|preventDefault={login} class="flex flex-col gap-2 w-75 text-black">
  <input bind:value={email} placeholder="Email">
  <input bind:value={password} placeholder="Password">
  <p class="text-red">{ error }</p>
  <button class="mt-4 bg-white text-black">
    Login
  </button>
</form>

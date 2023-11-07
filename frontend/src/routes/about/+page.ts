import type { PageLoad } from "./$types";

export const load: PageLoad = async ({ fetch }) => {
  const result = await fetch('/api/config')

  return (await result.json()) as { repository: string, sha: string, version: string }
}

export const ssr = false
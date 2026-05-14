<script lang="ts">
    import NavBar from "$lib/components/NavBar.svelte";
    import { goto } from "$app/navigation";
    import { page, navigating } from "$app/stores";
    import { cancel } from "@tauri-apps/plugin-barcode-scanner";

    import { user, checkAuth } from "$lib/auth.svelte";

    let { children } = $props();

    let checking = $state(true);

    $effect(() => {
        checkAuth();
        if (!user.user) {
            const redirectTo = $page.url.pathname + $page.url.search;
            goto(`/login?redirectTo=${encodeURIComponent(redirectTo)}`, {
                replaceState: true,
            });
        } else {
            checking = false;
        }
    });

    $effect(() => {
        if ($navigating) {
            cancel();
        }
    });
</script>

{#if checking}
    <div>Loading...</div>
{:else if user.user}
    {@render children()}
    <NavBar />
{:else}
    <p>Go back to login</p>
{/if}

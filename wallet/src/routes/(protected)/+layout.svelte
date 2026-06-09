<script lang="ts">
    import NavBar from "$lib/components/NavBar.svelte";
    import { goto } from "$app/navigation";
    import { page, navigating } from "$app/stores";
    import { cancel } from "@tauri-apps/plugin-barcode-scanner";

    import { user, checkAuth } from "$lib/auth.svelte";

    let { children } = $props();

    let checking = $state(true);

    import { onMount } from 'svelte';

    onMount(async () => {
        await checkAuth();
        checking = false;
    });

    $effect(() => {
        if (!checking && !user.user) {
            const redirectTo = $page.url.pathname + $page.url.search;
            goto(`/login?redirectTo=${encodeURIComponent(redirectTo)}`, {
                replaceState: true,
            });
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
    <div style="padding-bottom: 100px; min-height: 100vh;">
        {@render children()}
    </div>
    <NavBar />
{:else}
    <p>Go back to login</p>
{/if}

<script lang="ts">
    import { goto } from "$app/navigation";

    import { user, checkAuth } from "$lib/auth.svelte";

    let { children } = $props();

    let checking = $state(true);

    import { onMount } from 'svelte';

    onMount(async () => {
        await checkAuth();
        checking = false;
    });

    $effect(() => {
        if (!checking && user.user) {
            goto("/scanner/home", {
                replaceState: true,
            });
        }
    });
</script>

{@render children()}

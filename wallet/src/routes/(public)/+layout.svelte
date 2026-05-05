<script lang="ts">
    import { goto } from "$app/navigation";

    import { user, checkAuth } from "$lib/auth.svelte";

    let { children } = $props();

    let checking = true;

    $effect(() => {
        checkAuth();
        if (user.user) {
            goto("/home", {
                replaceState: true,
            });
        }
        checking = false;
    });
</script>

{@render children()}

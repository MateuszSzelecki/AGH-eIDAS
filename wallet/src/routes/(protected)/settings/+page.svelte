<script lang="ts">
    import { logout } from "$lib/auth.svelte";
</script>

<h1>Settings</h1>
<button onclick={() => logout()}>Logout</button>

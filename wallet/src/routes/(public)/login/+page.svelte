<script>
    import { login, user } from "$lib/auth.svelte";
    let username = $state("");
    let password = $state("");

    let loading = $state(false);

    async function handleLogin() {
        loading = true;
        const result = login(username, password);
        loading = false;
    }
</script>

<div class="form">
    <h2>Login</h2>

    <input type="text" placeholder="Username" bind:value={username} />

    <input type="password" placeholder="Password" bind:value={password} />

    <button onclick={handleLogin} disabled={loading}>
        {loading ? "Logging in..." : "Login"}
    </button>

    <p>Don't have an account?</p>
    <a href="/register">Create one</a>
</div>

<style>
    .form {
        max-width: 300px;
        margin: auto;
        display: flex;
        flex-direction: column;
        gap: 10px;
    }
</style>

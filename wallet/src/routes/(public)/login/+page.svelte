<script lang="ts">
    import { login } from "$lib/auth.svelte";
    
    let username = $state("");
    let password = $state("");
    let loading = $state(false);
    let errorMsg = $state("");

    async function handleLogin() {
        if (!username.trim() || !password.trim()) {
            errorMsg = "Wpisz nazwę użytkownika i hasło.";
            return;
        }
        loading = true;
        errorMsg = "";
        try {
            await login(username, password);
        } catch (err: any) {
            errorMsg = typeof err === "string" ? err : "Błąd logowania.";
        } finally {
            loading = false;
        }
    }
</script>

<div class="form">
    <h2>Login</h2>

    <input type="text" placeholder="Username" bind:value={username} />
    <input type="password" placeholder="Password" bind:value={password} />

    {#if errorMsg}
        <p style="color: red; font-size: 14px; margin: 0;">{errorMsg}</p>
    {/if}

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

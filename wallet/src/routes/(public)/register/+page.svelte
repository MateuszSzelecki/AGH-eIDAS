<script>
    import { register } from "$lib/auth.svelte";
    let username = $state("");
    let password = $state("");

    let loading = $state(false);

    async function handleRegister() {
        loading = true;
        const result = await register(username, password);
        loading = false;
    }
</script>

<div class="form">
    <h2>Register</h2>

    <input type="text" placeholder="Username" bind:value={username} />

    <input type="password" placeholder="Password" bind:value={password} />

    <button onclick={handleRegister} disabled={loading}>
        {loading ? "Registering..." : "Register"}
    </button>

    <p>Already registered?</p>
    <a href="/login">Login</a>
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

<script lang="ts">
    import { register } from "$lib/auth.svelte";
    
    let username = $state("");
    let email = $state("");
    let officeCode = $state("");
    let password = $state("");
    let confirmPassword = $state("");
    let loading = $state(false);
    let errorMsg = $state("");
    
    async function handleRegister() {
        if (!username.trim() || !email.trim() || !officeCode.trim() || !password.trim() || !confirmPassword.trim()) {
            errorMsg = "Please fill in all fields.";
            return;
        }
        if (password !== confirmPassword) {
            errorMsg = "Passwords must match.";
            return;
        }
        const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
        if (!emailRegex.test(email.trim())) {
            errorMsg = "Please enter a valid email address.";
            return;
        }
        if (password.length < 8) {
            errorMsg = "Password must be at least 8 characters long.";
            return;
        }
        if (!/[A-Z]/.test(password)) {
            errorMsg = "Password must contain at least one uppercase letter.";
            return;
        }
        if (!/[a-z]/.test(password)) {
            errorMsg = "Password must contain at least one lowercase letter.";
            return;
        }
        if (!/[0-9]/.test(password)) {
            errorMsg = "Password must contain at least one digit.";
            return;
        }

        loading = true;
        errorMsg = "";
        try {
            await register(username, email, officeCode, password);
        } catch (err: any) {
            errorMsg = typeof err === "string" ? err : "Registration error.";
        } finally {
            loading = false;
        }
    }
</script>

<div class="form">
    <h2>Register</h2>

    <input type="text" placeholder="Username" bind:value={username} />
    <input type="email" placeholder="Email" bind:value={email} />
    <input type="text" placeholder="Office Code (kod z urzędu)" bind:value={officeCode} />
    <input type="password" placeholder="Password" bind:value={password} />
    <input type="password" placeholder="Confirm Password" bind:value={confirmPassword} />

    {#if errorMsg}
        <p style="color: red; font-size: 14px; margin: 0;">{errorMsg}</p>
    {/if}

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

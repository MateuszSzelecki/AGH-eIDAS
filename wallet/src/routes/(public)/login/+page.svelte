<script lang="ts">
    import { login } from "$lib/auth.svelte";
    
    let username = $state("");
    let password = $state("");
    let showPassword = $state(false);
    let loading = $state(false);
    let errorMsg = $state("");

    async function handleLogin() {
        if (!username.trim() || !password.trim()) {
            errorMsg = "Please enter username and password.";
            return;
        }

        loading = true;
        errorMsg = "";

        try {
            await login(username, password);
        } catch (err: any) {
            errorMsg = typeof err === "string" ? err : "Login error.";
        } finally {
            loading = false;
        }
    }
</script>
<div class="app-container">
    <div class="form-card">
        <div class="top-bar"></div>
        
        <h2>eIDAS Wallet</h2>
        <p class="subtitle">Log in to your wallet</p>

        <div class="input-group">
            <div class="input-wrapper">
                <input type="text" placeholder="Username" bind:value={username} />
            </div>
            <div class="input-wrapper" style="position: relative;">
    <input 
        type={showPassword ? "text" : "password"} 
        placeholder="Password" 
        bind:value={password} 
    />
    <button 
        type="button" 
        class="eye-btn" 
        onclick={() => showPassword = !showPassword}
        aria-label={showPassword ? "Hide password" : "Show password"}
    >
        {#if showPassword}
            <!-- Ikona SOLID (widoczne) -->
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" width="24" height="24">
                <path d="M12 15a3 3 0 1 0 0-6 3 3 0 0 0 0 6Z" />
                <path fill-rule="evenodd" d="M1.323 11.447C2.811 6.976 7.028 3.75 12.001 3.75c4.97 0 9.185 3.223 10.675 7.69.12.362.12.752 0 1.113-1.487 4.471-5.705 7.697-10.677 7.697-4.97 0-9.186-3.223-10.675-7.69a1.762 1.762 0 0 1 0-1.113ZM17.25 12a5.25 5.25 0 1 1-10.5 0 5.25 5.25 0 0 1 10.5 0Z" clip-rule="evenodd" />
            </svg>
        {:else}
            <!-- Ikona OUTLINE (ukryte) -->
            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" width="24" height="24">
                <path stroke-linecap="round" stroke-linejoin="round" d="M2.036 12.322a1.012 1.012 0 0 1 0-.639C3.423 7.51 7.36 4.5 12 4.5c4.638 0 8.573 3.007 9.963 7.178.07.207.07.431 0 .639C20.577 16.49 16.64 19.5 12 19.5c-4.638 0-8.573-3.007-9.963-7.178Z" />
                <path stroke-linecap="round" stroke-linejoin="round" d="M15 12a3 3 0 1 1-6 0 3 3 0 0 1 6 0Z" />
            </svg>
        {/if}
    </button>
</div>
        </div>

        {#if errorMsg}
            <p style="color: #ff4d4d; font-size: 14px; margin-bottom: 15px; margin-top: -10px;">{errorMsg}</p>
        {/if}

        <button onclick={handleLogin} disabled={loading} class="login-btn">
            {loading ? "Logging in..." : "Login"}
        </button>
        
        <div class="footer">
            <p>Don't have an account?</p>
            <a href="/register">Create one</a>
        </div>
    </div>
</div>

<style>

@import url('https://fonts.googleapis.com/css2?family=Inter:wght@300;400;500;600;700&display=swap');

  @font-face {
    font-family: 'Coolvetica';
    src: url('/fonts/coolvetica.otf') format('opentype');
    font-weight: normal;
    font-style: normal;
  }

  :global(body) {
    margin: 0;
    padding: 0;
    min-height: 100vh;
    font-family: 'Inter', -apple-system, sans-serif;
    display: flex;
    justify-content: center;
    align-items: center;
    background: radial-gradient(circle at center, #597aa0 0%, #1b232b 100%);
    background-size: cover;
    background-attachment: fixed;
  }

  @media (max-width: 768px) {
    :global(body) {
      background: radial-gradient(circle at center, #597aa0 0%, #1b232b 73%);
    }
  }

  .app-container {
    display: flex;
    justify-content: center;
    align-items: center;
    width: 100%;
    height: 100%;
  }

  .form-card {
    background: 
      radial-gradient(circle at 50% -10%, rgba(156, 213, 255, 0.2) 0%, transparent 60%),
      #0f1419;
    backdrop-filter: blur(20px);
    padding: 3.5rem 2.5rem;
    border-radius: 4px;
    width: 340px;
    text-align: center;
    position: relative;
    border: 1px solid rgba(156, 213, 255, 0.15);
    box-shadow: 0 20px 60px rgba(0, 0, 0, 0.8);
    overflow: hidden;
    transition: all 0.3s ease;
  }

  @media (max-width: 768px) {
    .form-card {
      width: 85%;
      max-width: 300px;
      min-height: 55vh;
      display: flex;
      flex-direction: column;
      justify-content: center;
      padding: 4rem 1.5rem;
    }
  }

  .top-bar {
    position: absolute;
    top: -2px; left: 50%;
    transform: translateX(-50%);
    width: 100px; height: 3px;
    background: #9CD5FF;
    box-shadow: 0 0 12px #9CD5FF;
  }

  h2 {
    font-family: 'Coolvetica', sans-serif;
    font-weight: 400;
    font-size: 2.4rem;
    color: #ffffff;
    margin: 0;
    letter-spacing: 1px;
    position: relative;
    z-index: 2;
  }

  .subtitle {
    color: #7AAACE;
    font-weight: 300;
    margin-bottom: 2.5rem;
    font-size: 0.9rem;
    opacity: 0.8;
  }

  .input-group {
    display: flex;
    flex-direction: column;
    gap: 12px;
    margin-bottom: 2rem;
  }

  input {
    width: 100%;
    padding: 16px;
    border-radius: 16px;
    border: 1px solid rgba(255, 255, 255, 0.05);
    background: rgba(255, 255, 255, 0.03);
    color: white;
    font-size: 0.95rem;
    box-sizing: border-box;
    transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  }

  input:focus {
    outline: none;
    background: rgba(255, 255, 255, 0.07);
    border-color: #7AAACE;
    box-shadow: 0 0 15px rgba(122, 170, 206, 0.2);
  }

  .login-btn {
    width: 100%;
    padding: 16px;
    background: linear-gradient(90deg, #355872, #7AAACE);
    color: white;
    border: none;
    border-radius: 16px;
    cursor: pointer;
    font-weight: 600;
    font-size: 1rem;
    transition: all 0.3s ease;
    box-shadow: 0 4px 15px rgba(0, 0, 0, 0.2);
  }

  .login-btn:hover:not(:disabled) {
    transform: translateY(-2px);
    box-shadow: 0 8px 25px rgba(122, 170, 206, 0.3);
    filter: brightness(1.1);
  }

  .login-btn:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }

  .footer {
    margin-top: 2rem;
    font-size: 0.85rem;
  }

  .footer p { color: #5c6a75; margin-bottom: 0.5rem; }
  
  a {
    color: #9CD5FF;
    text-decoration: none;
    font-weight: 500;
    transition: color 0.2s;
  }

  a:hover { color: #ffffff; }

  .eye-btn {
    position: absolute;
    right: 15px;
    top: 50%;
    transform: translateY(-50%);
    background: none;
    border: none;
    font-size: 1.2rem;
    cursor: pointer;
    opacity: 0.6;
    transition: opacity 0.2s;
    padding: 0;
}

.eye-btn svg {
    color: #7AAACE; 
    width: 20px;
    height: 20px;
}

.eye-btn:hover {
    opacity: 1;
}

</style>
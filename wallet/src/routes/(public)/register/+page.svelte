<script lang="ts">
    import { register } from "$lib/auth.svelte";
    
    let username = $state("");
    let email = $state("");
    let officeCode = $state("");
    let password = $state("");
    let confirmPassword = $state("");
    
    let showPassword = $state(false);
    
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

<div class="app-container">
    <div class="form-card">
        <div class="top-bar"></div>
        
        <h2>eIDAS Wallet</h2>
        <p class="subtitle">Create your secure account</p>

        <div class="input-group">
            <div class="input-wrapper">
                <input type="text" placeholder="Username" bind:value={username} />
            </div>
            
            <div class="input-wrapper">
                <input type="email" placeholder="Email Address" bind:value={email} />
            </div>

            <div class="input-wrapper">
                <input type="text" placeholder="Office Code" bind:value={officeCode} />
            </div>

            <div class="input-wrapper" style="position: relative;">
                <input 
                    type={showPassword ? "text" : "password"} 
                    placeholder="Password" 
                    bind:value={password} 
                />
                <button type="button" class="eye-btn" onclick={() => showPassword = !showPassword}>
                    {#if showPassword}
                        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" width="20" height="20"><path d="M12 15a3 3 0 1 0 0-6 3 3 0 0 0 0 6Z" /><path fill-rule="evenodd" d="M1.323 11.447C2.811 6.976 7.028 3.75 12.001 3.75c4.97 0 9.185 3.223 10.675 7.69.12.362.12.752 0 1.113-1.487 4.471-5.705 7.697-10.677 7.697-4.97 0-9.186-3.223-10.675-7.69a1.762 1.762 0 0 1 0-1.113ZM17.25 12a5.25 5.25 0 1 1-10.5 0 5.25 5.25 0 0 1 10.5 0Z" clip-rule="evenodd" /></svg>
                    {:else}
                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" width="20" height="20"><path stroke-linecap="round" stroke-linejoin="round" d="M2.036 12.322a1.012 1.012 0 0 1 0-.639C3.423 7.51 7.36 4.5 12 4.5c4.638 0 8.573 3.007 9.963 7.178.07.207.07.431 0 .639C20.577 16.49 16.64 19.5 12 19.5c-4.638 0-8.573-3.007-9.963-7.178Z" /><path stroke-linecap="round" stroke-linejoin="round" d="M15 12a3 3 0 1 1-6 0 3 3 0 0 1 6 0Z" /></svg>
                    {/if}
                </button>
            </div>

            <div class="input-wrapper">
        <input 
            type="password" 
            placeholder="Confirm Password" 
            bind:value={confirmPassword} 
        />
    </div>
        </div>

        {#if errorMsg}
            <p class="error-text">{errorMsg}</p>
        {/if}

        <button onclick={handleRegister} disabled={loading} class="register-btn">
            {loading ? "Creating account..." : "Register"}
        </button>
        
        <div class="footer">
            <p>Already registered?</p>
            <a href="/login">Login</a>
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

  .app-container {
    display: flex;
    justify-content: center;
    align-items: center;
    width: 100%;
    height: 100%;
    padding: 20px;
  }

  .form-card {
    background: 
      radial-gradient(circle at 50% -10%, rgba(156, 213, 255, 0.2) 0%, transparent 60%),
      #0f1419;
    backdrop-filter: blur(20px);
    padding: 3rem 2.5rem;
    border-radius: 4px;
    width: 360px;
    text-align: center;
    position: relative;
    border: 1px solid rgba(156, 213, 255, 0.15);
    box-shadow: 0 20px 60px rgba(0, 0, 0, 0.8);
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
  }

  .subtitle {
    color: #7AAACE;
    font-weight: 300;
    margin-bottom: 2rem;
    font-size: 0.9rem;
    opacity: 0.8;
  }

  .input-group {
    display: flex;
    flex-direction: column;
    gap: 12px;
    margin-bottom: 1.5rem;
  }

  input {
    width: 100%;
    padding: 14px 16px;
    border-radius: 12px;
    border: 1px solid rgba(255, 255, 255, 0.05);
    background: rgba(255, 255, 255, 0.03);
    color: white;
    font-size: 0.9rem;
    box-sizing: border-box;
    transition: all 0.3s ease;
  }

  input:focus {
    outline: none;
    border-color: #7AAACE;
    background: rgba(255, 255, 255, 0.07);
  }

  .eye-btn {
    position: absolute;
    right: 12px;
    top: 50%;
    transform: translateY(-50%);
    background: none;
    border: none;
    color: #7AAACE;
    cursor: pointer;
    opacity: 0.6;
    padding: 5px;
    display: flex;
    align-items: center;
  }

  .eye-btn:hover { opacity: 1; }

  .error-text {
    color: #ff4d4d;
    font-size: 13px;
    margin-bottom: 15px;
    text-align: left;
  }

  .register-btn {
    width: 100%;
    padding: 16px;
    background: linear-gradient(90deg, #355872, #7AAACE);
    color: white;
    border: none;
    border-radius: 12px;
    cursor: pointer;
    font-weight: 600;
    font-size: 1rem;
    transition: all 0.3s ease;
  }

  .register-btn:hover:not(:disabled) {
    transform: translateY(-2px);
    filter: brightness(1.1);
  }

  .footer {
    margin-top: 1.5rem;
    font-size: 0.85rem;
  }

  .footer p { color: #5c6a75; margin-bottom: 0.5rem; }
  
  a {
    color: #9CD5FF;
    text-decoration: none;
    font-weight: 500;
  }

  a:hover { color: #ffffff; }
</style>
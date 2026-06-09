<script lang="ts">
    let firstName = $state("");
    let lastName = $state("");
    let dateOfBirth = $state("");
    
    let loading = $state(false);
    let errorMsg = $state("");
    let successCode = $state("");
    let copyHint = $state("Click the code to copy to clipboard");

    async function handleGenerate(e: Event) {
        e.preventDefault();
        
        if (!firstName.trim() || !lastName.trim() || !dateOfBirth) {
            errorMsg = "Please fill in all fields.";
            return;
        }

        loading = true;
        errorMsg = "";
        successCode = "";
        copyHint = "Click the code to copy to clipboard";

        const dobTimestamp = Math.floor(new Date(dateOfBirth).getTime() / 1000);

        try {
            const response = await fetch('http://127.0.0.1:8000/generate-code', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify({ 
                    firstName, 
                    lastName, 
                    dateOfBirth: dobTimestamp 
                })
            });

            if (response.ok) {
                const data = await response.json();
                successCode = data.code;
            } else {
                const errText = await response.text();
                errorMsg = errText || "Failed to generate code.";
            }
        } catch (err: any) {
            errorMsg = "Network error. Could not reach server. Make sure the backend is running.";
        } finally {
            loading = false;
        }
    }

    function copyCode() {
        if (!successCode) return;
        navigator.clipboard.writeText(successCode).then(() => {
            copyHint = "Copied to clipboard! ✓";
            setTimeout(() => {
                copyHint = "Click the code to copy to clipboard";
            }, 2000);
        });
    }
</script>

<div class="app-container">
    <div class="form-card">
        <div class="top-bar"></div>
        
        <h2>Admin Portal</h2>
        <p class="subtitle">Generate an activation code for a citizen</p>

        <form onsubmit={handleGenerate}>
            <div class="input-group">
                <div class="input-wrapper">
                    <input type="text" placeholder="First Name" bind:value={firstName} required />
                </div>
                <div class="input-wrapper">
                    <input type="text" placeholder="Last Name" bind:value={lastName} required />
                </div>
                <div class="input-wrapper date-wrapper">
                    <input type="date" placeholder="Date of Birth" bind:value={dateOfBirth} required class="date-input" />
                    <div class="custom-calendar-icon">
                        {#if dateOfBirth}
                            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" width="20" height="20">
                              <path fill-rule="evenodd" d="M6.75 2.25A.75.75 0 0 1 7.5 3v1.5h9V3A.75.75 0 0 1 18 3v1.5h.75a3 3 0 0 1 3 3v11.25a3 3 0 0 1-3 3H5.25a3 3 0 0 1-3-3V7.5a3 3 0 0 1 3-3H6V3a.75.75 0 0 1 .75-.75Zm13.5 9a1.5 1.5 0 0 0-1.5-1.5H5.25a1.5 1.5 0 0 0-1.5 1.5v7.5a1.5 1.5 0 0 0 1.5 1.5h13.5a1.5 1.5 0 0 0 1.5-1.5v-7.5Z" clip-rule="evenodd" />
                            </svg>
                        {:else}
                            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" width="20" height="20">
                              <path stroke-linecap="round" stroke-linejoin="round" d="M6.75 3v2.25M17.25 3v2.25M3 18.75V7.5a2.25 2.25 0 0 1 2.25-2.25h13.5A2.25 2.25 0 0 1 21 7.5v11.25m-18 0A2.25 2.25 0 0 0 5.25 21h13.5A2.25 2.25 0 0 0 21 18.75m-18 0v-7.5A2.25 2.25 0 0 1 5.25 9h13.5A2.25 2.25 0 0 1 21 11.25v7.5" />
                            </svg>
                        {/if}
                    </div>
                </div>
            </div>

            {#if errorMsg}
                <p class="error-msg">{errorMsg}</p>
            {/if}

            <button type="submit" disabled={loading} class="action-btn">
                {loading ? "Generating..." : "Generate Activation Code"}
            </button>
        </form>

        {#if successCode}
            <div class="result-box">
                <div class="status-badge">Success</div>
                <!-- eslint-disable-next-line a11y-interactive-supports-focus -->
                <!-- svelte-ignore a11y_interactive_supports_focus -->
                <div class="code-display" onclick={copyCode} onkeydown={(e) => e.key === 'Enter' && copyCode()} role="button">
                    {successCode}
                </div>
                <div class="copy-hint">{copyHint}</div>
            </div>
        {/if}
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
    min-height: 100vh;
    box-sizing: border-box;
    padding: 20px;
  }

  .form-card {
    background: 
      radial-gradient(circle at 50% -10%, rgba(156, 213, 255, 0.2) 0%, transparent 60%),
      #0f1419;
    backdrop-filter: blur(20px);
    padding: 3.5rem 2.5rem;
    border-radius: 4px;
    width: 100%;
    max-width: 380px;
    text-align: center;
    position: relative;
    border: 1px solid rgba(156, 213, 255, 0.15);
    box-shadow: 0 20px 60px rgba(0, 0, 0, 0.8);
    overflow: hidden;
    transition: all 0.3s ease;
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
    font-size: 2.2rem;
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

  .date-wrapper {
    position: relative;
    display: flex;
    align-items: center;
  }

  .date-input {
    color: white;
  }
  
  .date-input:invalid::-webkit-datetime-edit {
    color: rgba(255, 255, 255, 0.4);
  }

  .date-input::-webkit-calendar-picker-indicator {
    position: absolute;
    right: 16px;
    width: 24px;
    height: 24px;
    opacity: 0;
    cursor: pointer;
    z-index: 2;
  }

  .custom-calendar-icon {
    position: absolute;
    right: 18px;
    display: flex;
    align-items: center;
    justify-content: center;
    color: white;
    z-index: 1;
    pointer-events: none;
    opacity: 0.7;
    transition: opacity 0.2s;
  }

  .date-input:focus ~ .custom-calendar-icon,
  .custom-calendar-icon:has(svg[fill="currentColor"]) {
    opacity: 1;
    color: #9CD5FF;
  }

  
  .date-input:hover ~ .custom-calendar-icon {
    opacity: 1;
    transform: scale(1.1); 
    filter: drop-shadow(0 0 8px rgba(156, 213, 255, 0.6)); 
    color: #9CD5FF;
  }


  .error-msg {
    color: #ff4d4d;
    font-size: 14px;
    margin-bottom: 15px;
    margin-top: -10px;
  }

  .action-btn {
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

  .action-btn:hover:not(:disabled) {
    transform: translateY(-2px);
    box-shadow: 0 8px 25px rgba(122, 170, 206, 0.3);
    filter: brightness(1.1);
  }

  .action-btn:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }

  .result-box {
    margin-top: 2rem;
    background: rgba(255, 255, 255, 0.03);
    border: 1px dashed rgba(74, 222, 128, 0.4);
    border-radius: 16px;
    padding: 1.5rem 1rem;
    animation: fadeIn 0.4s ease-out;
  }

  .status-badge {
    font-size: 10px;
    text-transform: uppercase;
    letter-spacing: 1.5px;
    color: #4ade80;
    background: rgba(74, 222, 128, 0.1);
    padding: 4px 12px;
    border-radius: 20px;
    display: inline-block;
    margin-bottom: 0.5rem;
  }

  .code-display {
    font-family: monospace;
    font-size: 2rem;
    color: #ffffff;
    padding: 1rem;
    background: rgba(0, 0, 0, 0.3);
    border-radius: 8px;
    margin: 10px 0;
    cursor: pointer;
    letter-spacing: 4px;
    transition: all 0.2s;
  }

  .code-display:hover {
    background: rgba(74, 222, 128, 0.15);
    color: #4ade80;
    transform: scale(1.02);
  }

  .copy-hint {
    font-size: 12px;
    color: #7AAACE;
    opacity: 0.8;
  }

  @keyframes fadeIn {
    from { opacity: 0; transform: translateY(-10px); }
    to { opacity: 1; transform: translateY(0); }
  }

  @media (max-width: 768px) {
    .form-card {
      padding: 3rem 1.5rem;
    }
  }
</style>
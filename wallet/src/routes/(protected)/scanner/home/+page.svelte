<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { getIssuerUrl } from "$lib/auth.svelte";

    interface UserDocument {
        identifier: string;
        firstName: string;
        lastName: string;
        dateOfBirth: number;
        issueDate: number;
        expiryDate: number;
        sigR: string;
        sigS: string;
    }

    let document: UserDocument = $state({
        identifier: "", 
        firstName: "",
        lastName: "",
        dateOfBirth: 0,
        issueDate: 0,
        expiryDate: 0,
        sigR: "",
        sigS: "",
    });

    let loading = $state(false);
    let showCryptoDetails = $state(false);
  
    let activationCode = $state("");
    let codeError = $state("");

    $effect(() => {
        if (!document.identifier) {
            loadDocument();
        }
    });

    async function loadDocument() {
        document = await invoke("load_document");
    }

    async function requestDocument() {
        if (!activationCode.trim()) {
            codeError = "Please enter the activation code.";
            return;
        }
        
        loading = true;
        codeError = "";
        
        try {
            document = await invoke("request_document", { code: activationCode });
            activationCode = ""; 
        } catch (error) {
            codeError = "Failed to claim document. Please check your code.";
            console.error("Error claiming document:", error);
        } finally {
            loading = false;
        }
    }

    function formatDate(timestamp: number) {
        if (!timestamp) return "-";
        return new Date(timestamp * 1000).toLocaleDateString();
    }

</script>

<div class="app-container">
    <div class="dashboard-wrapper">
        <header class="dashboard-header">
            <h1 style="font-family: 'Coolvetica', sans-serif;">Home</h1>
            <p class="subtitle">Your secure EU Digital Wallet</p>
        </header>

        {#if document.identifier}
            <div id="document" class="digital-id-card">
            <div class="card-header">
              <span style="font-size: 12px; font-weight: 600; letter-spacing: 2px; text-transform: uppercase; color: #E0F2FE;">
        🇪🇺 EU Digital Identity
              </span>
            </div>
        <hr style="border: none; border-top: 1px solid rgba(255, 255, 255, 0.05); margin: 15px 0 20px 0;">
                <div class="card-body">
                    <div class="info-row">
                        <div class="info-group">
                            <span class="field-label">First Name</span>
                            <p class="value">{document.firstName}</p>
                        </div>
                        <div class="info-group" style="text-align: right;">
                            <span class="field-label">Last Name</span>
                            <p class="value">{document.lastName}</p>
                        </div>
                    </div>

                    <div class="info-row">
                        <div class="info-group">
                            <span class="field-label">Date of Birth</span>
                            <p class="value highlight">{formatDate(document.dateOfBirth)}</p>
                        </div>
                    </div>

                    <div class="info-row dates">
                        <div class="info-group">
                            <span class="field-label">Issue Date</span>
                            <p class="value small">{formatDate(document.issueDate)}</p>
                        </div>
                        <div class="info-group" style="text-align: right;">
                            <span class="field-label">Expiry Date</span>
                            <p class="value small">{formatDate(document.expiryDate)}</p>
                        </div>
                    </div>
                </div>

                <div class="card-footer">
                    <p class="id-number">ID: {document.identifier}</p>
                </div>
            </div>

            <div class="actions">
                <button class="action-btn outline" onclick={requestDocument}>Refresh Document</button>
                
                <button class="toggle-btn" onclick={() => showCryptoDetails = !showCryptoDetails}>
                    {showCryptoDetails ? "Hide" : "Show"} Cryptographic Signatures
                </button>
            </div>

            {#if showCryptoDetails}
                <div class="crypto-panel">
                    <div class="crypto-item">
                        <span class="crypto-label">Sig R:</span>
                        <code class="crypto-code">{document.sigR}</code>
                    </div>
                    <div class="crypto-item">
                        <span class="crypto-label">Sig S:</span>
                        <code class="crypto-code">{document.sigS}</code>
                    </div>
                </div>
            {/if}

        {:else}
            <div class="empty-state glass-card" style="padding: 40px 24px;">
                <div class="top-bar"></div>
                
                <h2 style="font-family: 'Inter', sans-serif; font-weight: 600; font-size: 1.8rem; margin-top: 0; margin-bottom: 8px; letter-spacing: 0.5px;">Claim Document</h2>
                <p style="color: #7AAACE; font-size: 14px; margin-bottom: 24px;">Enter the activation code provided by the office.</p>

                <div style="margin-bottom: 20px;">
                    <input 
                        type="text" 
                        placeholder="e.g. 8K2P9X" 
                        bind:value={activationCode} 
                        maxlength="6"
                        style="text-align: center; font-family: monospace; font-size: 1.2rem; letter-spacing: 2px; text-transform: uppercase;"
                      />
                </div>

                {#if codeError}
                    <p style="color: #ff4d4d; font-size: 13px; margin-top: -10px; margin-bottom: 15px; text-align: left;">{codeError}</p>
                {/if}

                <button class="action-btn" onclick={requestDocument} disabled={loading}>
                    {loading ? "Verifying..." : "Claim Document"}
                </button>
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
  color: white;
  background: radial-gradient(circle at center, #1b232b 30%, #080a0c 100%);
  background-attachment: fixed;
}

  .app-container {
    display: flex;
    justify-content: center;
    width: 100%;
    min-height: 100vh;
    padding: 40px 20px 120px 20px; 
    box-sizing: border-box;
  }

  .dashboard-wrapper {
    margin: auto; 
    width: 100%;
    max-width: 500px;
    display: flex;
    flex-direction: column;
    gap: 20px;
    animation: fadeIn 0.4s ease-out;
  }

  .dashboard-header {
    text-align: center;
  }

  .dashboard-header h1 {
    font-family: 'Coolvetica', sans-serif;
    font-weight: 400;
    font-size: 2.4rem;
    margin: 0 0 8px 0;
    color: #ffffff;
    letter-spacing: 1px;
  }

  .subtitle {
    color: #BEE3FF; 
    font-weight: 400;
    margin: 0;
    font-size: 0.95rem;
  }

  .glass-card {
    background: radial-gradient(circle at 50% -10%, rgba(156, 213, 255, 0.15) 0%, transparent 60%), #0f1419;
    border: 1px solid rgba(156, 213, 255, 0.15);
    box-shadow: 0 25px 50px rgba(0,0,0,0.5);
    border-radius: 12px; 
    position: relative; 
    overflow: hidden;
  }

  .top-bar {
    position: absolute;
    top: 0; left: 50%;
    transform: translateX(-50%);
    width: 100px; height: 3px;
    background: #9CD5FF; 
    box-shadow: 0 0 12px #9CD5FF;
    z-index: 10;
  }

  .id-card {
    padding: 24px;
  }

  .card-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 24px;
    border-bottom: 1px solid rgba(255, 255, 255, 0.1);
    padding-bottom: 12px;
  }

  .card-title {
    font-size: 12px;
    font-weight: 600;
    letter-spacing: 2px;
    text-transform: uppercase;
    color: #9CD5FF;
  }


  .digital-id-card {
    position: relative;
    isolation: isolate;
    overflow: hidden;
    border-radius: 20px;
    padding: 28px;

    background:
      linear-gradient(155deg,
        #1a2634 0%,
        #141d28 30%,
        #1c2836 55%,
        #111923 75%,
        #0c1218 100%);

    border: 1px solid rgba(156, 213, 255, 0.15);
    box-shadow:
      0 30px 60px -20px rgba(0, 0, 0, 0.75),
      0 12px 30px -10px rgba(0, 0, 0, 0.5),
      inset 0 1px 0 rgba(255, 255, 255, 0.08);
  }

  .digital-id-card::before {
    content: "";
    position: absolute;
    inset: -25%;
    z-index: -1;
    background:
      radial-gradient(55% 50% at 85% 90%, rgba(156, 213, 255, 0.25) 0%, transparent 70%),
      radial-gradient(50% 45% at 15% 100%, rgba(53, 88, 114, 0.35) 0%, transparent 75%),
      radial-gradient(45% 40% at 80% 10%, rgba(156, 213, 255, 0.15) 0%, transparent 80%),
      radial-gradient(40% 45% at 20% 25%, rgba(53, 88, 114, 0.25) 0%, transparent 80%);
    filter: blur(70px);
    opacity: 0.9;
  }

  .digital-id-card::after {
    content: "";
    position: absolute;
    inset: 0;
    z-index: -1;
    background:
      linear-gradient(180deg,
        rgba(255, 255, 255, 0.06) 0%,
        rgba(255, 255, 255, 0.01) 25%,
        transparent 55%),
      radial-gradient(circle 180px at 100% 0%, rgba(156, 213, 255, 0.12) 0%, transparent 70%);
    pointer-events: none;
  }

  @keyframes glare {
    0% { left: -100%; }
    20% { left: 200%; }
    100% { left: 200%; }
  }

  .status-active {
    font-size: 10px;
    text-transform: uppercase;
    background: rgba(74, 222, 128, 0.2);
    color: #4ade80;
    padding: 4px 8px;
    border-radius: 12px;
    letter-spacing: 1px;
  }

  .field-label {
    font-size: 10px;
    text-transform: uppercase;
    letter-spacing: 1px;
    color: #BEE3FF; 
    margin-bottom: 4px;
    display: block;
  }

  .crypto-panel {
    background: rgba(0, 0, 0, 0.4);
    border: 1px solid rgba(156, 213, 255, 0.1);
    border-radius: 12px;
    padding: 16px;
    animation: slideDown 0.3s ease-out;
  }

  .crypto-item {
    margin-bottom: 10px;
  }
  .crypto-item:last-child {
    margin-bottom: 0;
  }

  .crypto-label {
    display: block;
    font-size: 11px;
    color: #7AAACE;
    margin-bottom: 4px;
  }

  .crypto-code {
    display: block;
    font-family: monospace;
    font-size: 11px;
    color: #a0aec0;
    word-break: break-all;
    background: rgba(255,255,255,0.05);
    padding: 6px;
    border-radius: 6px;
  }

  .recent-activity {
    margin-top: 10px;
    background: rgba(255, 255, 255, 0.03);
    border-radius: 16px;
    padding: 20px;
    border: 1px solid rgba(255, 255, 255, 0.05);
  }

  .recent-activity h3 {
    font-size: 14px;
    margin: 0 0 16px 0;
    color: #7AAACE;
    font-weight: 500;
  }

  .activity-item {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .activity-icon {
    width: 32px;
    height: 32px;
    border-radius: 50%;
    background: rgba(74, 222, 128, 0.1);
    color: #4ade80;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 14px;
  }

  .activity-details p {
    margin: 0;
    font-size: 14px;
    color: white;
  }

  .activity-details span {
    font-size: 11px;
    color: #616c75;
  }

  .info-row {
    display: flex;
    justify-content: space-between;
    margin-bottom: 20px;
    gap: 16px;
  }

  .info-group {
    display: flex;
    flex-direction: column;
    gap: 4px;
    flex: 1;
  }

  .info-group label {
    font-size: 10px;
    text-transform: uppercase;
    letter-spacing: 1px;
    color: #616c75;
    margin: 0;
  }

  .info-group p.value {
    margin: 0;
    font-size: 16px;
    font-weight: 600;
    color: #ffffff;
  }

  .info-group p.value.highlight {
    color: #4ade80;
    text-shadow: 0 0 10px rgba(74, 222, 128, 0.2);
  }

  .info-group p.value.small {
    font-size: 14px;
    font-weight: 400;
    color: #E0F2FE;
  }

  .card-footer {
    margin-top: 24px;
    padding-top: 16px;
    border-top: 1px solid rgba(255, 255, 255, 0.1);
    background: rgba(255, 255, 255, 0.02);
    margin: 24px -24px -24px -24px; 
    padding: 16px 24px;
  }

  .id-number {
    margin: 0;
    font-family: monospace;
    font-size: 11px;
    color: rgba(117, 138, 154, 1.00); 
    letter-spacing: 1px;
    word-break: break-all;
    text-align: center;
  }

  .actions {
    display: flex;
    flex-direction: column;
    gap: 12px;
    margin-top: 10px;
  }

  .action-btn {
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
    box-shadow: 0 4px 15px rgba(0, 0, 0, 0.2);
  }

  .toggle-btn {
    background: none;
    border: none;
    color: #758a9a; 
    font-size: 13px;
    cursor: pointer;
    transition: color 0.2s;
    padding: 8px;
    text-decoration: underline;
    text-underline-offset: 4px;
  }

  .toggle-btn:hover {
    color: #9CD5FF;
  }

  .crypto-panel {
    background: rgba(0, 0, 0, 0.3);
    border: 1px solid rgba(255, 255, 255, 0.05);
    border-radius: 12px;
    padding: 16px;
    animation: slideDown 0.3s ease-out;
  }

  .empty-state {
    padding: 40px 24px;
    text-align: center;
  }

  .empty-state p {
    color: #7AAACE;
    font-size: 16px;
    margin: 0 0 24px 0;
  }

  @media (hover: hover) {
    .action-btn:hover {
      transform: translateY(-2px);
      box-shadow: 0 8px 25px rgba(122, 170, 206, 0.3);
      filter: brightness(1.1);
    }
  }

  @media (pointer: coarse) {
    .action-btn:active {
      transform: scale(0.98);
      filter: brightness(0.8);
    }
  }

  @keyframes slideDown {
    from { opacity: 0; transform: translateY(-10px); }
    to { opacity: 1; transform: translateY(0); }
  }

  @keyframes fadeIn {
    from { opacity: 0; }
    to { opacity: 1; }
  }

  input {
    width: 100%;
    padding: 16px;
    border-radius: 12px;
    border: 1px solid rgba(255, 255, 255, 0.05);
    background: rgba(255, 255, 255, 0.03);
    color: white;
    box-sizing: border-box;
    transition: all 0.3s ease;
  }

  input:focus {
    outline: none;
    border-color: #7AAACE;
    background: rgba(255, 255, 255, 0.07);
    box-shadow: 0 0 15px rgba(122, 170, 206, 0.2);
  }
</style>


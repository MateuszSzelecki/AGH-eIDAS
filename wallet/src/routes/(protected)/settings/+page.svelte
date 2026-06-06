<script lang="ts">
    import { logout } from "$lib/auth.svelte";
    import { getVersion } from "@tauri-apps/api/app";
    import { onMount } from "svelte";

    let appVersion = $state("...");

    onMount(async () => {
        try {
            appVersion = await getVersion();
        } catch {
            appVersion = "1.0.0"; 
        }
    });
</script>

<div class="app-container">
    <div class="settings-card glass-card">
        <div class="top-bar"></div>
        <h2 style="font-family: 'Inter', sans-serif; font-weight: 600; font-size: 1.8rem; margin-top: 0; margin-bottom: 8px; letter-spacing: 0.5px;">Settings</h2>
        <p class="subtitle">Manage your wallet</p>

        <div class="info-section">
            <p><strong>App Version:</strong> {appVersion}</p>
        </div>

        <button class="logout-btn" onclick={() => logout()}>
            Log Out
        </button>
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

    .app-container {
        display: flex;
        justify-content: center;
        align-items: center;
        width: 100%;
        min-height: 100vh;
        padding: 20px;
        box-sizing: border-box;
        color: white;
    }

    .settings-card {
        background: radial-gradient(circle at 50% -10%, rgba(156, 213, 255, 0.15) 0%, transparent 60%), #0f1419;
        border: 1px solid rgba(156, 213, 255, 0.15);
        border-radius: 12px;
        padding: 40px 30px;
        width: 100%;
        max-width: 340px;
        text-align: center;
        position: relative;
    }

    .top-bar {
        position: absolute;
        top: 0; left: 50%;
        transform: translateX(-50%);
        width: 100px; height: 3px;
        background: #9CD5FF; 
        box-shadow: 0 0 12px #9CD5FF;
    }

    h2 {
        font-family: 'Coolvetica', sans-serif;
        font-weight: 400;
        font-size: 2rem;
        margin: 0 0 8px 0;
    }

    .subtitle {
        color: #7AAACE;
        font-size: 0.9rem;
        margin-bottom: 30px;
    }

    .info-section {
        background: rgba(255, 255, 255, 0.03);
        border-radius: 8px;
        padding: 15px;
        margin-bottom: 30px;
        text-align: left;
        font-size: 14px;
        color: #a0aec0;
    }

    .info-section p {
        margin: 8px 0;
    }

    .info-section strong {
        color: white;
    }

    .logout-btn {
        width: 100%;
        padding: 16px;
        background: transparent;
        color: #ff4d4d;
        border: 2px solid #ff4d4d;
        border-radius: 12px;
        font-weight: 600;
        font-size: 1rem;
        cursor: pointer;
        transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
        box-shadow: 0 4px 15px rgba(255, 77, 77, 0.1);
    }

    @media (hover: hover) {
        .logout-btn:hover {
            background: #ff4d4d;
            color: white;
            transform: translateY(-2px);
            box-shadow: 0 8px 25px rgba(255, 77, 77, 0.4);
        }
    }
</style>
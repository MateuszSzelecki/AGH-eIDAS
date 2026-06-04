<script lang="ts">
    import {
        Format,
        checkPermissions,
        requestPermissions,
        scan,
    } from "@tauri-apps/plugin-barcode-scanner";

    import { navigating } from "$app/state";
    import { invoke } from "@tauri-apps/api/core";

    interface ChallengePayload {
        challengeId: string;
        nonce: string;
        timestamp: number;
        callbackUrl: string;
        verifierName: string;
    }

    let permission = $state("prompt");

    let scanning = $state(false);

    let result: ChallengePayload = $state({
        challengeId: "",
        nonce: "",
        timestamp: -1,
        callbackUrl: "", 
        verifierName: "",  
    });

    let success = $state(false);
    let loading = $state(false);

    $effect(() => {
        if (navigating) {
            checkCameraPermissions();
        }
    });

    export function validateChallengePayload(json: string): ChallengePayload {
        let parsed: any;

        console.log(json);

        try {
            parsed = JSON.parse(json);
        } catch {
            throw new Error("Invalid JSON");
        }

        if (
            !(
                typeof parsed === "object" &&
                parsed !== null &&
                typeof parsed.challengeId === "string" &&
                typeof parsed.nonce === "string" &&
                typeof parsed.timestamp === "number" &&
                typeof parsed.callbackUrl === "string" &&
                typeof parsed.verifierName === "string"
            )
        ) {
            console.log(typeof parsed);
            console.log(typeof parsed.challengeId);
            console.log(typeof parsed.nonce);
            console.log(typeof parsed.timestamp);
            console.log(typeof parsed.callbackUrl);
            console.log(typeof parsed.verifierName);
            throw new Error("Invalid ChallengePayload structure");
        }

        if (!/^[a-f0-9]{32}$/i.test(parsed.nonce)) {
            console.log(parsed.nonce);
            throw new Error("Invalid nonce format");
        }

        if (!Number.isInteger(parsed.timestamp) || parsed.timestamp <= -1) {
            throw new Error("Invalid timestamp");
        }

        try {
            new URL(parsed.callbackUrl);
        } catch {
            throw new Error("Invalid callback URL");
        }

        let payload: ChallengePayload = {
            challengeId: parsed.challengeId,
            nonce: parsed.nonce,
            timestamp: parsed.timestamp,
            callbackUrl: parsed.callbackUrl,
            verifierName: parsed.verifierName,
        };

        return payload;
    }

    async function checkCameraPermissions() {
        permission = await checkPermissions();
    }

    async function requestCameraPermissions() {
        permission = await requestPermissions();
    }

    async function doScan() {
        if (!scanning) {
            scanning = true;
            let permission = await checkPermissions();
            if (permission === "prompt") {
                permission = await requestPermissions();
            }
            if (permission === "granted") {
                const scanned = await scan({
                    formats: [Format.QRCode],
                    windowed: true,
                });

                if (scanned) {
                    result = validateChallengePayload(scanned.content);
                }
            }
        }
        scanning = false;
    }

    async function handleYes() {
        loading = true;
        await invoke("generate_proof", { challenge: result });
        loading = false;
        success = true;
        result = {
            challengeId: "",
            nonce: "",
            timestamp: -1,
            callbackUrl: "",
            verifierName: "",
        };
    }

    function resetScanner() {
    success = false;
    }

    async function handleNo() {
        result = {
            challengeId: "",
            nonce: "",
            timestamp: -1,
            callbackUrl: "",
            verifierName: "",
        };
    }
</script>

<div class="scanner-container">
    {#if permission != "granted"}
        <div class="glass-card">
            <div class="top-bar"></div>
            <div class="status-badge">System Access</div>
            <h3>Camera Required</h3>
            <p>Please grant camera permissions to scan verification codes.</p>
            <button class="action-btn" onclick={requestCameraPermissions}>Request Permissions</button>
        </div>

    {:else if scanning}
        <!-- UI Skanowania -->
        <div class="scan-overlay">
            <div class="scan-area">
                <div class="corner top-left"></div>
                <div class="corner top-right"></div>
                <div class="corner bottom-left"></div>
                <div class="corner bottom-right"></div>
                <div class="scan-line"></div>
            </div>
            <p class="hint">Point your camera at a QR code</p>
            <button class="cancel-btn" onclick={() => scanning = false}>Cancel</button>
        </div>

    {:else if result.callbackUrl}
        <div class="glass-card">
            <div class="top-bar"></div>
            <div class="status-badge">Request Detected</div>
            <h2 style="font-family: 'Coolvetica', sans-serif; font-weight: 500;">Verify Identity?</h2>
            <div class="details-box">
                <div class="item"><span>Verifier:</span> <strong>{result.verifierName}</strong></div>
                <div class="item"><span>Endpoint:</span> <small>{result.callbackUrl}</small></div>
            </div>
            <div class="button-group">
                <button class="action-btn" onclick={handleYes}>Yes, Generate Proof</button>
                <button class="text-btn" onclick={handleNo}>No, Decline</button>
            </div>
        </div>
    
    {:else if loading}
        <!-- Ekran ładowania -->
        <div class="glass-card" style="padding-top: 4rem; padding-bottom: 4rem;">
            <div class="top-bar" style="background: #5A7E9A; box-shadow: 0 0 10px #5A7E9A;"></div>
            
            <div class="spinner-container">
                <svg xmlns="http://www.w3.org/2000/svg" width="64" height="64" viewBox="0 0 2400 2400" style="color: #5A7E9A;">
                    <g stroke-width="200" stroke-linecap="round" stroke="currentColor" fill="none">
                        <line x1="1200" y1="600" x2="1200" y2="100"/>
                        <line opacity="0.5" x1="1200" y1="2300" x2="1200" y2="1800"/>
                        <line opacity="0.917" x1="900" y1="680.4" x2="650" y2="247.4"/>
                        <line opacity="0.417" x1="1750" y1="2152.6" x2="1500" y2="1719.6"/>
                        <line opacity="0.833" x1="680.4" y1="900" x2="247.4" y2="650"/>
                        <line opacity="0.333" x1="2152.6" y1="1750" x2="1719.6" y2="1500"/>
                        <line opacity="0.75" x1="600" y1="1200" x2="100" y2="1200"/>
                        <line opacity="0.25" x1="2300" y1="1200" x2="1800" y2="1200"/>
                        <line opacity="0.667" x1="680.4" y1="1500" x2="247.4" y2="1750"/>
                        <line opacity="0.167" x1="2152.6" y1="650" x2="1719.6" y2="900"/>
                        <line opacity="0.583" x1="900" y1="1719.6" x2="650" y2="2152.6"/>
                        <line opacity="0.083" x1="1750" y1="247.4" x2="1500" y2="680.4"/>
                        <animateTransform attributeName="transform" attributeType="XML" type="rotate" keyTimes="0;0.08333;0.16667;0.25;0.33333;0.41667;0.5;0.58333;0.66667;0.75;0.83333;0.91667" values="0 1199 1199;30 1199 1199;60 1199 1199;90 1199 1199;120 1199 1199;150 1199 1199;180 1199 1199;210 1199 1199;240 1199 1199;270 1199 1199;300 1199 1199;330 1199 1199" dur="0.83333s" begin="0s" repeatCount="indefinite" calcMode="discrete"/>
                    </g>
                </svg>
            </div>
            <h2 style="font-family: 'Coolvetica', sans-serif; font-weight: 500; margin-top: 40px; margin-bottom: 15px; font-size: 22px;">
                Generating Proof...
            </h2>
            <p style="font-size: 14px; color: #9CD5FF; opacity: 0.7; margin-bottom: 10px; max-width: 250px; margin-left: auto; margin-right: auto;">
                Please wait while we verify your identity securely.
            </p>
        </div>



    {:else if success}
        <!-- Ekran sukcesu -->
        <div class="glass-card">
            <div class="top-bar" style="background: #4ade80; box-shadow: 0 0 12px #4ade80;"></div>
            <div class="status-badge" style="color: #4ade80; background: rgba(74, 222, 128, 0.1);">Success</div>
            
            <div class="success-icon">
                <svg xmlns="http://www.w3.org/2000/svg" width="64" height="64" viewBox="0 0 24 24" fill="none" stroke="#4ade80" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                    <path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"></path>
                    <polyline points="22 4 12 14.01 9 11.01"></polyline>
                </svg>
            </div>
            <h2 style="font-family: 'Coolvetica', sans-serif; font-weight: 500;">Identity Verified</h2>
            <p style="margin-bottom: 30px; font-size: 14px; color: rgba(255, 255, 255, 0.7);">
                Your zero-knowledge proof was generated and verified successfully.
            </p>
            
            <button class="action-btn" onclick={resetScanner}>Done</button>
        </div>

    {:else}
        <!-- UI Startowe -->
        <div class="glass-card">
            <div class="top-bar"></div>
            <div class="qr-icon">
    <svg xmlns="http://www.w3.org/2000/svg" width="64" height="64" viewBox="0 0 24 24" fill="none" stroke="#9CD5FF" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
        <rect width="5" height="5" x="3" y="3" rx="1" />
        <rect width="5" height="5" x="16" y="3" rx="1" />
        <rect width="5" height="5" x="3" y="16" rx="1" />
        <path d="M21 16h-3a2 2 0 0 0-2 2v3" />
        <path d="M21 21v.01" />
        <path d="M12 7v3a2 2 0 0 1-2 2H7" />
        <path d="M3 12h.01" />
        <path d="M12 3h.01" />
        <path d="M12 16v.01" />
        <path d="M16 12h1" />
        <path d="M21 12v.01" />
        <path d="M12 21v-1" />
    </svg>
</div>
            <h3>Scanner Ready</h3>
            <p>Scan a QR code to provide proof of identity without sharing sensitive data.</p>
            <button class="action-btn" onclick={doScan}>Start Scan</button>
        </div>
    {/if}
</div>

<style>
    .scanner-container {
        height: 100vh; 
        width: 100%;
        display: flex; 
        justify-content: center; 
        align-items: center;
        background: radial-gradient(circle at center, #1b232b 0%, #080a0c 100%);
        color: white; 
        font-family: 'Inter', sans-serif;
        margin: 0; 
        padding: 20px; 
        box-sizing: border-box;
    }

    .glass-card {
        background: radial-gradient(circle at 50% -10%, rgba(156, 213, 255, 0.2) 0%, transparent 60%),
          #0f1419;
        border: 1px solid rgba(156, 213, 255, 0.15);
        padding: 3rem 2rem;
        border-radius: 4px; 
        width: 100%; max-width: 340px;
        text-align: center;
        box-shadow: 0 25px 50px rgba(0,0,0,0.5);
        position: relative; 
        overflow: hidden;   
    }

    .status-badge {
        font-size: 10px; text-transform: uppercase; letter-spacing: 1.5px;
        color: #9CD5FF; background: rgba(156, 213, 255, 0.1);
        padding: 4px 12px; border-radius: 20px; margin-bottom: 1.5rem; display: inline-block;
    }

    .details-box {
        background: rgba(255, 255, 255, 0.05);
        padding: 20px;
        border-radius: 12px;
        margin: 25px 0;
        text-align: left;
        border: 1px solid rgba(156, 213, 255, 0.1); 
    }

    .item {
        margin-bottom: 15px;
    }

    .item:last-child {
        margin-bottom: 0;
    }

    .item span { 
    color: #616c75; 
    font-size: 11px; 
    text-transform: uppercase; 
    letter-spacing: 1px;
    display: block; 
    margin-bottom: 2px;
    }

    .item strong { 
    color: #ffffff; 
    font-size: 16px; 
    }

    .item small { 
    color: #9CD5FF; 
    font-size: 13px; 
    opacity: 0.9;
    }

    .action-btn {
        width: 100%; padding: 16px; margin-top: 10px;
        background: linear-gradient(90deg, #355872, #7AAACE);
        color: white; border: none; border-radius: 12px;
        font-weight: 600; cursor: pointer; transition: all 0.2s;
    }

    .text-btn {
        background: none;
        border: none;
        color: #ff6b6b;
        margin-top: 20px;
        cursor: pointer;
        font-size: 14px;
        font-weight: 500;
        text-decoration: none;
        transition: opacity 0.2s;
    }

    .text-btn:hover {
        opacity: 0.7;
    }

    .cancel-btn {
        position: absolute; bottom: 40px;
        background: rgba(255,255,255,0.1);
        color: white; border: none; padding: 12px 24px; border-radius: 30px; cursor: pointer;
    }

    /* SCANNER OVERLAY */
    .scan-overlay {
        position: fixed; 
        inset: 0; 
        background: rgba(8, 10, 12, 0.7); 
        display: flex; 
        flex-direction: column; 
        align-items: center; 
        justify-content: center;
        z-index: 100;
    }

    .scan-area { 
        position: relative; 
        width: 250px; 
        height: 250px; 
    }

    .corner { 
        position: absolute; 
        width: 30px; 
        height: 30px; 
        border: 4px solid #9CD5FF; 
    }

    .top-left { top: 0; left: 0; border-right: none; border-bottom: none; }
    .top-right { top: 0; right: 0; border-left: none; border-bottom: none; }
    .bottom-left { bottom: 0; left: 0; border-right: none; border-top: none; }
    .bottom-right { bottom: 0; right: 0; border-left: none; border-top: none; }

    .scan-line {
        position: absolute; 
        width: 100%; 
        height: 2px;
        background: #9CD5FF; 
        box-shadow: 0 0 15px #9CD5FF;
        animation: scanMove 2s infinite ease-in-out;
    }

    @keyframes scanMove { 
        0%, 100% { top: 5%; opacity: 0.2; } 
        50% { top: 95%; opacity: 1; } 
    }

    .hint { 
        margin-top: 30px; 
        color: #7AAACE; 
        font-size: 14px; 
    }

    .qr-icon svg { 
        width: 60px; 
        height: 60px; 
        color: #7AAACE; 
        margin-bottom: 1.5rem; 
        opacity: 0.5; 
    }

    .success-icon svg {
        width: 60px;
        height: 60px;
        margin-bottom: 1.5rem;
        filter: drop-shadow(0 0 12px rgba(74, 222, 128, 0.4));
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

    /*  DLA LAPTOPA */
    @media (hover: hover) {
        .action-btn:hover {
            transform: translateY(-2px); 
            box-shadow: 0 8px 25px rgba(122, 170, 206, 0.3);
            filter: brightness(1.1);
        }
        
        .action-btn:active {
            transform: none;
        }
    }

    /* LA TELEFONU */
    @media (pointer: coarse) {
        .action-btn:active {
            transform: scale(0.95); 
            filter: brightness(0.7);
            transition: transform 0.1s;
        }
        
        .action-btn:hover {
            transform: none;
            filter: none;
            box-shadow: none;
        }
    }

    .spinner-container {
        display: flex;
        justify-content: center;
        align-items: center;
        margin-top: 10px;
    }
    </style>
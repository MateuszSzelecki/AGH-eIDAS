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
        invoke("generate_proof", { challenge: result });
        result = {
            challengeId: "",
            nonce: "",
            timestamp: -1,
            callbackUrl: "",
            verifierName: "",
        };
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

{#if permission != "granted"}
    <p>No cammera premissions</p>
    <button onclick={requestCameraPermissions}>Request permissions</button>
{:else if scanning}
    <div class="overlay">
        <div class="frame"></div>
        <p>Point your camera at a barcode</p>
    </div>
{:else if result.callbackUrl}
    <div id="request">
        <p class="result">Result: {result.callbackUrl}</p>
        <button onclick={handleYes}>Yes</button>
        <button onclick={handleNo}>No</button>
    </div>
{:else}
    <button onclick={doScan}> Start Scan </button>
{/if}

<style>
    :root {
        margin: 0;
        padding: 0;
        background: transparent; /* CRITICAL */
    }

    button {
        margin-top: 200px;
        left: 20px;
        z-index: 10;
    }

    .overlay {
        position: fixed;
        inset: 0;
        display: flex;
        align-items: center;
        justify-content: center;

        background: rgba(0, 0, 0, 0.1);
        z-index: 5;
    }

    .frame {
        width: 250px;
        height: 250px;
        border: 3px solid #00ffcc;
        border-radius: 12px;
        box-shadow: 0 0 20px rgba(0, 255, 204, 0.6);
    }

    .overlay p {
        position: absolute;
        bottom: 40px;
        color: white;
        font-size: 16px;
    }

    .result {
        z-index: 10;
    }
</style>

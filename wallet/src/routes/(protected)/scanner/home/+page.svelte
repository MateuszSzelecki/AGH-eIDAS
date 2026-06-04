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

    $effect(() => {
        if (!document.identifier) {
            loadDocument();
        }
    });

    async function loadDocument() {
        document = await invoke("load_document");
    }

    async function requestDocument() {
        const issuerUrl = getIssuerUrl();
        document = await invoke("request_document", { issuerUrl });
    }

    function formatDate(timestamp: number) {
        if (!timestamp) return "-";
        return new Date(timestamp * 1000).toLocaleDateString();
    }
</script>

<h1>Home</h1>

{#if document.identifier}
    <div id="document">
        <p>ID: {document.identifier}</p>
        <p>First Name: {document.firstName}</p>
        <p>Last Name: {document.lastName}</p>
        <p>Date of Birth: {formatDate(document.dateOfBirth)}</p>
        <p>Issue Date: {formatDate(document.issueDate)}</p>
        <p>Expiry Date: {formatDate(document.expiryDate)}</p>
        <p style="word-break: break-all; font-size: 0.8em; margin-bottom: 5px;">Sig R: {document.sigR}</p>
        <p style="word-break: break-all; font-size: 0.8em; margin-top: 0;">Sig S: {document.sigS}</p>
    </div>
    <button onclick={requestDocument}>Refresh Document</button>
{:else}
    <p>No document found</p>
    <button onclick={requestDocument}>Request Document</button>
{/if}
<!-- TO DO show id info on home screen -->

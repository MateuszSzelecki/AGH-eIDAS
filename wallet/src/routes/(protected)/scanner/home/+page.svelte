<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";

    interface UserDocument {
        identifier: string;
        firstName: string;
        lastName: string;
        dateOfBirth: number;
        issueDate: number;
        expiryDate: number;
        signature: string;
    }

    let document: UserDocument = $state({
        identifier: "",
        firstName: "",
        lastName: "",
        dateOfBirth: 0,
        issueDate: 0,
        expiryDate: 0,
        signature: "",
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
        document = await invoke("request_document");
    }
</script>

<h1>Home</h1>

{#if document.identifier}
    <div id="document">
        <p>ID: {document.identifier}</p>
        <p>First Name: {document.firstName}</p>
        <p>Last Name: {document.lastName}</p>
        <p>Date of Birth: {document.dateOfBirth}</p>
        <p>Issue Date: {document.issueDate}</p>
        <p>Expiry Date: {document.expiryDate}</p>
        <p>Signature: {document.signature}</p>
    </div>
{:else}
    <p>No document found</p>
    <button onclick={requestDocument}>Request Document</button>
{/if}
<!-- TO DO show id info on home screen -->

<script lang="ts">
    import { goto } from "$app/navigation";
    import { page } from "$app/state";

    const items = [
        { label: "Home", path: "/scanner/home", icon: "🏠" },
        { label: "Scan", path: "/scanner", icon: "📷" },
        { label: "Settings", path: "/settings", icon: "⚙️" },
    ];

    function navigate(path: string) {
        goto(path);
    }
</script>

<nav class="bottom-bar">
    {#each items as item}
        <button
            class:selected={page.url.pathname === item.path}
            onclick={() => navigate(item.path)}
        >
            <span class="icon">{item.icon}</span>
            <span class="label">{item.label}</span>
        </button>
    {/each}
</nav>

<style>
    .bottom-bar {
        position: fixed;
        bottom: 0;
        left: 0;
        right: 0;

        display: flex;
        justify-content: space-around;
        align-items: center;

        height: 64px;
        background: #111;
        border-top: 1px solid #222;

        z-index: 1000;
    }

    button {
        all: unset;
        flex: 1;

        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;

        color: #888;
        font-size: 12px;
        cursor: pointer;
    }

    .icon {
        font-size: 20px;
    }

    button.selected {
        color: white;

        transform: translateY(-5px);
        background: #222;
        border-radius: 12px;
        padding: 6px 12px;
    }
</style>

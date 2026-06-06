<script lang="ts">
    import { goto } from "$app/navigation";
    import { page } from "$app/state";

    // Używamy gotowych ścieżek SVG zamiast emoji
    const items = [
        { 
            label: "Home", 
            path: "/scanner/home", 
            icon: `<path stroke-linecap="round" stroke-linejoin="round" d="M2.25 12l8.954-8.955c.44-.439 1.152-.439 1.592 0L21.75 12M4.5 9.75v10.125c0 .621.504 1.125 1.125 1.125H9.75v-4.875c0-.621.504-1.125 1.125-1.125h2.25c.621 0 1.125.504 1.125 1.125V21h4.125c.621 0 1.125-.504 1.125-1.125V9.75M8.25 21h8.25" />` 
        },
        { 
            label: "Scan", 
            path: "/scanner", 
            icon: `<path stroke-linecap="round" stroke-linejoin="round" d="M6.827 6.175A2.31 2.31 0 015.186 7.23c-.38.054-.757.112-1.134.175C2.999 7.58 2.25 8.507 2.25 9.574V18a2.25 2.25 0 002.25 2.25h15A2.25 2.25 0 0021.75 18V9.574c0-1.067-.75-1.994-1.802-2.169a47.865 47.865 0 00-1.134-.175 2.31 2.31 0 01-1.64-1.055l-.822-1.316a2.192 2.192 0 00-1.736-1.039 48.774 48.774 0 00-5.232 0 2.192 2.192 0 00-1.736 1.039l-.821 1.316z" /><path stroke-linecap="round" stroke-linejoin="round" d="M16.5 12.75a4.5 4.5 0 11-9 0 4.5 4.5 0 019 0zM18.75 10.5h.008v.008h-.008V10.5z" />` 
        },
        { 
            label: "Settings", 
            path: "/settings", 
            icon: `<path stroke-linecap="round" stroke-linejoin="round" d="M10.343 3.94c.09-.542.56-.94 1.11-.94h1.093c.55 0 1.02.398 1.11.94l.149.894c.07.424.384.764.78.93.398.164.855.142 1.205-.108l.737-.527a1.125 1.125 0 011.45.12l.773.774c.39.389.44 1.002.12 1.45l-.527.737c-.25.35-.272.806-.107 1.204.165.397.505.71.93.78l.893.15c.543.09.94.56.94 1.109v1.094c0 .55-.397 1.02-.94 1.11l-.893.149c-.425.07-.765.383-.93.78-.165.398-.143.854.107 1.204l.527.738c.32.447.269 1.06-.12 1.45l-.774.773a1.125 1.125 0 01-1.449.12l-.738-.527c-.35-.25-.806-.272-1.203-.107-.397.165-.71.505-.781.929l-.149.894c-.09.542-.56.94-1.11.94h-1.094c-.55 0-1.019-.398-1.11-.94l-.148-.894c-.071-.424-.384-.764-.781-.93-.398-.164-.854-.142-1.204.108l-.738.527c-.447.32-1.06.269-1.45-.12l-.773-.774a1.125 1.125 0 01-.12-1.45l.527-.737c.25-.35.273-.806.108-1.204-.165-.397-.505-.71-.93-.78l-.894-.15c-.542-.09-.94-.56-.94-1.109v-1.094c0-.55.398-1.02.94-1.11l.894-.149c.424-.07.765-.383.93-.78.165-.398.143-.854-.107-1.204l-.527-.738a1.125 1.125 0 01.12-1.45l.773-.773a1.125 1.125 0 011.45-.12l.737.527c.35.25.807.272 1.204.107.397-.165.71-.505.78-.929l.15-.894z" /><path stroke-linecap="round" stroke-linejoin="round" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />` 
        },
    ];

    // Aktualnie otwarta strona (żeby wiedzieć, co podświetlić, gdy myszka ucieknie)
    let activeIndex = $derived(
        Math.max(0, items.findIndex(item => page.url.pathname === item.path))
    );

    // Indeks elementu, na który najeżdża myszka (null, jeśli myszka nie jest nad navbarem)
    let hoveredIndex = $state<number | null>(null);

    // Obliczamy, gdzie ma być podświetlenie (hover priorytetem, inaczej active)
    let currentIndex = $derived(hoveredIndex !== null ? hoveredIndex : activeIndex);

    function navigate(path: string) {
        goto(path);
    }
</script>

<div class="nav-container">
    <!-- onmouseleave na całym navbare resetuje hover -->
    <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
    <nav class="navbar" onmouseleave={() => hoveredIndex = null}>
        <ul>
            <div class="active-pill" style="transform: translateX(calc({currentIndex} * 100%));"></div>
            
            {#each items as item, i}
                <!-- onmouseenter na konkretnym <li> zmienia pozycję podświetlenia -->
                <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
                <li onmouseenter={() => hoveredIndex = i}>
                    <button
                        class:active={activeIndex === i}
                        onclick={() => navigate(item.path)}
                    >
                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="nav-icon">
                            {@html item.icon}
                        </svg>
                        <span class="nav-label">{item.label}</span>
                    </button>
                </li>
            {/each}
        </ul>
    </nav>
</div>

<style>
    .nav-container {
        position: fixed;
        bottom: 30px;
        left: 0;
        width: 100%;
        display: flex;
        justify-content: center;
        z-index: 1000;
        pointer-events: none;
    }
    
    .navbar {
        pointer-events: auto; 
        padding: 8px;
        background: rgba(15, 20, 25, 0.7); 
        border-radius: 60px;
        backdrop-filter: blur(20px);
        -webkit-backdrop-filter: blur(20px);
        box-shadow: 0 15px 35px rgba(0, 0, 0, 0.6), inset 0 0 0 1px rgba(255, 255, 255, 0.05);
        width: 90%;
        max-width: 400px; 
    }

    ul {
        position: relative;
        list-style: none;
        display: flex;
        margin: 0;
        padding: 0;
        width: 100%;
    }

    .active-pill {
        position: absolute;
        top: 0;
        left: 0;
        width: 33.333%; 
        height: 100%;
        background: rgba(156, 213, 255, 0.15);
        box-shadow: 0 0 20px rgba(156, 213, 255, 0.3), inset 0 0 8px rgba(156, 213, 255, 0.2);
        border-radius: 40px;
        /* To odpowiada za płynne przesunięcie! */
        transition: transform 0.35s cubic-bezier(0.4, 0, 0.2, 1);
        z-index: 0;
    }

    li {
        margin: 0;
        flex: 1; 
        z-index: 1; 
    }
    
    button {
        all: unset; 
        display: flex;
        align-items: center;
        justify-content: center;
        gap: 8px; /* Odstęp między ikoną a tekstem */
        box-sizing: border-box;
        width: 100%;
        color: #758a9a; 
        font-family: 'Inter', sans-serif;
        padding: 12px 0;
        border-radius: 40px;
        cursor: pointer;
        transition: color 0.3s ease;
    }

    .nav-icon {
        width: 20px;
        height: 20px;
        transition: transform 0.3s ease;
    }

    .nav-label {
        font-size: 14px;
        font-weight: 500;
    }

    button:hover {
        color: #b0cce4;
    }
    
    button:hover .nav-icon {
        transform: scale(1.1); /* Lekkie powiększenie ikony na hover */
    }
    
    button.active {
        color: #ffffff; 
        font-weight: 600;
        text-shadow: 0 0 10px rgba(255, 255, 255, 0.3);
    }
</style>
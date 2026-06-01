
<script>
    import { login, user } from "$lib/auth.svelte";
    let username = $state("");
    let password = $state("");
    let loading = $state(false);

    async function handleLogin() {
        loading = true;
        const result = await login(username, password); // dodałem await na wypadek gdyby login był asynchroniczny
        loading = false;
    }
</script>

<div class="app-container">
    <div class="form-card">
        <!-- TWOJE ELEMENTY HUD / SCI-FI -->
        <div class="top-bar"></div>
        <div class="side-trapezoid left"></div>
        <div class="side-trapezoid right"></div>
        <div class="bottom-triangle left"></div>
        <div class="bottom-triangle right"></div>
        
        <h2>eIDAS Wallet</h2>
        <p class="subtitle">Zaloguj się do swojego portfela</p>

        <div class="input-group">
            <div class="input-wrapper">
                <input type="text" placeholder="Nazwa użytkownika" bind:value={username} />
            </div>
            <div class="input-wrapper">
                <input type="password" placeholder="Hasło" bind:value={password} />
            </div>
        </div>

        <button onclick={handleLogin} disabled={loading} class="login-btn">
            {loading ? "Weryfikacja..." : "Zaloguj się"}
        </button>
        
        <div class="footer">
            <p>Nie masz konta?</p>
            <a href="/register">Utwórz nowe konto</a>
        </div>
    </div>
</div>


<style>


@font-face {
  font-family: 'Coolvetica';
  src: url('/fonts/coolvetica.otf') format('opentype');
  font-weight: normal;
  font-style: normal;
}

@import url('https://fonts.googleapis.com/css2?family=Inter:wght@300;400;600;700&display=swap');

  :global(body) {
    margin: 0;
    padding: 0;
    min-height: 100vh;
    font-family: 'Inter', -apple-system, sans-serif;
    display: flex;
    justify-content: center;
    align-items: center;
  
    background: 
      radial-gradient(circle at center, #597aa0 0%, #1b232b 100%);
    background-size: cover;
    background-attachment: fixed;

    /* SPECJALNIE DLA TELEFONU (ekrany węższe niż 768px) */
  @media (max-width: 768px) {
    :global(body) {
      /* Bardziej zbity, mniejszy gradient na środku telefonu */
      background: radial-gradient(circle at center, #597aa0 0%, #080a0c 70%);
    }
  

  }
}

  .app-container {
    display: flex;
    justify-content: center;
    align-items: center;
    width: 100%;
    height: 100%;
  }

  .form-card {
    /* Twoje kolory + radialny gradient jako światło u góry */
    background: 
      radial-gradient(circle at 50% -10%, rgba(156, 213, 255, 0.2) 0%, transparent 60%),
      #0f1419;
    backdrop-filter: blur(20px);
    padding: 3.5rem 2.5rem;
    border-radius: 4px;
    width: 100%;
    max-width: 340px;
    text-align: center;
    position: relative;
    border: 1px solid rgba(156, 213, 255, 0.15);
    box-shadow: 0 20px 60px rgba(0, 0, 0, 0.8);
    overflow: hidden;


    /* Ustawienia dla LAPTOPA */
    width: 340px; 
    min-height: auto; 
    transition: all 0.3s ease;
  }

  /* SPECJALNIE DLA TELEFONU */
  @media (max-width: 768px) {
    .form-card {
      width: 85%; /* Węższa w stosunku do ekranu */
      max-width: 300px;
      min-height: 55vh; /* Wyższa, żeby wyglądała smuklej */
      display: flex;
      flex-direction: column;
      justify-content: center; /* Wyśrodkowuje zawartość wewnątrz wysokiej karty */
      padding: 4rem 1.5rem;
    }


  }

  /* --- KSZTAŁTY HUD / SCI-FI --- */

  .top-bar {
    position: absolute;
    top: -2px; left: 50%;
    transform: translateX(-50%);
    width: 100px; height: 3px;
    background: #9CD5FF;
    box-shadow: 0 0 12px #9CD5FF;
  }

  .side-trapezoid.left {
    left: 0; top: 15%; height: 70%; width: 4px;
    background: rgba(156, 213, 255, 0.3);
    /* To robi ukośne ścięcie na końcach linii */
    clip-path: polygon(0 0, 100% 5%, 100% 95%, 0 100%);
  }
  .side-trapezoid.right {
    right: 0; top: 15%; height: 70%; width: 4px;
    background: rgba(156, 213, 255, 0.3);
    clip-path: polygon(0 5%, 100% 0, 100% 100%, 0 95%);
  }

  h2 {
    font-family: 'Coolvetica', sans-serif;
    font-weight: 400;
    font-size: 2.4rem;
    color: #ffffff;
    margin: 0;
    letter-spacing: 1px; /* Rozluźnione litery */
    /* Usuwamy wszelkie cienie/triangles, które mogły tu być */
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

  .login-btn {
    width: 100%;
    padding: 16px;
    /* Gradient jak na przycisku "Confirm swap" */
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

  .login-btn:hover:not(:disabled) {
    transform: translateY(-2px);
    box-shadow: 0 8px 25px rgba(122, 170, 206, 0.3);
    filter: brightness(1.1);
  }

  .footer {
    margin-top: 2rem;
    font-size: 0.85rem;
  }

  .footer p { color: #5c6a75; margin-bottom: 0.5rem; }
  
  a {
    color: #9CD5FF;
    text-decoration: none;
    font-weight: 500;
    transition: color 0.2s;
  }

  a:hover { color: #ffffff; }

  .spinner {
    display: inline-block;
    width: 14px;
    height: 14px;
    border: 2px solid rgba(255,255,255,0.2);
    border-radius: 50%;
    border-top-color: #fff;
    animation: spin 0.8s linear infinite;
    margin-right: 10px;
  }
  @keyframes spin { to { transform: rotate(360deg); } }
</style>

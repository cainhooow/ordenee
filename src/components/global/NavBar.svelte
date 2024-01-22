<script lang="ts">
    import { page } from "$app/stores";

    function currentRoute(route: String) {
        return route == $page.route.id
    };

</script>

<nav class="navbar">
    <section class="app-title">
        <a href="/">
            <h1>Ordenee</h1>
        </a>
    </section>
    <section class="app-discover">
        <a class="{currentRoute('/clientes') ? "active" : "" }" href="/clientes">
            Clientes
        </a>
        <a class="{currentRoute('/tecnicos') ? "active" : "" }" href="/tecnicos">
            Tecnicos
        </a>
        <a class="{currentRoute('/equipamentos') ? "active" : "" }" href="/equipamentos">
            Equipamentos
        </a>
        <a class="{currentRoute('/ordens') ? "active" : "" }" href="/ordens">
            Ordens
        </a>
        <a class="{currentRoute('/empresa') ? "active" : "" }" href="/empresa">
            Empresa
        </a>
    </section>
</nav>

<style lang="scss">
    .navbar {
        display: flex;
        align-items: center;
        justify-content: space-between;

        background-color: var(--primary);
        padding: 1.5rem;
    }

    .navbar
    .app-title {
        font-size: 1.6rem;
    }

    .navbar
    .app-discover {
        margin: .5rem;
        position: relative;
    }
    .navbar
    .app-discover a {
        margin: .5rem;
        position: relative;
        z-index: 1;
        background-color: var(--primary);
    }

    .navbar
    .app-discover a.active {
        background-color: var(--white);
        border-radius: .3rem;
        color: var(--primary);
        padding: .5rem;
        box-shadow: 1px 6px 15px 0.1px #983dff8f;
        transition: all 50ms ease-in-out;
    }

    .navbar
    .app-discover a.active:before {
        content: "";
        position: absolute;
        width: 100%;
        height: .1rem;   
        left: 0;
        bottom: -.5rem;
        z-index: 0;
        transition: all 200ms ease-in-out;
        z-index: 0;
    }

    .navbar
    .app-discover a:before {
        content: "";
        position: absolute;
        background-color: var(--thirdary);
        width: 0;
        height: .1rem;   
        left: 0;
        bottom: -.5rem;
        z-index: 0;
        transition: all 200ms ease-in-out;
        z-index: 0;
    }


    .navbar
    .app-discover a:hover::before {
        width: 100%
    }
    
</style>
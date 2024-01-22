<script lang="ts">
    import NavBar from "../components/global/NavBar.svelte";
</script>

<main>
    <section class="container app-presentation">
        <img src="https://imgs.search.brave.com/slGWKEt6NESzb0I9OeIUX_gRN2p5ZHGA9fcWIas8s_g/rs:fit:860:0:0/g:ce/aHR0cHM6Ly9pbWcu/ZnJlZXBpay5jb20v/cHJlbWl1bS1wc2Qv/YmVhdXRpZnVsLWxh/cHRvcC1zY3JlZW4t/bW9ja3VwLWFkLXdp/dGgtcHJlc2VudGF0/aW9uLXNsaWRlc181/Mzg3Ni05ODc5MC5q/cGc_c2l6ZT02MjYm/ZXh0PWpwZw" alt="presentation">
        <div class="pre-bg-metadata">
            <section class="pre-metadata">
                <h1>Olá, seja bem-vindo ao Ordenee!</h1>
                <p>Obrigado por utilizar nossos serviços. O Ordenee foi feito para gerenciar seus
                    serviços, clientes e a sua empresa!
                    <br><br>
                    Aqui está um guia de começo: <br><br>
                    - Optar primeiramente em configurar os tecnicos, formas de pagamento e equipamentos, clientes são
                    adicionados quando uma nova ordem é criada;<br><br>
                    - Ordens: Todas as ordens podem ser encontradas na Tab "Ordens";<br><br>
                    - Empresa: O sistema pode ser modificado de acordo com a sua empresa, você pode
                    adicionar sua logo e informações adicionais.
                </p>
            </section>
        </div>   
    </section>
    <section class="container app-fastaccess">
        <section class="app-fastaccess-header">
            <i class="ri-menu-line"></i>
            <h1>Acesso rapido</h1>
        </section>
        <section class="app-fastaccess-items">
            <a href="/clientes" type="button">
                <i class="ri-team-fill"></i>
                Clientes
            </a>
            
            <a href="/tecnicos" type="button">
                <i class="ri-team-fill"></i>
                Tecnicos
            </a>

            <a href="/equipamentos/new-equipament" type="button">
                <i class="ri-briefcase-2-line"></i>
                Adicionar equipamento
            </a>
            
            <a href="/equipamentos" type="button">
                <i class="ri-briefcase-2-line"></i>
                Equipamentos
            </a>
            
            <a href="/pagform" type="button">
                <i class="ri-bank-card-line"></i>
                Formas de pagamento
            </a>

            <a href="/ordens/new-order" type="button">
                <i class="ri-article-line"></i>
                Nova OS
            </a>
    
            <a href="/ordens" type="button">
                <i class="ri-article-line"></i>
                Ordens
            </a>
    
            <a href="/empresa" type="button">
                <i class="ri-shake-hands-line"></i>
                Empresa
            </a>
        </section>
    </section>
</main>

<style lang="scss">
    .app-presentation {
        position: relative;
        margin: 1.5rem .8rem;
        overflow: hidden;
    }
    .app-presentation img {
        width: 100%;
        height: 380px;
        border-radius: .5rem;
        object-fit: cover;
    }

    .app-presentation 
    .pre-bg-metadata {
        position: absolute;
        display: flex;
        align-items: end;
        background: linear-gradient(10deg, #000, #00000054);
        border-radius: .5rem;
        width: 100%;
        height: 380px;
        top: 0;
        bottom: 0;
    }

    .app-presentation 
    .pre-bg-metadata
    .pre-metadata {
        display: flex;
        width: 80%;
        flex-direction: column;
        padding: 1.2rem;
        color: var(--white)
    }

    .pre-metadata h1 {
        font-size: 1.5rem;
    }

    .pre-metadata p {
        font-size: 1rem;
        margin: .5rem 0;
    }

    .app-fastaccess-header {
        display: flex;
        align-items: center;
        padding: .5rem 1.5rem;
        gap: .5rem;
        background-color: var(--primary);
        border-radius: .5rem .5rem 0 0;
    }

    .app-fastaccess-header h1, i {
        font-size: 1.5rem;
    }

    .app-fastaccess {
        display: flex;
        width: 100%;
        flex-direction: column;
        padding: .5rem .9rem;
    }

    .app-fastaccess-items {
        display: flex;
        width: 100%;
        flex-direction: row;
        flex-wrap: wrap;
        flex-basis: 100px;
        padding: 1rem;
        background: linear-gradient(190deg, var(--primary-opacity-1), var(--secondary));
    }

    .app-fastaccess-items a[type="button"] {
        background: #fff;
        display: flex;
        gap: .5rem;
        align-items: center;
        color: var(--primary);
        padding: .5rem 1.5rem;
        margin: .5rem;
        border-radius: .5rem;
        cursor: pointer;
        border: none;
        transition: all 100ms ease-in;
    }

    .app-fastaccess-items a i {
        font-size: 1rem;
    }


    .app-fastaccess-items a[type="button"]:hover {
        background-color: var(--primary);
        color: var(--white);
    }
</style>
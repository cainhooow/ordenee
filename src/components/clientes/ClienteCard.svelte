<script lang="ts">
    export let cliente: any = {};
    console.log(cliente)

</script>


<style lang="scss">
    .cliente-card {
        position: relative;
        padding: 1.5rem;
        background: var(--primary);
        border-radius: .5rem;
        transition: all 200ms ease-in-out;
        border-left: 0.5px solid var(--secondary);
        z-index: 1;
        overflow: hidden;
    }

    .cliente-card:before {
        content: "";
        position: absolute;
        width: 10%;
        transform: skewX(-20deg) translateY(200px);
        height: 250px;
        top: 0;   
        right: 3rem;
        transition: all 200ms ease-in-out;
        background-color: var(--white);
        z-index: 0;
    }

    .cliente-card:hover::before {
        transform: skewX(-20deg) translateY(0);
    }

    .cliente-card:hover {
        border-left-color: var(--white);
        border-radius: 0 .5rem .5rem 0;
    }

    .cliente-card
    .cliente-header {
        display: flex;
        flex-direction: column;
        // padding: 1.5rem;
    }

    .cliente-header h1 {
        font-size: 1.5rem;
    }

    .cliente-header span {
        color: darkgray;
    }

    .cliente-info {
        display: flex;
        align-items: center;
        gap: 1.5rem;
    }

    .cliente-profile {
        display: flex;
        align-items: center;
        justify-content: center;
        width: 50px;
        height: 50px;
        background-color: rgb(255, 138, 95);
        border-radius: 50%;
    }
</style>

<section class="cliente-card">
    <div class="cliente-header">
        <div class="cliente-info">
            <h1 class="cliente-profile">{cliente.name.slice()[0]}</h1>
            <h1>{cliente.name}</h1>
        </div>
        <span class="pt-1.5">Criado em: {new Date(cliente.createdAt).toLocaleDateString("pt-br")}
              as {new Date(cliente.createdAt).toLocaleTimeString("pt-br")}
        </span>
    </div>
    <div class="cliente-actions pt-1.5">
        <button>
            <i class="ri-delete-bin-line"></i>
            Deletar
        </button>
        <button>
            <i class="ri-edit-circle-line"></i>
            Editar
        </button>
    </div>
</section>

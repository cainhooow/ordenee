<script lang="ts">
    import { page } from "$app/stores";
	import type { Payment } from "../../../types/Payment";

    export let data;

    const method = data.method;
</script>

<h1>{method?.name}</h1>
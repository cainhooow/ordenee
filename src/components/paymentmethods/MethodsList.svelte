<script lang="ts" context="module">
	function gotoPaymentInfo(ev: MouseEvent, methodId: number) {
		ev.preventDefault();
		window.location.href = `/paymentmethods/methods?method=${methodId}`;
	}
</script>

<script lang="ts">
	import type { Payment } from '../../types/Payment';
	export let method = {} as Payment;
</script>

<button
	class="flex border rounded items-start p-2 bg-zinc-700/10 border-zinc-700 hover:border-zinc-600 transition-all ease-in-out delay-[10ms] hover:drop-shadow-[0_0_100px_rgb(10,200,55,1)]"
	tabindex={method.id}
	on:click={(ev) => gotoPaymentInfo(ev, method.id)}
>
	<h3>{method.name}</h3>
</button>

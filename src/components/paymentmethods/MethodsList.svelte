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
	class="flex border rounded items-start p-2 bg-zinc-700/10 hover:bg-zinc-700/50 border-zinc-700 hover:border-zinc-600 transition-all ease-in-out delay-[10ms]"
	tabindex={method.id}
	on:click={(ev) => gotoPaymentInfo(ev, method.id)}
>
	<h3>{method.name}</h3>
</button>

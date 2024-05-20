<script lang="ts">
	import Footer from '../components/global/Footer.svelte';
	import NavBar from '../components/global/NavBar.svelte';

	import '../app.css';
	import Window from '../components/global/Window.svelte';
</script>

<Window />
<main class="overflow-y-scroll h-screen">
	<slot />
</main>

<style lang="scss" global>
	@import '../styles/global.scss';
</style>

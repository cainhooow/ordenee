<script lang="ts">
	import Footer from '../components/global/Footer.svelte';
	import NavBar from '../components/global/NavBar.svelte';
</script>

<NavBar />
<slot />

<style lang="scss" global>
	@import '../styles/global.scss';
</style>

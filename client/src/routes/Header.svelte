<script lang="ts">
	import { page } from "$app/stores";
	import { APIClient } from "$lib/ApiClient";

	let api = new APIClient();
	async function logout() {
		await api.userLogout(""?? "");
	}
</script>

<header>
	<div></div>

	<div class="corner">
		{#if $page.data.user}
			<a href="/user">{$page.data.user.email}</a>
			<a href="#" on:click={logout}>Logout</a>
		{:else}
			<a href="/login"> Login </a>
		{/if}
	</div>
</header>

<style>
	header {
		display: flex;
		justify-content: space-between;
	}

	.corner {
		position: relative;
		top: 10px;
		width: 8em;
		height: 3em;
	}

	a:hover {
		color: var(--color-theme-1);
	}
</style>

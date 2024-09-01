<script lang="ts">
	import { page } from "$app/stores";

	function logout() {
		fetch("/api/logout", {
			method: "POST"
		});
	}
</script>

<header>
	<div></div>

	<div class="corner">
		{#if $page.data.user}
			<a href="/user">{$page.data.user.user_name}</a>
			<a href="#" >Logout</a>
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
		width: 5em;
		height: 3em;
	}

	a:hover {
		color: var(--color-theme-1);
	}
</style>

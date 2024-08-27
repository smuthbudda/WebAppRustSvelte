<script lang="ts">
    import { APIClient } from "$lib/ApiClient";
    import { userDetails } from "$lib/store"

    let isLogin = true;
    let apiClient = new APIClient();
    let firstName = "";
    let lastName = "";
    let email = "";
    let phone = "";
    let password = "";
    let username = "";
    let currentError = "";

    async function login() {
        let token = await apiClient.userLogin(username, password);
        sessionStorage.setItem("token", token[1].access_token);
        let result  = await apiClient.getMyDetails(token[1].access_token);
        console.log(result);
        userDetails.setName(result[1].first_name)
        userDetails.setId(result[1].id)
    }

</script>
<input type="checkbox" bind:value={isLogin}/>
{#if isLogin}
    <form on:submit|preventDefault={login}>
        <div class="login">
            <label for="username">Username</label>
            <input type="text" id="username" bind:value={username} />
        </div>
        <div class="login">
            <label for="password">Password</label>
            <input type="password" id="password" bind:value={password} />
        </div>
        <div class="submit">
            <button type="submit">Login</button>
        </div>
        <div>
            <small>{currentError}</small>
        </div>
    </form>
{:else}
    <form on:submit|preventDefault={login}>
        <div class="login">
            <label for="first_name">First Name</label>
            <input type="text" id="first_name" bind:value={firstName} />
        </div>
        <div class="login">
            <label for="last_name">lastName</label>
            <input type="text" id="last_name" bind:value={lastName} />
        </div>
        <div class="login">
            <label for="email">email</label>
            <input type="text" id="email" bind:value={email} />
        </div>
        <div class="login">
            <label for="phone">phone</label>
            <input type="text" id="phone" bind:value={phone} />
        </div>
        <div class="login">
            <label for="username">Username</label>
            <input type="text" id="username" bind:value={email} />
        </div>
        <div class="login">
            <label for="password">Password</label>
            <input type="password" id="password" bind:value={password} />
        </div>
        <div class="submit">
            <button type="submit">Login</button>
        </div>
        <div>
            <small>{currentError}</small>
        </div>
    </form>
{/if}

<style>
    form {
        padding: 2rem;
        display: flex;
        flex-direction: column;
        gap: 1rem;
    }

    .login {
        display: flex;
        justify-content: center;
        flex-direction: column;
    }

    .login > input {
        padding: 0.2rem;
    }

    .submit {
        display: flex;
        justify-content: center;
    }
</style>

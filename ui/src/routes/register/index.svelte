<script context="module">
    export async function load({ session }) {
        if (session.user) {
            return {
                status: 302,
                redirect: "/questions",
            };
        }

        return {};
    }
</script>

<script>
    import { post } from "$lib/api.js";
    import ListErrors from "$lib/ListErrors.svelte";
    import Pagination from "$lib/Pagination.svelte";

    let username = "";
    let email = "";
    let password = "";
    let confirm_password = "";
    let errors = null;

    async function check_username_availablity() {
        if (username.length < 3) {
            return;
        } else {
            const response = await post("check-username-availability", {
                username,
            });
            errors = response.erros;

            if (response.data.success) {
                M.toast({ html: "Username available!" });
            } else {
                M.toast({ html: "Username unavailable" });
            }
        }
    }
    async function submit() {
        if (username.length < 3) {
            M.toast({ html: "Username must be at least 3 character." });
            return;
        }
        if (
            password !== confirm_password &&
            (password.length < 16 || confirm_password.length)
        ) {
            M.toast({
                html: "Passphrases must be at least 16 characters and be same.",
            });
            return;
        }
        const response = await post(`register`, {
            username,
            email,
            password,
            confirm_password,
        });

        if (response.code === 200) {
            M.toast({
                html: "You have been sent a confirmation email. Please verify!",
            });
        } else {
            M.toast({
                html: response.msg,
            });
        }
    }
</script>

<svelte:head>
    <title>Register ❤ Kunjika</title>
</svelte:head>

<main>
    <div class="row">
        <div class="col-md-6 offset-md-3 col-xs-12">
            <h3 class="text-xs-center">Register</h3>
            <p class="text-xs-center" style="margin-left:20px">
                <a href="/login">Have an account?</a>
            </p>

            <ListErrors {errors} />

            <form on:submit|preventDefault={submit} class="col s6">
                <div class="row">
                    <div class="input-field col s12">
                        <input
                            class="validate"
                            type="text"
                            required
                            bind:value={username}
                            on:keyup={check_username_availablity}
                            id="username"
                        />
                        <label for="username">Username</label>
                    </div>
                    <div class="input-field col s12">
                        <input
                            class="validate"
                            type="email"
                            required
                            bind:value={email}
                            minlength="6"
                            maxlength="256"
                        />
                        <label for="email">Email</label>
                    </div>
                    <div class="input-field col s12">
                        <input
                            class="validate"
                            type="password"
                            required
                            bind:value={password}
                            minlength="16"
                            maxlength="64"
                        />
                        <label for="password">Passphrase</label>
                    </div>
                    <div class="input-field col s12">
                        <input
                            class="validate"
                            type="password"
                            required
                            bind:value={confirm_password}
                            minlength="16"
                            maxlength="64"
                        />
                        <label for="password">Confirm Passphrase</label>
                    </div>
                </div>
                <div class="b-wrapper">
                    <button class="btn" type="submit" name="action"
                        >Submit
                        <i class="material-icons right">send</i>
                    </button>
                </div>
            </form>
        </div>
    </div>
</main>

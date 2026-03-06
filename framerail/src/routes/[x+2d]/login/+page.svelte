<script lang="ts">
  import { page } from "$app/state"
  import { invalidateAll } from "$app/navigation"
  import { errorPopupState } from "$lib/stores.svelte"
  import { superForm } from "sveltekit-superforms"
  import { untrack } from "svelte"

  import type { PageProps } from "./$types"

  let isLoggedIn = $state<boolean>(page.data.isLoggedIn)

  let { data }: PageProps = $props()

  const { form, enhance } = superForm(
    untrack(() => data.loginForm),
    {
      onResult: async ({ result }) => {
        if (result.type === "success" && result.data?.isLoggedIn) {
          isLoggedIn = true
          await invalidateAll()
          return
        }

        if (result.type === "failure" && result.data?.loginError) {
          errorPopupState.current = {
            state: true,
            message: result.data.error.message,
            data: result.data.error.data
          }
        }
      }
    }
  )
</script>

{#if isLoggedIn}
  {page.data.internationalization?.["login.toast"]}
{:else}
  <form id="login" class="login-form" method="POST" use:enhance>
    <input
      name="nameOrEmail"
      class="auth-name-or-email"
      placeholder={page.data.internationalization?.specifier}
      type="text"
      bind:value={$form.nameOrEmail}
    />
    <input
      name="password"
      class="auth-password"
      placeholder={page.data.internationalization?.password}
      type="password"
      bind:value={$form.password}
    />
    <div class="action-row auth-actions">
      <button class="action-button auth-button button-cancel clickable" type="button">
        {page.data.internationalization?.cancel}
      </button>
      <button class="action-button auth-button button-login clickable" type="submit">
        {page.data.internationalization?.login}
      </button>
    </div>
  </form>
{/if}

<style lang="scss">
  .login-form {
    display: flex;
    flex-direction: column;
    gap: 1em;
    align-items: center;
    justify-content: center;
  }
</style>

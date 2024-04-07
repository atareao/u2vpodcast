<script lang="ts">
    import { Button, Modal, Label, Input, Checkbox } from 'flowbite-svelte';
    import { DarkMode, Navbar, NavBrand, NavLi, NavUl, NavHamburger } from 'flowbite-svelte'
    import "../app.css";
    import logo from '$lib/assets/favicon/favicon-48x48.png';
    import LoginForm from '$lib/components/LoginForm.svelte';

    let btnClass = "text-primary-500 dark:text-primary-600 border dark:border-gray-800";
    let loginFormShow = false;
</script>
<Navbar class="mb-8" let:hidden let:toggle>
  <NavBrand href="/">
    <img
      src={logo}
      class="mr-3 h-6 sm:h-9"
      alt="U2VPodcast"
    />
    <span class="self-center whitespace-nowrap text-xl font-semibold dark:text-white">
    U2VPodcast
    </span>
  </NavBrand>
  <NavHamburger on:click={toggle} />
  <NavUl {hidden}>
    <NavLi href="/" active={true}>Home</NavLi>
    <NavLi href="/about">About</NavLi>
    <NavLi href="/services">Services</NavLi>
    <NavLi href="/pricing">Pricing</NavLi>
    <NavLi href="/contact">Contact</NavLi>
    <Button on:click={() => (loginFormShow = true)}>Login</Button>
    <DarkMode {btnClass} />
  </NavUl>
</Navbar>
<slot />
<LoginForm bind:open={loginFormShow} />

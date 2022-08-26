<script lang="ts">
  import { defineComponent } from "vue";
  import NavBar from "./components/NavBar.vue"
  export default defineComponent ({
    name: "App",
    components: {
      NavBar
    },
  });
</script>

<template>
  <header>

  </header>

  <main>
    <h1> Here is a component: </h1>
    <NavBar />
    <h1> Here's another </h1>
  </main>
</template>


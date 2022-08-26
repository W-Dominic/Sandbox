<script lang="ts">

import { defineComponent } from 'vue'

export default defineComponent({
  data() {
    return {
      count: 0
    }
  }
})

</script>

<template>
  <h2> Navbar Component </h2>
  <button @click="count++">You clicked me {{ count }} times.</button>
</template>
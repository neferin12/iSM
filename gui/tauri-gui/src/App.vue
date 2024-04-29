<script setup lang="ts">
import {RouterLink, RouterView} from 'vue-router'
import {BNavbar} from "bootstrap-vue-next";
</script>

<template>
  <BNavbar data-tauri-drag-region id="navbar" variant="primary" v-b-color-mode="'dark'">
    <BNavbarBrand tag="h1" class="mb-0">RiSM GUI</BNavbarBrand>
    <BCollapse id="nav-collapse" is-nav>
      <BNavbarNav>
        <BNavItem to="/">New calculation</BNavItem>
        <BNavItem to="/about">About</BNavItem>
      </BNavbarNav>
    </BCollapse>

  </BNavbar>
  <main class="content">
    <div>
      <b-card>
        <RouterView/>
      </b-card>
    </div>
  </main>
</template>

<style scoped>
.content {
  padding: 20px;
  display: flex;
  justify-content: center;
}

.content > div {
  max-width: 100%;
  width: 900px
}

#navbar {
  position: sticky;
  top: 0;
  left: 0;
  z-index: 1000;
}
</style>

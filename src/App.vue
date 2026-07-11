<script setup>
  import { invoke } from '@tauri-apps/api/core'

  async function CreateProfile() {
    await invoke('add_profile', {token: ""})
  }
</script>

<template>
  <div class="page-wrapper">
    <div class="main-container">
      <router-view />
    </div>
    <div class="side-box">
      <router-view name="profile_info"/>
    </div>
  </div>
</template>

<style>
  html, body {
    margin: 0;
    padding: 0;
    background-color: #3c3c3c;
  }

  a {
    color: #fff;
    text-decoration: none;
  }
</style>

<style scoped>
  .page-wrapper {
    display: flex;
    justify-content: center;
    min-height: 100vh;
    background-color: #3c3c3c;
  }
  .main-container {
    width: 100%;
    max-width: 800px;
  }

  .side-box {
    display: none;
    padding: 10px;
    background-color: #1d1d1d;
  }

  @media (min-width: 800px) {
    .main-container {
      flex-shrink: 0;
      max-width: 400px;
    }
    .side-box {
      display: block;
      flex: 1;
    }
  }
</style>
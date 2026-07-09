<template>
  <div class="page-wrapper">
    <div class="main-container">
      <div class="Hero">
        180000
      </div>
      <nav style="background-color: #1d1d1d;">
        <router-link to="/">Home</router-link>
        <router-link to="/settings">Settings</router-link>
      </nav>
      <router-view />
    </div>
    <div class="side-box"></div>
  </div>
</template>
<style>
  html, body {
    margin: 0;
    padding: 0;
    background-color: #3c3c3c;
  }
</style>
<style scoped>
  .page-wrapper {
    display: flex;
    justify-content: center;
    min-height: 100vh;
  }
  .main-container {
    width: 100%;
    max-width: 700px;
  }
  .Hero {
    width: 100%; 
    max-width: 700px;
    height: 150px; 
    font-family: 'Bungee', sans-serif;
    color: #fff;
    background-image: url('@/assets/hero.svg');
    background-size: cover;
    background-color: #CE7F1F; 
    display: flex; 
    align-items: center; 
    justify-content: center;
  }
  nav {
    background-color: #1d1d1d;
    padding: 10px;
    gap: 10px;
    display: flex;
  }
  nav :deep(a) {
    color: #fff;
    text-decoration: none;
  }
  .side-box {
    display: none;
    background-color: #1d1d1d;
  }
  @media (min-width: 700px) {
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
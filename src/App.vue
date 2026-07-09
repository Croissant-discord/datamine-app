<template>
  <div class="main-container" style="max-width: 400px;">
    <div class="Hero" style="">
      180000
    </div>

    <nav style="background-color: #1d1d1d;">
      <router-link to="/">Home</router-link>
      <router-link to="/settings">Settings</router-link>
    </nav>

    <router-view />    
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
.Hero {
  width: 100%; 
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
</style>

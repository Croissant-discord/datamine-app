<template>
  <div style="width: 100%; height: 150px; background-color: #CE7F1F;"></div>
  
  <nav style="background-color: #1d1d1d; padding: 10px; gap: 10px; display: flex;">
    <router-link to="/">Home</router-link>
    <router-link to="/settings">Settings</router-link>
  </nav>

  <router-view />
</template>

<style>
  html, body {
    margin: 0;
    padding: 0;
    background-color: #3c3c3c;
  }
</style>
<script setup>
import { ref, onMounted, onBeforeUnmount } from 'vue'
import { invoke } from '@tauri-apps/api/core'

const profiles = ref([])
const isMenuOpen = ref(false)
const isImporting = ref(false)
const importToken = ref('')

async function loadProfiles() {
  try {
    profiles.value = await invoke('profile_list')
  } catch (e) {
    console.error('loadProfiles failed:', e)
  }
}

async function CreateProfile(token = '') {
  isMenuOpen.value = false
  isImporting.value = false
  importToken.value = ''
  await invoke('profile_add', { token })
  await loadProfiles()
}

function toggleMenu() {
  isMenuOpen.value = !isMenuOpen.value
  if (!isMenuOpen.value) isImporting.value = false
}

function showImportInput() {
  isImporting.value = true
}

function submitImport() {
  if (!importToken.value.trim()) return
  CreateProfile(importToken.value.trim())
}

function handleOutsideClick(e) {
  if (!e.target.closest('.add-profile-wrapper')) {
    isMenuOpen.value = false
    isImporting.value = false
  }
}

onMounted(() => {
  const loadProfilesWithRetry = async (delay = 500, maxDelay = 5000) => {
    try {
      await loadProfiles()
    } catch (err) {
      console.warn('loadProfiles failed, retrying...', err)
      setTimeout(() => {
        loadProfilesWithRetry(Math.min(delay * 1.5, maxDelay), maxDelay)
      }, delay)
    }
  }

  setTimeout(() => {
    loadProfilesWithRetry()
  }, 500)

  document.addEventListener('click', handleOutsideClick)
})

onBeforeUnmount(() => {
  document.removeEventListener('click', handleOutsideClick)
})
</script>

<template>
  <div class="main-container">
    <div class="Hero">
      180000
    </div>
    <div style="background-color: #1d1d1d; color: #fff; display: flex; align-items: center; justify-content: space-between; padding: 12px 20px;">
      <span>Accounts</span>
      <div class="add-profile-wrapper">
        <span class="add_profile" @click.stop="toggleMenu">+</span>
        <div v-if="isMenuOpen" class="add-profile-menu" @click.stop>
          <div class="import-form">
            <input
              v-model="importToken"
              type="text"
              placeholder="Paste token"
              @keyup.enter="submitImport"
            />
            <button @click="submitImport">Add</button>
          </div>
        </div>
      </div>
    </div>
    <div class="profile-list">
      <div v-for="profile in profiles">
        <div class="profile-container">
          {{ profile }}
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
  .main-container {
    width: 100%;
    color: #fff;
    box-sizing: border-box;
    scrollbar-width: thin;
    scrollbar-color: #1d1d1d transparent;
  }
  .profile-list {
    overflow-y: scroll;
    padding: 10px;
    height: 280px;
  }
  .profile-container {
    width: 100%;
    height: 70px;
    margin-bottom: 10px;
    cursor: pointer;
    background-color: #1d1d1d;
    border-radius: 10px;
  }
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
  .add-profile-wrapper {
    position: relative;
    display: inline-block;
  }
  .add_profile {
    cursor: pointer;
  }
  .add-profile-menu {
    position: absolute;
    top: 28px;
    right: 0;
    min-width: 180px;
    background: #1d1d1d;
    border: 1px solid #333;
    border-radius: 8px;
    box-shadow: 0 4px 12px rgba(0,0,0,0.3);
    padding: 6px;
    z-index: 10;
  }
  .menu-item {
    padding: 8px 10px;
    border-radius: 6px;
    cursor: pointer;
    font-size: 14px;
    white-space: nowrap;
  }
  .menu-item:hover {
    background: #2a2a2a;
  }
  .import-form {
    display: flex;
    gap: 6px;
    padding: 6px;
  }
  .import-form input {
    flex: 1;
    min-width: 0;
    background: #111;
    border: 1px solid #333;
    border-radius: 6px;
    color: #fff;
    padding: 6px 8px;
    font-size: 13px;
  }
  .import-form button {
    background: #333;
    border: none;
    border-radius: 6px;
    color: #fff;
    padding: 6px 10px;
    font-size: 13px;
    cursor: pointer;
  }
  .import-form button:hover {
    background: #444;
  }

  @media (min-width: 800px) {
    .main-container {
      flex-shrink: 0;
      max-width: 400px;
    }
  }
</style>
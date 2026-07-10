<script setup>
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'

const profiles = ref([])

async function loadProfiles() {
  profiles.value = await invoke('profile')
}

async function CreateProfile() {
	profiles.value = await invoke('saved_profiles')
}

onMounted(() => {
  loadProfiles()
})
</script>

<template>
	<div class="main-container">
		{{profiles}}
	</div>

	<div @click="CreateProfile" style="width: 20px; height: 20px; background-color: #1d1d1d;"></div>
</template>

<style scoped>
	.main-container {
		width: 100%;
		height: auto;
		color: #fff;
		padding: 10px;
		box-sizing: border-box;
	}
</style>
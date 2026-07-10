<script setup>
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'

const profiles = ref([])

async function loadProfiles() {
  profiles.value = await invoke('profile')
}

async function CreateProfile() {
	await invoke('saved_profiles')
	await loadProfiles()
}

onMounted(() => {
  loadProfiles()
})
</script>

<template>
	<div class="main-container">
		{{profiles}}
	</div>

	<div @click="CreateProfile" style="cursor: pointer; width: 20px; height: 20px; background-color: #1d1d1d;"></div>
</template>
	
<style scoped>
	.main-container {
		width: 100%;
		height: 290px;
		color: #fff;
		overflow-y: scroll;
		padding: 10px;
		box-sizing: border-box;

		/* Firefox */
		scrollbar-width: thin;
		scrollbar-color: #1d1d1d transparent;
	}

</style>
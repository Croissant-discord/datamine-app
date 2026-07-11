<script setup>
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'

const profiles = ref([])

async function loadProfiles() {
  try {
    profiles.value = await invoke('profile')
  } catch (e) {
    console.error('loadProfiles failed:', e)
  }
}

async function CreateProfile() {
	await invoke('add_profile', {token: ""})
	await loadProfiles()
}

onMounted(() => {
  setTimeout(() => {
    loadProfiles()
  }, 500)
})
</script>

<template>
	<div class="main-container">
		<div v-for="profile in profiles">
			<div class="profile-container">
				{{profile}}
			</div>
		</div>
	</div>

	<div @click="CreateProfile" style="cursor: pointer; width: 20px; height: 20px; background-color: #1d1d1d;"></div>
</template>
	
<style scoped>
	.main-container {
		width: 100%;
		height: 280px;
		color: #fff;
		overflow-y: scroll;
		padding: 10px;
		box-sizing: border-box;

		/* Firefox */
		scrollbar-width: thin;
		scrollbar-color: #1d1d1d transparent;
	}

	.profile-container {
		width: 100%;
		height: 70px;
		margin-bottom: 10px;
		cursor: pointer;
		background-color: #1d1d1d;
		border-radius: 10px;
	}

</style>
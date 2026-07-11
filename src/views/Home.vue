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
     <div class="Hero">
        180000
      </div>
      <div style="background-color: #1d1d1d; color: #fff; display: flex; align-items: center; justify-content: space-between; padding: 12px 20px;">
        <span>Accounts</span>
        <span class="add_profile" @click="CreateProfile">+</span>
      </div>

     <div class="profile-list">
				<div v-for="profile in profiles">
					<div class="profile-container">
						{{profile}}
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
		

		/* Firefox */
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

</style>


<style scoped>
  .page-wrapper {
    display: flex;
    justify-content: center;
    min-height: 100vh;
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

  .add_profile {
    cursor: pointer;
  }

  .side-box {
    display: none;
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
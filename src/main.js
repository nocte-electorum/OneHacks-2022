import App from './App.vue'
import { appWindow } from '@tauri-apps/api/window'
import { createApp } from 'vue'
import { invoke } from '@tauri-apps/api'

createApp(App).mount('#app')

appWindow.listen("tauri://close-requested", async () => {
	await invoke("save_keys")
})
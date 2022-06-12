import App from './App.vue'
import { appWindow } from '@tauri-apps/api/window'
import { createApp } from 'vue'
import { invoke } from '@tauri-apps/api'


const app = createApp(App)
app.mount('#app')
// app.provide('currentScreen', 'CreateScreen')


appWindow.listen("tauri://close-requested", async () => {
	await invoke("save_keys")
	appWindow.close()
})
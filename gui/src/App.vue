<script setup lang="ts">
import { onMounted, ref, watch } from "vue";
import "./assets/globals.css";
import Map from "./components/Map.vue";
import Settings from "./components/Settings.vue";
import { invoke } from "@tauri-apps/api/core";
import { getGeoLocation } from "./lib/geo";
import { listen } from "@tauri-apps/api/event";
import { quickConnect, stopTunnel } from "./lib/tunnel";
import { Toaster } from 'vue-sonner';
import 'vue-sonner/style.css'
import Toolbar from "./components/Toolbar.vue";
import { startPinging, stopPinging } from "./lib/network";


interface TunnelPayload {
	name: string;
	is_active: boolean;
};

const isConnected = ref(false);
const isSettingsOpen = ref(false);
const activeTunnel = ref<string | null>(null);
const mapFocusIp = ref<string | null>(null);

const locationData = ref({
	country: "",
	as_name: ""
});

const networkData = ref<{ latency: null | number }>({
	latency: null
});

const availableEndpoints = ref<any[]>([]);

async function handleToggle() {

	if (isConnected.value) {
		await stopTunnel();
	} else {
		await quickConnect();
	};

};

async function getAvailableEndpoints() {

	const confs: String[] = await invoke("get_configs");

	const locationsPromises = confs.map(async (conf) => {

		const lastIndex = conf.lastIndexOf('.');
		const ip = lastIndex !== -1 ? conf.substring(0, lastIndex) : conf;

		const res = await getGeoLocation(ip.toString());
		return res;

	});

	const locations = await Promise.all(locationsPromises);
	const validLocations = locations.filter(Boolean);

	return validLocations;

};

onMounted(async () => {

	try {

		const status = await invoke<TunnelPayload>('get_current_tunnel_status');

		isConnected.value = status.is_active;
		activeTunnel.value = status.name;

	} catch (err) { }

	await listen("tunnel-status", (event: { payload: TunnelPayload }) => {
		isConnected.value = event.payload.is_active;
		activeTunnel.value = event.payload.name;
	});

	await listen("ping-result", (event: { payload: [string, number] }) => {
		networkData.value.latency = event.payload[1];
	});

	await listen("ping-stopped", () => {
		networkData.value.latency = null;
	});

	const endpoints = await getAvailableEndpoints();
	availableEndpoints.value = endpoints;

});

watch(() => isConnected.value, async (connected) => {

	const data = await getGeoLocation();

	if (data) {

		locationData.value = { ...data };

		if (!connected) {
			mapFocusIp.value = data.ip;
		} else {
			mapFocusIp.value = activeTunnel.value;
		}

	};

	if (connected) {
		startPinging();
	} else {
		stopPinging();
	};

}, { immediate: true });

const openSettings = () => isSettingsOpen.value = true;
const closeSettings = () => isSettingsOpen.value = false;

</script>

<template>

	<Toaster class="background-blur-xl" :toastOptions="{
		class: 'backdrop-blur-xl !bg-[#19272a]/60 border-t border-white/20 border-x border-b border-white/5'
	}" :closeButton="true" closeButtonPosition="top-right" position="top-left" theme="dark" richColors />

	<main class="h-screen w-screen bg-[#19272a] bg-cover bg-center">

		<Map :tunnel="mapFocusIp" :isConnected="isConnected" />

		<!-- gradient -->
		<div class="absolute h-full w-full bg-linear-to-b via-transparent to-black/10 z-20 pointer-events-none transition-colors duration-1000"
			:class="isConnected ? 'from-emerald-500/30' : 'from-[#ff006e]/30'" />

		<Toolbar :isOpen="isSettingsOpen" v-on:open="openSettings" v-on:close="closeSettings" />

		<div class="absolute z-50 bottom-0 left-0 w-full p-4 flex flex-col items-center">

			<!-- <button @click="handleToggle" class="py-4 rounded-full px-14 mb-8 font-medium select-none shadow-2xl flex items-center space-x-4"
				:class="isConnected ? 'bg-neutral-700' : 'bg-linear-to-br from-pink-500 via-purple-500 to-blue-600'">
				<span>{{ isConnected ? "Disconnect" : "Connect" }}</span>
			</button> -->

			<!-- <button @click="handleToggle" style="box-shadow: -5px 5px 0px 0px oklch(58.5% 0.233 277.117)"
				class="mb-8 text-base px-8 py-3 bg-black border-[3px] border-indigo-700 rounded-2xl text-white font-black transition-transform duration-[400ms] ease-[cubic-bezier(0.68,-0.55,0.265,1.55)]">
				{{ isConnected ? "Disconnect" : "Connect" }}
			</button> -->

			<div class="mb-8">
				<button @click="handleToggle" :class="['uiverse', { connected: isConnected }]">
					<div class="wrapper">
						<span>{{ isConnected ? "Disconnect" : "Connect" }}</span>
						<div class="circle circle-12"></div>
						<div class="circle circle-11"></div>
						<div class="circle circle-10"></div>
						<div class="circle circle-9"></div>
						<div class="circle circle-8"></div>
						<div class="circle circle-7"></div>
						<div class="circle circle-6"></div>
						<div class="circle circle-5"></div>
						<div class="circle circle-4"></div>
						<div class="circle circle-3"></div>
						<div class="circle circle-2"></div>
						<div class="circle circle-1"></div>
					</div>
				</button>

			</div>

			<!-- <div v-if="isConnected" class="h-auto lg:w-1/2 w-full py-2">
				<div>
					<p class="text-[12px] text-neutral-400">Latency</p>
					<p class="text-sm">{{ networkData.latency ? `${networkData.latency}ms` : 'Detecting...' }}</p>
				</div>
			</div> -->

			<div class="h-auto lg:w-1/2 w-full z-50 md:px-0 px-4 py-2 flex justify-between select-none">
				<!-- <div class="h-auto bg-neutral-900 w-full z-50 rounded-md border border-neutral-500/20 px-4 py-2 flex justify-between"> -->

				<div>
					<p class="text-[12px] text-neutral-400">Your IP Address</p>
					<p class="text-sm">{{ mapFocusIp || 'Detecting...' }}</p>
				</div>

				<div class="h-full w-px border border-neutral-500/20 mx-8" />

				<div>
					<p class="text-[12px] text-neutral-400">Country</p>
					<p class="text-sm">{{ locationData.country || 'Detecting...' }}</p>
				</div>

				<div class="h-full w-px border border-neutral-500/20 mx-8" />

				<div>
					<p class="text-[12px] text-neutral-400">Provider</p>
					<p class="text-sm">{{ locationData.as_name || 'Detecting...' }}</p>
				</div>

			</div>

		</div>

		<Settings :isOpen="isSettingsOpen" @close="isSettingsOpen = false" />

	</main>

</template>
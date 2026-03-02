<script setup lang="ts">
import { onMounted, ref, watch } from "vue";
import "./assets/globals.css";
import Map from "./components/Map.vue";
import Settings from "./components/Settings.vue";
import { invoke } from "@tauri-apps/api/core";
import { GeoLocation, getGeoLocation } from "./lib/geo";
import { listen } from "@tauri-apps/api/event";
import { getTunnelStatus, quickConnect, stopTunnel, TunnelMetadata } from "./lib/tunnel";
import { toast, Toaster } from 'vue-sonner';
import 'vue-sonner/style.css'
import Toolbar from "./components/Toolbar.vue";
import { startPinging, stopPinging } from "./lib/network";
import { Loader, Loader2, Search, ServerCog, X } from "lucide-vue-next";


interface TunnelPayload {
	name: string;
	is_active: boolean;
};

const isConnected = ref(false);
const isPending = ref(false);

const isSettingsOpen = ref(false);
const activeTunnel = ref<string | null>(null);
const mapFocusIp = ref<string | null>(null);

const serverSelection = ref(false);

const locationData = ref({
	country: "",
	as_name: ""
});

const networkData = ref<{ latency: null | number }>({
	latency: null
});

const availableEndpoints = ref<GeoLocation[]>([]);

async function toggleConnection() {

	isPending.value = true;

	if (isConnected.value) {
		await stopTunnel();
	} else {
		await quickConnect();
	};

	try {
		const { data } = await getTunnelStatus();
		if (data) {
			isConnected.value = data.is_active;
			activeTunnel.value = data.name;
			console.log(data)
		};
	} catch (err) {
		toast("Failed to sync connection status");
	};

	isPending.value = false;

};

async function getAvailableEndpoints() {

	const confs: TunnelMetadata[] = await invoke("get_configs");

	const locationsPromises = confs.map(async (conf) => {

		const ip = conf.name.split("-")[1];

		console.log(ip)

		const res = await getGeoLocation(ip);
		console.log(res)
		return res;

	});

	const locations = await Promise.all(locationsPromises);
	const validLocations = locations.filter(i => i !== null);

	console.log(validLocations)

	return validLocations;

};

onMounted(async () => {

	try {

		const status = await invoke<TunnelPayload>('get_current_tunnel_status');

		console.log(status);

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

	if (!connected) {
		await new Promise(r => setTimeout(r, 500));
	}

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

const toggleServerSelection = () => serverSelection.value = !serverSelection.value;

</script>

<template>

	<Toaster class="background-blur-xl" :toastOptions="{
		class: 'backdrop-blur-xl !bg-[#19272a]/60 border-t border-white/20 border-x border-b border-white/5'
	}" :closeButton="true" closeButtonPosition="top-right" position="top-left" theme="dark" richColors />

	<main class="h-screen w-scree bg-cover bg-center isolate">

		<Map :tunnel="mapFocusIp" :isConnected="isConnected" />


		<!-- gradient -->
		<div class="absolute h-full w-full bg-linear-to-b via-transparent to-black/10 z-20 pointer-events-none transition-colors duration-1000"
			:class="isConnected ? 'from-accent-500/30' : 'from-brand-500/30'" />

		<Toolbar :isOpen="isSettingsOpen" v-on:open="openSettings" v-on:close="closeSettings" />

		<div class="absolute z-50 bottom-0 left-0 w-full p-4 flex flex-col items-center">

			<div class="flex justify-center items-center mb-8 space-x-2">

				<button @click="toggleConnection" :disabled="isPending"
					class="h-12 w-52 capitalize font-semibold text-lg select-none flex items-center justify-center disabled:bg-neutral-500 disabled:text-neutral-800"
					:class="isConnected ? 'bg-neutral-500 text-neutral-100' : 'bg-accent-500 text-black '">
					<span v-if="isPending" class="flex items-center gap-x-4">
						<Loader2 class="animate-spin" />
					</span>
					<span v-else>
						{{ isConnected ? "disconnect" : "connect" }}
					</span>
				</button>

				<button @click="toggleServerSelection"
					class="size-12 capitalize font-semibold text-lg select-none flex items-center justify-center disabled:bg-neutral-500 disabled:text-neutral-800 bg-neutral-700 text-neutral-200">
					<ServerCog />
				</button>

			</div>

			<div class="h-auto lg:w-2/3 w-full z-50 md:px-0 px-4 py-2 flex justify-between select-none">

				<div v-if="isConnected">
					<p class="text-sm text-neutral-400">Latency</p>
					<p class="text-sm">{{ networkData.latency ? `${networkData.latency}ms` : 'Detecting...' }}</p>
				</div>

				<div>
					<p class="text-sm text-neutral-400">Your IP Address</p>
					<p class="text-sm">{{ mapFocusIp || 'Detecting...' }}</p>
				</div>

				<div class="h-full w-px border border-neutral-500/20 mx-8" />

				<div>
					<p class="text-sm text-neutral-400">Country</p>
					<p class="text-sm">{{ locationData.country || 'Detecting...' }}</p>
				</div>

				<div class="h-full w-px border border-neutral-500/20 mx-8" />

				<div>
					<p class="text-sm text-neutral-400">Provider</p>
					<p class="text-sm">{{ locationData.as_name || 'Detecting...' }}</p>
				</div>

			</div>

		</div>


		<Settings :isOpen="isSettingsOpen" @close="isSettingsOpen = false" />

		<Transition name="slide">
			<div v-if="serverSelection" class="absolute z-80 w-96 h-full p-4">

				<div
					class="w-full h-full bg-neutral-800/50 rounded-lg backdrop-blur-sm p-4 border border-neutral-400/10 select-none">

					<div class="w-full flex items-start justify-between gap-x-8">

						<div>
							<p class="font-medium text-lg select-none text-neutral-100">Server list</p>
							<p class="font-medium text-sm select-none text-neutral-400">Select a server to continue with
								a
								secure connection</p>
						</div>

						<button @click="toggleServerSelection" class="rounded-full bg-neutral-700 p-1.5">
							<X class="text-neutral-100" :size="18" />
						</button>

					</div>

					<div class="h-10 w-full rounded-md bg-neutral-700 mt-6 flex items-center px-4 space-x-4">
						<Search class="text-neutral-400" :size="20" />
						<input placeholder="Search..." class="outline-none" />
					</div>

					<div class="mt-10 overflow-y-auto max-h-[calc(100vh-250px)] pr-2">
						<TransitionGroup name="stagger" appear>
							<div v-for="(i, index) in availableEndpoints" :key="i.ip"
								:style="{ transitionDelay: `${index * 50}ms` }"
								class="flex items-center gap-x-4 bg-neutral-700/40 hover:bg-neutral-700/80 p-3 rounded-md mb-2 cursor-pointer transition-colors">
								<div class="w-10 rounded-sm overflow-hidden border border-white/5">
									<img :src="`https://flagcdn.com/h80/${i.country_code.toLowerCase()}.webp`"
										class="w-full h-full object-cover" />
								</div>
								<p class="text-neutral-100 text-sm font-medium">{{ i.country }}</p>
							</div>
						</TransitionGroup>
					</div>

				</div>

			</div>
		</Transition>


	</main>

</template>
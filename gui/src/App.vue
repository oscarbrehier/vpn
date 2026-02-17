<script setup lang="ts">
import { onMounted, ref } from "vue";
import "./assets/globals.css";
import Map from "./components/Map.vue";
import { Server, Settings, User, X } from "lucide-vue-next";

const isConnected = ref(false);
const isConnecting = ref(false);
const isSettingsOpen = ref(false);

async function handleToggle() {
	if (isConnected.value) {
		isConnected.value = false
		return
	}

	isConnecting.value = true
	await new Promise(resolve => setTimeout(resolve, 2000))
	isConnecting.value = false
	isConnected.value = true
};

const locationData = ref({
	ip: "",
	city: "",
	country: "",
	lat: 0,
	lon: 0,
	isp: ""
});

async function getGeoLocation() {

	try {

		const res = await fetch(`http://ip-api.com/json/`);
		const data = await res.json();

		if (data.status == "success") {
			locationData.value = {
				ip: data.query,
				...data
			};
		}

	} catch (err) {

	}

}

onMounted(async () => {
	await getGeoLocation();
});

const openSettings = () => isSettingsOpen.value = true;
const closeSettings = () => isSettingsOpen.value = false;

</script>

<template>

	<main class="h-screen w-screen bg-[#19272a] bg-cover bg-center">

		<!-- gradient -->
		<div class="absolute h-full w-full bg-linear-to-b via-transparent to-black/10 z-20 pointer-events-none transition-colors duration-1000"
			:class="isConnected ? 'from-emerald-500/30' : 'from-red-500/30'" />

		<!-- toolbar -->
		<div class="h-20 w-auto absolute top-0 right-0 z-60 p-4">

			<button
				v-if="!isSettingsOpen"
				@click="openSettings"
			>
				<Settings />
			</button>

			<button
				v-else="isSettingsOpen"
				@click="closeSettings"
			>
				<X />
			</button>

		</div>

		<!-- connection info -->
		<div
			class="absolute h-20 w-full bottom-0 right-0 z-30 pointer-events-none flex items-center justify-between px-4">

			<div>
				<p class="text-sm text-neutral-400">Your IP Address</p>
				<p>{{ locationData.ip || 'Detecting...' }}</p>
			</div>

			<div>
				<p class="text-sm text-neutral-400">Country</p>
				<p>{{ locationData.country || 'Detecting...' }}</p>
			</div>

			<div>
				<p class="text-sm text-neutral-400">Provider</p>
				<p>{{ locationData.isp || 'Detecting...' }}</p>
			</div>

		</div>

		<Map :lat="locationData.lat" :lon="locationData.lon" :country="locationData.country" />

		<!-- <div class="absolute p-4 bottom-0">

			<div
				class="h-full w-[30vw] bg-neutral-800/30 backdrop-blur-xl p-4 rounded-md border border-neutral-100/10 flex flex-col justify-between">

				<div></div>

				<div>

					<div class="flex flex-col space-y-4">

						<div className="flex flex-col gap-1.5">
							<label htmlFor="host"
								className="flex items-center gap-1.5 text-[12px] font-medium text-secondary-foreground">
								IP Address
							</label>
							<input id="host" placeholder="e.g. "
								class="bg-neutral-800 w-full rounded px-4 py-2 outline-none text-sm font-mono" />
						</div>

						<div className="flex flex-col gap-1.5">
							<label htmlFor="ssh-user"
								className="flex items-center gap-1.5 text-[12px] font-medium text-secondary-foreground">
								SSH Username
							</label>
							<input id="ssh-user" placeholder="e.g. root"
								class="bg-neutral-800 w-full rounded px-4 py-2 outline-none text-sm font-mono" />
						</div>

					</div>

					<button @click="handleToggle" class="h-10 w-full rounded bg-[#0652DD] mt-8 text-sm">
						Connect
					</button>

				</div>

			</div>

		</div> -->

		<div
			v-if="isSettingsOpen"
			class="absolute w-full h-full bg-neutral-700/40 z-50 backdrop-blur-xl p-8 flex items-center justify-center">

			<!-- <h1 class="text-xl">Settings</h1> -->

			<div class="w-[40vw]">

				<div class="space-y-4">

					<div className="flex flex-col gap-1.5">
						<label htmlFor="host"
							className="flex items-center gap-1.5 text-[12px] font-medium text-secondary-foreground">
							IP Address
						</label>
						<input id="host" placeholder="e.g. "
							class="bg-neutral-800/70 w-full rounded-lg px-4 py-2 outline-none text-sm font-mono" />
					</div>

					<div className="flex flex-col gap-1.5">
						<label htmlFor="ssh-user"
							className="flex items-center gap-1.5 text-[12px] font-medium text-secondary-foreground">
							SSH Username
						</label>
						<input id="ssh-user" placeholder="e.g. root"
							class="bg-neutral-800/70 w-full rounded-lg px-4 py-2 outline-none text-sm font-mono" />
					</div>

					<div className="flex flex-col gap-1.5">
						<label htmlFor="ssh-key"
							className="flex items-center gap-1.5 text-[12px] font-medium text-secondary-foreground">
							SSH Private Key
						</label>
						<input id="ssh-key" placeholder="e.g. root"
							class="bg-neutral-800/70 w-full rounded-lg px-4 py-2 outline-none text-sm font-mono" />
					</div>

				</div>

				<button class="w-full bg-blue-700 rounded-lg h-10 text-sm mt-6">
					Save configuration
				</button>

			</div>

		</div>

	</main>

</template>
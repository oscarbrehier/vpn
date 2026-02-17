<script setup lang="ts">
import { onMounted, reactive, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import "./assets/globals.css";
import PowerButton from "./components/PowerButton.vue";
import Map from "./components/Map.vue";

const isConnected = ref(false);
const isConnecting = ref(false);

async function handleToggle() {
	if (isConnected.value) {
		isConnected.value = false
		return
	}

	isConnecting.value = true
	await new Promise(resolve => setTimeout(resolve, 2000))
	isConnecting.value = false
	isConnected.value = true
}

</script>

<template>

	<main class="h-screen w-screen bg-[#19272a] bg-cover bg-center">

		<div class="absolute h-full w-full bg-linear-to-b from-red-500/30 via-transparent to-black/10 z-10 pointer-events-none">
		</div>

		<Map />

		<!-- <div
			class="h-full w-[35vw] bg-neutral-800/30 backdrop-blur-xl p-4 rounded-md border border-neutral-100/10 flex flex-col justify-between">

			<div></div>

			<div>

				<div class="flex flex-col gap-2">

					<input placeholder="Host/IP" class="bg-neutral-800 w-full rounded px-4 py-2 outline-none" />

					<input placeholder="Username" class="bg-neutral-800 w-full rounded px-4 py-2 outline-none" />

					<input placeholder="SSH Key" class="bg-neutral-800 w-full rounded px-4 py-2 outline-none" />

				</div>

				<button class="h-10 w-full rounded bg-[#0652DD] mt-4">
					Configure
				</button>

			</div>

		</div> -->

	</main>

</template>
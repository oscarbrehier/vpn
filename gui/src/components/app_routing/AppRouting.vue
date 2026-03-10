<script setup lang="ts">
import { computed, onMounted, ref } from 'vue';
import { AppGroup, getRunningApps, getTunneledApps, updateTunneledApps } from '../../lib/apps';
import AppRow from './AppRow.vue';
import { TunnelMode } from '../../lib/tunnel';
import { Zap } from 'lucide-vue-next';

defineProps<{
	mode: TunnelMode;
}>();

const allRunningApps = ref<AppGroup[]>([]);
const activeTunneledPids = ref<number[]>([]);

const isAppProtected = (app: AppGroup) => {
	return app.pids.some(pid => activeTunneledPids.value.includes(pid));
};

const protectedApps = computed(() => {
	return allRunningApps.value
		.filter(app => isAppProtected(app))
		.sort((a, b) => a.name.localeCompare(b.name, undefined, { sensitivity: 'base' }));
});

const excludedApps = computed(() => {
	return allRunningApps.value
		.filter(app => !isAppProtected(app))
		.sort((a, b) => a.name.localeCompare(b.name, undefined, { sensitivity: 'base' }));
});

async function toggleAppProtection(app: AppGroup) {

	const isCurrentlyProtected = isAppProtected(app);
	let newPidsList = [...activeTunneledPids.value];

	if (!isCurrentlyProtected) {
		newPidsList.push(...app.pids);
	} else {
		newPidsList = newPidsList.filter(pid => !app.pids.includes(pid));
	};

	await updateTunneledApps(newPidsList);
	activeTunneledPids.value = newPidsList;

};

onMounted(async () => {

	const [appsData, tunnelData] = await Promise.all([
		getRunningApps(),
		getTunneledApps()
	]);

	if (appsData) allRunningApps.value = appsData;
	if (tunnelData) activeTunneledPids.value = tunnelData;

});


</script>

<template>
	<div v-if="mode == 'split'">
		<div>
			<div
				class="w-full flex items-center justify-between text-sm text-neutral-300 mb-2 sticky top-0 bg-neutral-800 py-1">
				<p>Protected Apps</p>
				<p>({{ protectedApps.length }})</p>
			</div>

			<AppRow v-for="app in protectedApps" :app="app" @click="toggleAppProtection(app)" />
		</div>

		<div>
			<div
				class="w-full flex items-center justify-between text-sm text-neutral-300 mb-2 sticky top-0 bg-neutral-800 py-1">
				<p>Available Apps</p>
				<p>({{ excludedApps.length }})</p>
			</div>

			<AppRow v-for="app in excludedApps" :app="app" @click="toggleAppProtection(app)" />
		</div>
	</div>

	<div v-else class="flex flex-col items-center justify-center pt-10">

		<div
			class="size-16 bg-brand-500/10 rounded-full flex items-center justify-center mb-6 ring-1 ring-brand-500/20">
			<Zap class="text-brand-500" />
		</div>

		<h3 class="text-white font-semibold mb-2">Global Protection On</h3>
		<p class="text-xs text-neutral-400 leading-relaxed mb-6">
			All your apps are currently secured. To manage routing for specific applications, switch your connection
			mode to <span class="text-brand-400 font-semibold">Split Tunnel</span>.
		</p>

		<div class="text-[10px] text-neutral-500 italic bg-white/5 p-3 rounded-sm border border-white/5">
			Tip: Split tunneling is useful if you want to keep your games or local streaming apps off the VPN.
		</div>
	</div>

</template>

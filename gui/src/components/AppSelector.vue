<script setup lang="ts">
import { computed, onMounted, ref } from 'vue';
import Drawer from './Drawer.vue';
import { AppGroup, getRunningApps } from '../lib/apps';
import { AppWindowMac } from 'lucide-vue-next';

const apps = ref<AppGroup[]>([]);

const sortedApps = computed(() => {
	return [...apps.value].sort((a, b) => a.name.localeCompare(b.name, undefined, { sensitivity: 'base' }));
});

onMounted(async () => {

	const data = await getRunningApps();
	if (data) {
		apps.value = data;
	}

});

</script>

<template>

	<Drawer>

		<div v-for="app in sortedApps" class="flex items-center gap-2 mb-2">
			<div class="w-8 h-8 shrink-0 flex items-center justify-center">
				<img v-if="app.icon_base64" :src="app.icon_base64" class="w-full h-full object-contain">
				<AppWindowMac v-else />
			</div>
			{{ app.name }}
		</div>

	</Drawer>

</template>
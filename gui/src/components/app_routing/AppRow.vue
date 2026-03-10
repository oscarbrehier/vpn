<script setup lang="ts">
import { AppWindowMac } from 'lucide-vue-next';
import { AppGroup } from '../../lib/apps';

defineProps<{
	app: AppGroup;
}>();

defineEmits<{
	(e: 'click'): void;
}>();

</script>
<template>

	<div class="flex items-center gap-2 mb-2" @click="$emit('click')">
		
		<div class="w-8 h-8 shrink-0 flex items-center justify-center">
			<img v-if="app.icon_base64" :src="app.icon_base64" class="w-full h-full object-contain">
			<AppWindowMac v-else />
		</div>
		{{ app.name }}

	</div>

</template>
<script setup lang="ts">
import { Split, Zap } from 'lucide-vue-next';
import { TunnelMode } from '../lib/tunnel';

defineProps<{ mode: TunnelMode }>();

defineEmits<{
	(e: 'set-mode', mode: TunnelMode): void;
}>();

</script>
<template>

	<div class="relative h-12 w-auto bg-neutral-800 grid grid-cols-2">

		<div class="absolute top-0 bottom-0 w-[calc(50%)] transition-all duration-300 ease-out" :class="[
			mode === 'full' ? 'translate-x-0 bg-brand-500' : 'translate-x-full bg-accent-500'
		]" />

		<button @click="$emit('set-mode', 'full')"
			class="group relative z-10 flex items-center justify-center transition-colors duration-300 px-8"
			:class="mode === 'full' ? 'text-white' : 'text-neutral-400 hover:text-neutral-200'">
			<Zap :size="20" :stroke-width="mode === 'full' ? 2.5 : 2" />

			<span
				class="absolute -top-11 scale-0 transition-all bg-neutral-800 px-2 py-1 text-[10px] font-bold text-white group-hover:scale-100">
				{{ mode === 'full' ? 'Currently Active' : 'Switch to Full Tunnel' }}
			</span>

		</button>

		<button @click="$emit('set-mode', 'split')"
			class="group relative z-10 flex items-center justify-center transition-colors duration-300 px-8"
			:class="mode === 'split' ? 'text-neutral-900' : 'text-neutral-400 hover:text-neutral-200'">
			<Split :size="20" :stroke-width="mode === 'split' ? 2.5 : 2" />

			<span
				class="absolute -top-11 scale-0 transition-all bg-neutral-800 px-2 py-1 text-[10px] font-bold text-white group-hover:scale-100">
				{{ mode === 'split' ? 'Currently Active' : 'Switch to Split Tunnel' }}
			</span>

		</button>

	</div>

</template>
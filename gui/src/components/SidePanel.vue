<script setup lang="ts">
import { X } from 'lucide-vue-next';
import { ref } from 'vue';

defineProps<{
	isOpen: boolean;
	title?: string;
	description?: string;
}>();

const emit = defineEmits(['close']);

const close = () => emit('close');

</script>

<template>

	<Transition name="slide">

		<div v-if="isOpen" class="absolute top-0 left-0 z-60 h-screen flex items-end p-2">

			<div class="bg-neutral-800 h-full p-4 w-[30vw] flex flex-col justify-between rounded-md">

				<div class="h-12 w-full flex items-center justify-between overflow-hidden">

					<div>

						<p>{{ title }}</p>
						<p class="text-sm text-neutral-400">{{ description }}</p>

					</div>

					<button @click="close" class="bg-neutral-600 rounded-full size-8 flex items-center justify-center">
						<X :size="16" />
					</button>

				</div>

				<div class="flex-1 min-h-0 w-full overflow-y-scroll hide-scrollbar mt-10">
					<slot></slot>
				</div>

			</div>

		</div>

	</Transition>

</template>
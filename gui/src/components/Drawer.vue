<script setup lang="ts">
import { X } from 'lucide-vue-next';

</script>

<template>
	<div class="absolute top-0 left-0 z-60 h-screen flex items-end p-2">

		<div class="bg-neutral-800 h-full p-4 w-[30vw] flex flex-col justify-between">

			<div class="h-12 w-full flex justify-between overflow-hidden">

				<div></div>

				<button class="bg-neutral-600 rounded-full size-8 flex items-center justify-center">
					<X :size="16" />
				</button>

			</div>

			<div class="flex-1 min-h-0 w-full overflow-y-scroll hide-scrollbar">
				<slot></slot>
			</div>

		</div>


	</div>
</template>
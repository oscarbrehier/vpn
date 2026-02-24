<script setup lang="ts">

import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import { Upload } from "lucide-vue-next";
import { ref } from "vue";

defineProps<{ isOpen: boolean }>();
const emit = defineEmits(['close']);

const hostIp = ref("");
const sshUser = ref("");
const sshPath = ref("");
const isSaving = ref(false);

async function selectKeyFile() {

	const selected = await open({
		multiple: false,
		directory: false,
		filters: [{
			name: 'SSH Key',
			extensions: ['*']
		}]
	});

	if (selected && typeof selected === 'string') {
		sshPath.value = selected;
	};

};

async function handleSave() {

	if (!hostIp.value || !sshUser.value || !sshPath.value) {
		alert("Please fill in all fields");
		return;
	}

	isSaving.value = true;

	try {

		await invoke("setup_server", {
			serverIp: hostIp.value,
			user: sshUser.value,
			keyFile: sshPath.value
		});

		emit('close');

	} catch (error) {

		alert(error);

	} finally {

		isSaving.value = false;

	};

}

</script>

<template>

	<Teleport to="body">

		<Transition name="fade">

			<div v-if="isOpen"
				class="fixed inset-0 w-full h-full bg-neutral-700/40 z-50 backdrop-blur-xl p-8 flex items-center justify-center">

				<div class="w-[40vw]">

					<div class="space-y-6">

						<div className="flex flex-col gap-1.5">
							<label class="text-xs font-semibold text-neutral-400 uppercase tracking-wider">
								IP Address
							</label>
							<input v-model="hostIp" id="host" placeholder="e.g. "
								class="bg-neutral-800/50 border border-white/5 w-full rounded-xl px-4 py-3 text-sm font-mono flex items-center justify-between hover:bg-neutral-800 transition-colors">
						</div>

						<div className="flex flex-col gap-1.5">
							<label class="text-xs font-semibold text-neutral-400 uppercase tracking-wider">
								SSH Username
							</label>
							<input v-model="sshUser" id="ssh-user" placeholder="e.g. root"
								class="bg-neutral-800/50 border border-white/5 w-full rounded-xl px-4 py-3 text-sm font-mono flex items-center justify-between hover:bg-neutral-800 transition-colors">
						</div>

						<div class="flex flex-col gap-2">
							<label class="text-xs font-semibold text-neutral-400 uppercase tracking-wider">
								SSH Private Key
							</label>

							<button @click="selectKeyFile"
								class="bg-neutral-800/50 border border-white/5 w-full rounded-xl px-4 py-3 text-sm font-mono flex items-center justify-between hover:bg-neutral-800 transition-colors">
								<div class="flex items-center gap-4 overflow-hidden">
									<Upload class="size-3 shrink-0" />
									<span class="truncate text-neutral-300 text-sm pr-4">
										{{ sshPath ? sshPath : 'Select Private Key' }}
									</span>
								</div>
								<span v-if="sshPath"
									class="text-[10px] text-emerald-500 font-bold uppercase">Selected</span>
							</button>
						</div>

					</div>

					<button @click="handleSave" :disabled="isSaving"
						class="w-full bg-blue-700 rounded-lg h-12 text-sm mt-8">
						{{ isSaving ? 'Saving...' : 'Save configuration' }}
					</button>

				</div>

			</div>

		</Transition>

	</Teleport>

</template>

<style scoped>
.fade-enter-active,
.fade-leave-active {
	transition: opacity 0.3s ease;
}

.fade-enter-from,
.fade-leave-to {
	opacity: 0;
}
</style>
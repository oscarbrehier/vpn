<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue';

const svgContent = ref('');
const isConnected = ref(false);
const isDragging = ref(false);

const SVG_W = 2000;
const SVG_H = 857;

const transform = reactive({
	x: 0,
	y: 0,
	scale: 1
});

const lastMousePos = { x: 0, y: 0 };

onMounted(async () => {
	try {
		const response = await fetch('/world.svg');
		const text = await response.text();
		svgContent.value = text.replace(/<svg[^>]*>|<\/svg>/g, '');
	} catch (err) {
		console.error("Failed to load map:", err);
	};
});

const startPan = (e: MouseEvent) => {
	isDragging.value = true;
	lastMousePos.x = e.clientX;
	lastMousePos.y = e.clientY;
};

const stopPan = () => {
	isDragging.value = false;
};

const doPan = (e: MouseEvent) => {

	if (!isDragging.value) return;

	const rect = (e.currentTarget as HTMLElement).getBoundingClientRect();

	const ratio = Math.max(SVG_W / rect.width, SVG_H / rect.height);
	const dx = (e.clientX - lastMousePos.x) * ratio;
	const dy = (e.clientY - lastMousePos.y) * ratio;

	let newX = transform.x + dx;
	let newY = transform.y + dy;

	const mapW = SVG_W * transform.scale;
	const mapH = SVG_H * transform.scale;

	const minX = SVG_W - mapW;
	const minY = SVG_H - mapH;

	transform.x = Math.min(0, Math.max(newX, minX));
	transform.y = Math.min(0, Math.max(newY, minY));

	lastMousePos.x = e.clientX;
	lastMousePos.y = e.clientY;

};

const handleWheel = (e: WheelEvent) => {

	const rect = (e.currentTarget as HTMLElement).getBoundingClientRect();
	const ratio = Math.max(SVG_W / rect.width, SVG_H / rect.height);

	const zoomSpeed = 0.1;
	const delta = e.deltaY > 0 ? -zoomSpeed : zoomSpeed;

	const oldScale = transform.scale;

	const newScale = Math.max(1, Math.min(8, oldScale + delta));

	const mouseX = (e.clientX - rect.left) * ratio;
	const mouseY = (e.clientY - rect.top) * ratio;

	const scaleRatio = newScale / oldScale;
	let nextX = mouseX - (mouseX - transform.x) * scaleRatio;
	let nextY = mouseY - (mouseY - transform.y) * scaleRatio;

	const mapW = SVG_W * newScale;
	const mapH = SVG_H * newScale;
	const minX = SVG_W - mapW;
	const minY = SVG_H - mapH;

	transform.x = Math.min(0, Math.max(nextX, minX));
	transform.y = Math.min(0, Math.max(nextY, minY));
	transform.scale = newScale;

};

const resetView = () => {
	transform.x = 0;
	transform.y = 0;
	transform.scale = 1;
};
</script>

<template>
	<div class="absolute inset-0 w-full h-full cursor-grab active:cursor-grabbing bg-[#1a1b26]" @mousedown="startPan"
		@mousemove="doPan" @mouseup="stopPan" @mouseleave="stopPan" @wheel.prevent="handleWheel">

		<svg viewBox="0 0 2000 857" class="w-full h-full">

			<g :style="{
				transform: `translate(${transform.x}px, ${transform.y}px) scale(${transform.scale})`,
				transformOrigin: '0 0',
				transition: isDragging ? 'none' : 'transform 0.1s ease-out'
			}">

				<g v-html="svgContent" class="fill-[#2b2c36] stroke-[#676a82]/20"></g>

				<circle cx="1000" cy="428" r="8" fill="#10b981" v-if="isConnected" />
			</g>
		</svg>
	</div>
</template>

<style scoped>
:deep(path) {
	stroke-width: 0.3px;
	vector-effect: non-scaling-stroke;
	stroke: #676a82;
}
</style>
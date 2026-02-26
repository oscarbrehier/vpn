import { runCommand } from "./tauri";

export async function startPinging() {
	console.log("start pinging")
	const { error } = await runCommand("start_ping_loop");
	console.error(error)
};

export async function stopPinging() {
	await runCommand("stop_ping_loop");
};
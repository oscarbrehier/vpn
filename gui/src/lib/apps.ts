import { runCommand } from "./tauri";

type Pid = number;

export interface AppGroup {
	pids: Pid[],
	name: string,
	path: string | null,
	icon_base64: string | null
};

export async function updateTunneledApps(pids: Pid[]) {

	await runCommand("update_tunneled_apps", true, {
		pids
	});

};

export async function getRunningApps() {

	const { data } = await runCommand<AppGroup[]>("fetch_apps", true);
	return data;

};

export async function getTunneledApps() {
	const { data } = await runCommand<Pid[]>("get_tunneled_apps", true);
	return data;
}
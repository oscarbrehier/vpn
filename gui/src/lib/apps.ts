import { runCommand } from "./tauri";

export interface AppGroup {
	pids: number[],
	name: string,
	path: string | null,
	icon_base64: string | null
}

export async function getRunningApps() {

	const { data } = await runCommand<AppGroup[]>("fetch_apps", true);
	return data;

};
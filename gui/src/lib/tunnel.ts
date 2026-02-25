import { message } from "@tauri-apps/plugin-dialog";
import { GeoLocation, getGeoLocation } from "./geo";
import { runCommand } from "./tauri";
import { toast } from "vue-sonner";

export interface VpnConfig {
	name: string;
	file_path: string;
	location?: GeoLocation;
}

export async function getConfigurations(): Promise<VpnConfig[]> {

	const { data: confs, error } = await runCommand<string[]>("get_configs", true);

	if (error || !confs) return [];

	const locationsPromises = confs.map(async (conf) => {

		const lastIndex = conf.lastIndexOf('.');
		const ip = lastIndex !== -1 ? conf.substring(0, lastIndex) : conf;

		const res = await getGeoLocation(ip);

		if (!res) return null;

		return {
			name: ip,
			file_path: conf,
			location: res
		};

	});

	const results = await Promise.all(locationsPromises);
	return results.filter((item) => item !== null);

};

export async function stopTunnel(): Promise<{ error: string | null }> {
	return await runCommand("stop_tunnel", true);
};

export async function quickConnect(): Promise<void> {
	await runCommand("quick_connect", true);
};
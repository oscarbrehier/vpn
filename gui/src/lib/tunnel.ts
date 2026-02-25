import { invoke } from "@tauri-apps/api/core";
import { GeoLocation, getGeoLocation } from "./geo";

export interface VpnConfig {
	name: string;
	file_path: string;
	location?: GeoLocation;
}

export async function getConfigurations(): Promise<VpnConfig[]> {

	const confs: string[] = await invoke("get_configs");

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

	try {

		await invoke("stop_tunnel");
		return { error: null };

	} catch (err) {

		const message = err instanceof Error ? err.message : "Unknown error";
		return { error: message };

	};

};
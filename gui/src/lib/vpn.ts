import { invoke } from "@tauri-apps/api/core";
import { getGeoLocation } from "./geo";

export async function getConfigurations() {

	const confs: String[] = await invoke("get_configs");

	const locationsPromises = confs.map(async (conf) => {

		const lastIndex = conf.lastIndexOf('.');
		const ip = lastIndex !== -1 ? conf.substring(0, lastIndex) : conf;

		const res = await getGeoLocation(ip.toString());
		return res;

	});

	const locations = await Promise.all(locationsPromises);
	const validLocations = locations.filter(Boolean);

	return validLocations;

}
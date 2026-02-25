import { invoke } from "@tauri-apps/api/core";

export interface GeoLocation {
	ip: string;
	asn: string;
	as_name: string;
	as_domain: string;
	country_code: string;
	country: string;
	continent_code: string;
	continent: string;
};

export async function getGeoLocation(ip_address?: string): Promise<GeoLocation | null> {

	try {

		let data = await invoke<GeoLocation>("get_geo_info", {
			ip: ip_address
		});

		return data;

	} catch (err) {
		return null;
	}

};

export async function getGeoLocation(ip_address?: string): Promise<any | null> {

	try {

		let url = `http://ip-api.com/json`;
		if (ip_address) url += `/${ip_address}`;

		const res = await fetch(url);
		const data = await res.json();

		return data;

	} catch (err) {
		return null;
	}

};

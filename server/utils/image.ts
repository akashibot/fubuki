import { decode } from "imagescript";
import { ofetch } from "ofetch";
import sharp from "sharp";

export async function safeDecode(buffer: Uint8Array) {
	const image = sharp(buffer);
	const metadata = await image.metadata();

	return decode(
		await image[
			metadata.format === "webp" ? "png" : metadata.format
		]().toBuffer(),
	);
}

export async function readFromUrl(url: string) {
	const data = await ofetch<ArrayBuffer, "arrayBuffer">(url, {
		responseType: "arrayBuffer",
	});

	return safeDecode(Buffer.from(data));
}

export async function readFontFromUrl(url: string) {
	const data = await ofetch<ArrayBuffer, "arrayBuffer">(url, {
		responseType: "arrayBuffer",
	});

	return new Uint8Array(data);
}

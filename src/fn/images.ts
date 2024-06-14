import { ofetch } from "ofetch";
import sharp from "sharp";

export async function loadFromUrl(url: string) {
	return ofetch<ArrayBuffer, "arrayBuffer">(url, {
		responseType: "arrayBuffer",
		onRequestError: () => {
			throw new Error(`An error has occurred while loading image: '${url}'`);
		},
	});
}

export async function toSharpBuffer(url: string) {
	return await sharp(await loadFromUrl(url)).toBuffer({
		resolveWithObject: true,
	});
}

export async function toSharp(url: string) {
	return sharp(await loadFromUrl(url));
}

export async function createBox(width: number, height: number) {
	return sharp({
		create: {
			width,
			height,
			channels: 4,
			background: { r: 255, g: 255, b: 255, alpha: 1 },
		},
	});
}

export async function createText(
	text: string,
	width: number,
	height: number,
	color?: string,
) {
	return sharp({
		text: {
			text: `<span background="white" color="${color ?? "black"}">${text}</span>`,
			align: "center",
			width,
			height, // padding
			rgba: true,
			font: "Impact",
			wrap: "word",
		},
	});
}

import { createStorage, prefixStorage } from "unstorage";

export const cache = createStorage();
export const memeCache = prefixStorage(cache, "meme");

export async function getCachedSpeech(url: string) {
	return memeCache.getItemRaw<Buffer>(getSpeechCacheKey(url)).catch(() => null);
}

export async function createCachedSpeech(url: string, buffer: Buffer) {
	return memeCache.setItemRaw<Buffer>(getSpeechCacheKey(url), buffer);
}

export async function getCachedCaption(url: string, text: string) {
	return memeCache
		.getItemRaw<Buffer>(getCaptionCacheKey(url, text))
		.catch(() => null);
}

export async function createCachedCaption(
	url: string,
	text: string,
	buffer: Buffer,
) {
	return memeCache.setItemRaw<Buffer>(getCaptionCacheKey(url, text), buffer);
}

export const getSpeechCacheKey = (url: string) => `speech:${url}`;
export const getCaptionCacheKey = (url: string, text: string) =>
	`caption:${url}:${encodeURIComponent(text)}`;

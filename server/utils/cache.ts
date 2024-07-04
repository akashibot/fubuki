import { createStorage } from "unstorage";
import lruCacheDriver from "unstorage/drivers/lru-cache";

export const cache = createStorage({
	driver: lruCacheDriver({ maxSize: 10 }),
});

export function makeCacheKey(name: string, url: string) {
	return `${name}:${url}`;
}

export async function writeToCache(
	key: string,
	buffer: Buffer,
	format = "png",
) {
	return cache
		.setItemRaw<{ buffer: Buffer; format: string }>(key, {
			buffer,
			format,
		})
		.catch(() => undefined);
}

export async function readFromCache(key: string) {
	return cache.getItemRaw<{ buffer: Buffer; format: string }>(key);
}

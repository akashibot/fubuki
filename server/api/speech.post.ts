import { generateSpeech } from "~/utils/meme";

interface RouteBody {
	image: string;
}

export default eventHandler(async (event) => {
	const body = await readBody<RouteBody>(event);

	const cache = await readFromCache(makeCacheKey("speech", body.image));

	if (cache) {
		setHeader(event, "Content-Type", `image/${cache.format}`);

		return cache.buffer;
	}

	const { format, buffer } = await generateSpeech(body.image);

	setHeader(event, "Content-Type", `image/${format}`);

	return buffer;
});

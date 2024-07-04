import { generateCaption } from "~/utils/meme";

interface RouteBody {
	image: string;
	text: string;
}

export default eventHandler(async (event) => {
	const body = await readBody<RouteBody>(event);

	const cache = await readFromCache(makeCacheKey("caption", body.image));

	if (cache) {
		setHeader(event, "Content-Type", `image/${cache.format}`);

		return cache.buffer;
	}

	const { format, buffer } = await generateCaption(body.image, body.text);

	setHeader(event, "Content-Type", `image/${format}`);

	return buffer;
});

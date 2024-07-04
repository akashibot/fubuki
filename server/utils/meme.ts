import { Frame, GIF, Image, TextLayout } from "imagescript";
import { readFontFromUrl, readFromUrl } from "./image";

export async function generateCaption(url: string, text: string) {
	const image = await readFromUrl(url);
	const font = await readFontFromUrl(
		"https://github.com/matmen/ImageScript/raw/master/tests/fonts/opensans%20bold.ttf",
	);

	if (!image) throw createError("Image not found");

	if (image instanceof Image) {
		const captionBox = new Image(image.width, image.height / 2).fill(
			0xffffffff,
		);
		const base = new Image(image.width, image.height + captionBox.height);

		const captionTextLayout = new TextLayout({
			maxWidth: captionBox.width - 30, // X Padding
			maxHeight: captionBox.height - 30, // Y Padding
			wrapStyle: "word",
			verticalAlign: "right",
			horizontalAlign: "middle",
		});

		const captionText = await Image.renderText(
			font,
			54,
			text,
			0x00000fff,
			captionTextLayout,
		);

		base
			.composite(captionBox, 0, 0)
			.composite(image, 0, captionBox.height)
			.composite(
				captionText,
				image.width / 2 - captionText.width / 2,
				captionBox.height / 2 - captionText.height / 2,
			);

		if (base.width < 50 ?? base.height < 50)
			base.resize(base.width * 3, base.height * 3);

		const buffer = await base.encode(undefined, {
			author: "akashi",
			comment: "hehe you got me",
		});

		await writeToCache(
			makeCacheKey("caption", url),
			Buffer.from(buffer),
			"png",
		);

		return {
			buffer,
			format: "png",
		};
	}

	throw new Error("GIF support not implemented");
}

export async function generateSpeech(url: string) {
	const image = await readFromUrl(url);
	const balloon = await readFromUrl(
		"https://github.com/freespywares/stufffffffff/blob/main/z0nqjst12ih61.jpg?raw=true",
	);

	if (!image) throw createError("Image not found");

	if (image instanceof Image) {
		image
			.fit(image.width, image.height + (balloon.height - 100) * 2)
			.composite(
				balloon.resize(image.width, balloon.height - 100) as Image,
				0,
				0,
			)
			.crop(0, 0, image.width, image.height - balloon.height);

		const buffer = await image.encode(undefined, {
			author: "akashi",
			comment: "hehe you got me",
		});

		await writeToCache(makeCacheKey("speech", url), Buffer.from(buffer), "png");

		return {
			buffer,
			format: "png",
		};
	}

	const frames = [];

	for (const frame of image) {
		frames.push(
			frame
				.fit(image.width, image.height + (balloon.height - 100) * 2)
				.composite(
					balloon.resize(image.width, balloon.height - 100) as Image,
					0,
					0,
				)
				.crop(0, 0, image.width, image.height - balloon.height),
		);
	}

	const gif = new GIF(frames);

	const buffer = await gif.encode();

	await writeToCache(makeCacheKey("speech", url), Buffer.from(buffer), "gif");

	return {
		buffer,
		format: "gif",
	};
}

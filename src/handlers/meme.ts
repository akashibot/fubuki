import { defineEventHandler, readBody, setHeader } from "h3";
import type { CaptionBody, MemeBody } from "../types/bodies";
import { createBox, createText, toSharp, toSharpBuffer } from "../fn/images";
import sharp from "sharp";
import {
	createCachedCaption,
	createCachedSpeech,
	getCachedCaption,
	getCachedSpeech,
} from "../fn/cache";

export const captionHandler = defineEventHandler(async (event) => {
	const body = await readBody<CaptionBody>(event);

	setHeader(event, "Content-Type", "image/png");

	const isCached = await getCachedCaption(body.image, body.caption);

	if (isCached) return Buffer.from(isCached);

	const { data: imgBuffer, info: imgMetadata } = await toSharpBuffer(
		body.image,
	);

	const captionWidth = Math.floor(imgMetadata.width);
	const captionHeight = Math.floor(imgMetadata.height / 2);

	const base = await createBox(
		captionWidth,
		imgMetadata.height + captionHeight,
	);

	const captionBox = await createBox(captionWidth, captionHeight);

	const captionText = await createText(
		body.caption,
		Math.round(captionWidth / 2),
		captionHeight - 50,
	);

	base.composite([
		{
			input: await captionBox
				.composite([
					{
						input: await captionText.png().toBuffer(),
						gravity: sharp.gravity.center,
					},
				])
				.png()
				.toBuffer(),
			top: 0,
			left: 0,
		},
		{
			input: imgBuffer,
			top: captionHeight,
			left: 0,
		},
	]);

	const buffer = await base.png().toBuffer();

	await createCachedCaption(body.image, body.caption, buffer);

	return buffer;
});

export const speechHandler = defineEventHandler(async (event) => {
	const body = await readBody<MemeBody>(event);

	setHeader(event, "Content-Type", "image/png");

	const isCached = await getCachedSpeech(body.image);

	if (isCached) return Buffer.from(isCached);

	const balloon = await toSharp(
		"https://raw.githubusercontent.com/akashibot/.github/main/assets/templates/z0nqjst12ih61.jpg",
	);
	const { data: target, info: targetInfo } = await toSharpBuffer(body.image);

	if (!targetInfo.width || !targetInfo.height)
		throw new Error("Invalid image size");

	const balloonHeight = targetInfo.height / 2;

	balloon.resize({
		fit: "fill",
		width: targetInfo.width,
		height: balloonHeight,
	});

	const base = await createBox(
		targetInfo.width,
		targetInfo.height + balloonHeight,
	);

	base.composite([
		{
			input: await balloon.png().toBuffer(),
			top: 0,
			left: 0,
		},
		{
			input: target,
			top: balloonHeight,
			left: 0,
		},
	]);

	const buffer = await base.png().toBuffer();

	await createCachedSpeech(body.image, buffer);

	return buffer;
});

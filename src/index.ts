import { createApp, createRouter, toWebHandler } from "h3";
import { captionHandler, speechHandler } from "./handlers/meme";

const publicPort: number = 5000;
const app = createApp();
const router = createRouter();

router.post("/meme/caption", captionHandler);
router.post("/meme/speech", speechHandler);

app.use(router);

const bunHandler = toWebHandler(app);

Bun.serve({
	port: publicPort,
	fetch: (req) => bunHandler(req),
});

console.info(`Listening on port ${publicPort}`);

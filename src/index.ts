import { createApp, createRouter, toWebHandler } from "h3";
import {
	createIPX,
	ipxFSStorage,
	ipxHttpStorage,
	createIPXH3Handler,
} from "ipx";
import { captionHandler, speechHandler } from "./handlers/meme";

const ipx = createIPX({
	storage: ipxFSStorage({ dir: "./public" }),
	httpStorage: ipxHttpStorage({ allowAllDomains: true }),
});

const publicPort: number = 5000;
const app = createApp();
const router = createRouter();

router.post("/meme/caption", captionHandler);
router.post("/meme/speech", speechHandler);

app.use(router);
app.use("/ipx", createIPXH3Handler(ipx));

const bunHandler = toWebHandler(app);

Bun.serve({
	port: publicPort,
	fetch: (req) => bunHandler(req),
});
console.log("[API]", "READY", "+", "On port %s", publicPort);

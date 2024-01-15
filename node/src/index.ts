import express, { Express, Request, Response } from "express";
import { middleware } from "./middleware";
import { test } from "./controllers";

// make random port number
const port = 8000;
const app: Express = express();

app.get("/", (_req: Request, res: Response) => {
  res.send("HELLO FROM EXPRESS");
});

app.get("/hi", (_req: Request, res: Response) => {
  res.send("BYEEE!!");
});

// actual use case where middleware has to check auth before allowing route
app.get("/coins/:userID", middleware, test);

app.listen(port, () => {
  console.log(`now listening on port ${port}`);
});

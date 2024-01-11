import express, { Express, Request, Response } from "express";
const port = 8000;

const app: Express = express();

app.get("/", (_req: Request, res: Response) => {
  res.send("HELLO FROM EXPRESS");
});

app.get("/hi", (_req: Request, res: Response) => {
  res.send("BYEEE!!");
});

app.listen(port, () => {
  console.log(`now listening on port ${port}`);
});

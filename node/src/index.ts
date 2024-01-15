import express, { Express, Request, Response } from "express";
import { middleware } from "./middleware";
import { middlwareController, test } from "./controllers";

// make random port number
const port = 8000;
const app: Express = express();

// INITIAL API ROUTE
app.get("/", (_req: Request, res: Response) => {
  res.send("HELLO FROM EXPRESS");
});

// Test case to see a route without any middleware
app.get("/test", test);

// actual use case where middleware has to check auth before allowing route
app.get("/coins/:userID", middleware, middlwareController);

app.listen(port, () => {
  console.log(`now listening on port ${port}`);
});

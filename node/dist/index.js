"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
const express_1 = __importDefault(require("express"));
const middleware_1 = require("./middleware");
const controllers_1 = require("./controllers");
// make random port number
const port = 8000;
const app = (0, express_1.default)();
// INITIAL API ROUTE
app.get("/", (_req, res) => {
    res.send("HELLO FROM EXPRESS");
});
// Test case to see a route without any middleware
app.get("/test", controllers_1.test);
// actual use case where middleware has to check auth before allowing route
app.get("/coins/:userID", middleware_1.middleware, controllers_1.middlwareController);
app.listen(port, () => {
    console.log(`now listening on port ${port}`);
});

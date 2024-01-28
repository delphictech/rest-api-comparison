"use strict";
var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.middleware = void 0;
const data_1 = require("../utils/data");
/**
 * the middleware function that runs whenever someone tries to access certain api routes
 *
 * @param {Request} req
 * @param {Response} res
 * @param {NextFunction} next
 * @return {*}
 */
const middleware = (req, res, next) => __awaiter(void 0, void 0, void 0, function* () {
    // grab the userID from the params
    const { userID } = req.params;
    // grab the authtoken from the headers
    const { authtoken } = req.headers;
    console.log("userID", userID, "authToken", authtoken, "mock user data", data_1.mockLoginDetails[userID]);
    try {
        if (data_1.mockLoginDetails[userID].authToken === authtoken) {
            return next();
        }
        return res.sendStatus(403).send("NOT AUTHORIZED");
    }
    catch (e) {
        return res.json({ message: "Internal Error" });
    }
});
exports.middleware = middleware;

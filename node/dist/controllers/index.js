"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.test = exports.GetCoinBalance = void 0;
const data_1 = require("../utils/data");
/**
 * Sends the balance and the username back to the client
 *
 * @param {Request} req
 * @param {Response} res
 */
const GetCoinBalance = (req, res) => {
    const { userID } = req.params;
    res.status(200).json({
        data: { balance: data_1.mockCoinDetails[userID].balance, userName: userID },
    });
};
exports.GetCoinBalance = GetCoinBalance;
/** Test case, just sending basic json response */
const test = (_req, res) => {
    res.json({ message: "testing route", code: 200 });
};
exports.test = test;

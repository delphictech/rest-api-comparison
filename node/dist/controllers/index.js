"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.test = exports.middlwareController = void 0;
const data_1 = require("../utils/data");
/**
 *
 *
 * @param {Request} req
 * @param {Response} res
 */
const middlwareController = (req, res) => {
    const { userID } = req.params;
    res.status(200).json({
        data: { balance: data_1.mockCoinDetails[userID].balance, userName: userID },
    });
};
exports.middlwareController = middlwareController;
/** Test case, just sending basic response */
const test = (_req, res) => {
    res.send("TESTING ROUTE");
};
exports.test = test;

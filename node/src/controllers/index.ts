import { Request, Response } from "express";
import { mockCoinDetails } from "../utils/data";

/**
 * 
 *
 * @param {Request} req
 * @param {Response} res
 */
export const GetCoinBalance = (req: Request, res: Response) => {
  const { userID } = req.params;

  res.status(200).json({
    data: { balance: mockCoinDetails[userID].balance, userName: userID },
  });
};

/** Test case, just sending basic response */
export const test = (_req: Request, res: Response) => {
  res.send("TESTING ROUTE");
};

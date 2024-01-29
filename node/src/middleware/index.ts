import { NextFunction, Request, Response } from "express";
import { mockLoginDetails } from "../utils/data";
/**
 * the middleware function that runs whenever someone tries to access certain api routes
 *
 * @param {Request} req
 * @param {Response} res
 * @param {NextFunction} next
 * @return {*}
 */
export const middleware = async (
  req: Request,
  res: Response,
  next: NextFunction
) => {
  try {
    // grab the userID from the params
    const { userID } = req.params;
    // grab the authtoken from the headers
    const { authtoken } = req.headers;

    console.log(
      "userID",
      userID,
      "authToken",
      authtoken,
      "mock user data",
      mockLoginDetails[userID]
    );

    if (mockLoginDetails[userID].authToken === authtoken) {
      return next();
    }
    return res.status(403).send("NOT AUTHORIZED");
  } catch (e) {
    return res.json({ message: "Internal Error" });
  }
};

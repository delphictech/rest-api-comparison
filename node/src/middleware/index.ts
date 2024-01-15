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
  const { userID } = req.params;
  const { authtoken } = req.headers;
  console.log("userID", userID, "authToken", authtoken);
  console.log(mockLoginDetails[userID]);

  try {
    if (mockLoginDetails[userID].authToken === authtoken) {
      return next();
    }
    return res.sendStatus(403).send("NOT AUTHORIZED");
  } catch (e) {
    return res.json({ message: "Internal Error" });
  }
};

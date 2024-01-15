import { NextFunction, Request, Response } from "express";
import { mockLoginDetails } from "../data";

export const middleware = async (
  req: Request,
  res: Response,
  next: NextFunction
) => {
  const { userID } = req.params;
  const { authtoken } = req.headers;
  console.log("userID", userID, "authToken", authtoken);

  try {
    if (mockLoginDetails[userID].authToken === authtoken) {
      return next();
    }
    return res.sendStatus(403);
  } catch (e) {
    return res.json({ message: "Internal Error" });
  }
};

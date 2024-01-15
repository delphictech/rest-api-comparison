import { LoginDetails, GenericAuthObject } from "./types/api";

export const mockLoginDetails: GenericAuthObject<LoginDetails> = {
  alex: {
    authToken: "123",
    userName: "alex",
  },
  jason: {
    authToken: "234",
    userName: "jason",
  },
  marie: {
    authToken: "345",
    userName: "marie",
  },
};
export const mockCoinDetails = {
  alex: {
    coins: 100,
  },
  jason: {
    coins: 200,
  },
  marie: {
    coins: 300,
  },
};

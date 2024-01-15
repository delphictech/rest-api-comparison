import {
  LoginDetails,
  GenericAuthObject,
  CoinBalanceResponse,
} from "../types/api";

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
export const mockCoinDetails: GenericAuthObject<CoinBalanceResponse> = {
  alex: {
    balance: 100,
  },
  jason: {
    balance: 200,
  },
  marie: {
    balance: 300,
  },
};

import {
  LoginDetails,
  GenericAuthObject,
  CoinBalanceResponse,
} from "../types/api";

// Login/Auth detauils
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

// Balance object for the users
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

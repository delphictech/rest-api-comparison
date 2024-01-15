export interface CoinBalanceResponse {
  authToken: number;
  balance: number;
}

export interface CoinBalanceParams {
  userName: string;
}

export interface LoginDetails {
  authToken: string;
  userName: string;
}

export interface GenericAuthObject<T> {
  [key: string]: T;
}

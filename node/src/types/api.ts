export interface CoinBalanceResponse {
  balance: number;
}

export interface LoginDetails {
  authToken: string;
  userName: string;
}

export interface GenericAuthObject<T> {
  [key: string]: T;
}

// balance type
export interface CoinBalanceResponse {
  balance: number;
}

// Login type
export interface LoginDetails {
  authToken: string;
  userName: string;
}

// generic object type 
export interface GenericAuthObject<T> {
  [key: string]: T;
}

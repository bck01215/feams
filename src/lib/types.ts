type Token = {
    access_token: string;
    expires_in: number;
    refresh_token: string;
    scope: string;
    token_type: string;
  };
  
  type User = {
    login_date: number;
    name: string;
    token: Token;
  };
  
  export type { User, Token };
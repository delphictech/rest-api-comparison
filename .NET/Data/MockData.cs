using System.Collections.Generic;
using MyProject.Models;

namespace MyProject.Data
{
    public static class MockData
    {
        public static readonly Dictionary<string, LoginDetails> MockLoginDetails = new Dictionary<string, LoginDetails>
        {
            { "alex", new LoginDetails { AuthToken = "123", UserName = "alex" } },
            { "jason", new LoginDetails { AuthToken = "234", UserName = "jason" } },
            { "marie", new LoginDetails { AuthToken = "345", UserName = "marie" } }
        };

        public static readonly Dictionary<string, CoinBalanceResponse> MockCoinDetails = new Dictionary<string, CoinBalanceResponse>
        {
            { "alex", new CoinBalanceResponse { Balance = 100 } },
            { "jason", new CoinBalanceResponse { Balance = 200 } },
            { "marie", new CoinBalanceResponse { Balance = 300 } }
        };
    }
}

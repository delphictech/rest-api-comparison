package tools

type mockDB struct {}

var MockLoginDetails = map[string]LoginDetails {
	"alex": {
		AuthToken: "123",
		userName: "alex",
	  },
	"jason": {
		AuthToken: "a234",
		userName: "jason",
	  },
	"marie": {
		AuthToken: "b345",
		userName: "marie",
	  },
}

var MockCoinDetails = map[string]CoinBalanceDetails {
	"alex": {
		balance: 100,
	},
	"jason": {
		balance: 200,
	},
	"marie": {
		balance: 300,
	},
}




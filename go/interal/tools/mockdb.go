package tools

type mockDB struct {}

var MockLoginDetails = map[string]LoginDetails {
	"alex": {
		authToken: "123",
		userName: "alex",
	  },
	"jason": {
		authToken: "a234",
		userName: "jason",
	  },
	"marie": {
		authToken: "b345",
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




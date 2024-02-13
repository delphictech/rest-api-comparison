package tools

var MockLoginDetails = map[string]LoginDetails{
	"alex": {
		AuthToken: "123",
		userName:  "alex",
	},
	"jason": {
		AuthToken: "234",
		userName:  "jason",
	},
	"marie": {
		AuthToken: "345",
		userName:  "marie",
	},
}

var MockCoinDetails = map[string]CoinBalanceDetails{
	"alex": {
		Balance: 100,
	},
	"jason": {
		Balance: 200,
	},
	"marie": {
		Balance: 300,
	},
}

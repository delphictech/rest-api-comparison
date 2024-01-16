package api

// referncing the golang-standards/api/README.md
// We can see that the api folder is similiar to a utils or types folder in ts
// This folder should define certain parts of our api. In our case just statically typing our api



type CoinBalanceParams struct {
	Username string
}

type CoinBalanceResponse struct {
	Code int
	Balance int64
}

type Error struct {
	Code int
	Message string
}
  

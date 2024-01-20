package tools

type LoginDetails struct {
AuthToken string
userName string
}

type CoinBalanceDetails struct {
 balance int64
}


type DatabaseInterface interface {
	GetUserLoginDetails(username string) *LoginDetails
	GetUserCoins(username string) *CoinBalanceDetails
	SetupDatabase() error
}

// func NewDatabase(name string, token string) (LoginDetails) {

// 	// var database DatabaseInterface = &mockDB{}

// 	// var err error = database.SetupDatabase()

// 	// if err != nil {
// 	// 	log.Error(err)
// 	// 	return nil, err
// 	// }



// 	// return &database, nil


// 	return mockLoginDetails[name]

// }
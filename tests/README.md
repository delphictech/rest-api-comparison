## Testing 

Unit tests are in place to make testing these API routes easier.

These unit tests should work regardless of your language if your server runs on the correct URL. Before running these tests please make sure you have [node](https://github.com/promethean-tech/rest-api-comparison/tree/main/node#prerequisites) installed on your local machine 

To run these unit tests:

- start your local server at localhost8000

```bash
cd tests
npm install
npm run test
```
You can also run specific test files by running

```bash
npm run test api_error.test.ts
```
```bash
npm run test api_success.test.ts
```
```bash
npm run test api_test.ts
```

- Please note that even if you don't have the server running, at least 1 unit test should pass ensuring that you have the tests folder configured/installed properly.


## API Data Requirements

These tests require that each route returns the same data, regardless of the language used. Down below is a list of the required responses back from the server to make these tests pass.


### http://localhost:8000/

return data:

```bash
"HELLO WORLD"
```

### http://localhost:8000/test

return data:

```bash
{
message: "testing route"
code: 200
}
```

### http://localhost:8000/coins/alex (Success)

return data:

```bash
{
username: "alex"
balance: 100
}
```

### http://localhost:8000/coins/alex (Error)

Needs to return some form of [403/Forbidden](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/403) error to the users.

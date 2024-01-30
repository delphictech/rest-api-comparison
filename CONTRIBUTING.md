# Contributing to rest-api-comparison

Thank you for your interest in contributing to this project! Before you get started, please take a moment to read through the following guidelines.

## Getting Started

- Fork the repository.
- Clone your forked repository locally.
- Create a new branch on your local machine
- If you wish to test or verify other language APIs, please follow the specific README.md inside their designated folder.

## Issues

- Find or create issues on the [Issues page](https://github.com/promethean-tech/go-vs-node-api/issues).
- Follow issue labeling guidelines.
- Specify if the issue is a bug fix or a language addition.

## Pull Requests

- If not done already, create a new branch for your feature, language addition, or bugfix with a relevant branch name to the issue.
- Make your changes and commit.
- Create a pull request, referencing the related issue.
- Additional information in the Pull-Request is appreciated, but not needed if documentation/README is sufficient.

## Coding Standards

- Since this repo, is a collaboration of many different languages/frameworks, many different standards will have to be used to best match the language.
- In your language's README.md file please provide several links or resources to the "Coding Standards" section of the README. This will provide the maintainers of the repo to inspect your code to make sure, you are using best practices for your specific language/framework.
- You can look at the Golang example [here](https://github.com/promethean-tech/rest-api-comparison/tree/main/go)
- Without these resources, your PR will not be merged.

# Language API requirements

- You need to make a server that runs on your local machine running at "http://localhost:8000"
- Since this repo is primarily aimed at beginners, it only requires 3 REST endpoints.
  
  
     * http://localhost:8000/
     this route will just return some message to the user running it on their local machine. EX: "Hello from GO!" 
    
     * http://localhost:8000/test
     this route will just return some json or an object for the user to see EX {message: "Hello again!", code: 200}

     * http://localhost:8000/coins/{userID}
     this route is much more complicated. It requires middleware to check the user's auth instance by checking the headers inside the request and validate 
     the user's token before allowing them access to the route. After this validation, it returns the user's token  
     amount (See more down below on how to do this).
  
- These three routes are required, but any additional routes/examples are encouraged.
- Use this data for user auth validation
```javascript
const mockLoginDetails = {
  alex: {
    authToken: "123",
    userName: "alex",
  },
  jason: {
    authToken: "234",
    userName: "jason",
  },
  marie: {
    authToken: "345",
    userName: "marie",
  },
};
const mockCoinDetails = {
  alex: {
    balance: 100,
  },
  jason: {
    balance: 200,
  },
  marie: {
    balance: 300,
  },
};
```

- For example, if you go to route localhost:8000/coins/alex and you have an input called "authtoken" inside the headers set to "123" then you could receive his balance of 100 tokens. If the username or authToken is not set correctly inside the headers, then this route will throw an error  explaining that they are not permitted to access this route.

## Testing 

A testing folder is in place that will test all three of these basic API routes. Before creating your PR please make sure all of the unit tests pass.

To run these unit tests, run your local server at localhost8000

```bash
cd tests
npm install
npm run test
```
- Please note that even if you don't have the server running, at least 1 unit test should pass ensuring that you have the tests folder configured/installed properly.
- For more testing requirements, please check the tests [README.md](https://github.com/promethean-tech/rest-api-comparison/tree/dev/tests)

## Documentation

If you are adding a language to the repo, please copy this as a baseline README.md for your language. If your language's README.md does not contain at least these fields, your PR will not be merged. The node README example can be found [here](https://github.com/promethean-tech/rest-api-comparison/tree/main/node#learning-nodejs-rest-api-project)

```
# Learning [Your language or framework] REST API Project

  *** Brief 1-3 sentences talking about the project/language.

## Getting Running

### Installation/Prerequisites

  Describe how to install the language onto your local machine.

### Running Project

  Describe how you can run the server on your machine and test the 3 required endpoints.

### Language Project layout resources

  *** 1-3 links providing the correct project layout for your language.

### Language Specifics
  
  *** 1-3 links providing links to learn the language or documentation

#### Summary

  *** Summary of the language, why might it be useful to work with? Pros vs cons.

```

## Bug Fixes or Minor additions
- Follow the general rules above.
- If you make changes to the code layout/format please provide links to why the changes were made and update the "Language Project layout resources" in the readme

## License

This project is licensed under the MIT license. See the [LICENSE](https://opensource.org/license/mit/) file for details.

# Learning Golang REST API Project

This is a basic Go project created for learning purposes. The project emphasizes making REST API calls and implementing middleware for a simple web application.

## Getting Running

### Installation/Prerequisites

Ensure you have [Go](https://golang.org/dl/) installed on your machine.

- Use the link above to install the latest version of Go, using an exe
- Or you can install a specific version of go using [ASDF](https://github.com/asdf-community/asdf-golang)

  Once the installation process is finished, you can check if it was successful by running

 ```bash
go version
  ```
  
### Running Project

Run these commands to run the project on localhost:8000

```bash
cd go
go mod tidy
go get -u
go run cmd/api/main.go
  ```

This will start the server at localhost8000. Please note that there is no listener on the go code, so any changes you make to the code will require you to restart the server.

## Resources

### Language Project layout resources

  - [golang-standards/project-layout](https://github.com/golang-standards/project-layout) official GitHub repo example
  - [Go Organization](https://go.dev/doc/modules/layout) recomendations 
    

### Language Specifics
  
  - [Middleware Patterns](https://drstearns.github.io/tutorials/gomiddleware/) in go
  - [chi](https://pkg.go.dev/github.com/go-chi/chi) go package used for setting up the REST API. Other frameworks such as [gin](https://gin-gonic.com/) and many others use the exact same layout/formatting as chi, so you can choose any package you like since they are all so similar.
  - Go has many cool features, such as multiple thread computation, and many others that did not get touched in this project. Check out [a tour of go](https://go.dev/tour/concurrency/1) to better understand some of its best features.

#### Summary

  Go is an incredibly simple language, that offers performance very close to c++ or rust. With this statement alone, many companies are switching to go to increase development speed without costing them any reasonable amount of performance. Because of this, go will continue to grow and be a great solution to small-medium-sized companies looking for great performance and an easy language to learn.

If you would like to compare go and rust [here](https://blog.logrocket.com/when-to-use-rust-when-to-use-golang/) is a good article!





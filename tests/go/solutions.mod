// Same as go.mod, but the tests run against solutions/go: go test -modfile=solutions.mod ./...
module tests

go 1.23

require exercises v0.0.0

replace exercises => ../../solutions/go

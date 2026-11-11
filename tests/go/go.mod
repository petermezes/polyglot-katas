// Tests for exercises/go. Run against the solutions with: go test -modfile=solutions.mod ./...
module tests

go 1.23

require exercises v0.0.0

replace exercises => ../../exercises/go

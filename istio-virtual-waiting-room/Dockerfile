FROM golang:1.22

WORKDIR /app

# Copy go mod and sum files
COPY go.mod go.sum ./

# Download all dependencies
RUN go mod download

# Copy the source code
COPY main.go .

# Build the application
RUN go build -o main .

# Run the application
CMD ["./main"]

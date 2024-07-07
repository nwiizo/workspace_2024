FROM golang:1.22 AS build

WORKDIR /app

COPY go.mod go.sum ./
RUN go mod download

COPY . .

RUN CGO_ENABLED=0 GOOS=linux go build -o /weblitechecker

FROM scratch
COPY --from=build /weblitechecker /weblitechecker
ENTRYPOINT ["/weblitechecker"]

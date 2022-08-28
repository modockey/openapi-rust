IP_ADDRESS = 1.1.1.1

setup:
	docker compose up -d

down:
	docker compose down

run: setup
	cd ./server/api && cargo run

curl-get:
	curl -X GET localhost:8080/ip -i

curl-post:
	curl -X POST localhost:8080/ip -H "Content-Type: application/json" -d '{"IPv4_address":"${IP_ADDRESS}"}' -i
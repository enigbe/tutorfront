tutors-server: 
	cargo run -p tutors --bin server

tutors-service:
	cargo run -p tutors

start-deps:
	docker compose up -d db

clean-deps:
	docker compose down

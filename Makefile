up:
	docker-compose -f docker/docker-compose.yml build;
	docker-compose -f docker/docker-compose.yml up;

psql:
	docker-compose -f docker/docker-compose.yml exec db psql -U user -d flash_card_db

api-hello:
	curl -X GET http://localhost:8000

api-get-flash-cards:
	curl -X GET http://localhost:8000/flash_card

api-add-flash-card:
	curl -X POST http://localhost:8000/flash_card \
	-H "Content-Type: application/json" \
	-d  '{    \
		"question": "What is Rust?", \
		"answer": "Rust is a systems programming language.", \
		"learning_topic": "Programming" \
	}' 

api-batch-flash-card:
	curl -X POST -F "file=@./app/csv_files/test.csv" http://localhost:8000/batch_csv_import

api-get-learning-topic:
	curl -X GET http://localhost:8000/learning_topic

api-add-learning-topic:
	curl -X POST http://localhost:8000/learning_topic \
	-H "Content-Type: application/json" \
	-d '{"subject": "API"}'

api-test:
	@echo "Testing API endpoints..."
	@SUCCESS=""
	@FAILED=""

	@$(MAKE) api-hello && SUCCESS+="api-hello\n" || FAILED+="api-hello\n"
	@$(MAKE) api-get-flash-cards && SUCCESS+="api-get-flash-cards\n" || FAILED+="api-get-flash-cards\n"
	@$(MAKE) api-add-flash-card && SUCCESS+="api-add-flash-card\n" || FAILED+="api-add-flash-card\n"
	@$(MAKE) api-batch-flash-card && SUCCESS+="api-batch-flash-card\n" || FAILED+="api-batch-flash-card\n"
	@$(MAKE) api-get-learning-topic && SUCCESS+="api-get-learning-topic\n" || FAILED+="api-get-learning-topic\n"
	@$(MAKE) api-add-learning-topic && SUCCESS+="api-add-learning-topic\n" || FAILED+="api-add-learning-topic\n"

	@echo "\n=== Test Summary ==="
	@if [ -n "$$SUCCESS" ]; then \
		echo "Success:"; \
		echo -e "$$SUCCESS"; \
	fi
	@if [ -n "$$FAILED" ]; then \
		echo "Failed:"; \
		echo -e "$$FAILED"; \
	else \
		echo "All endpoints tested successfully!"; \
	fi

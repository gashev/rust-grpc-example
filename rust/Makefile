create_docker_registry:
	docker run -d -p 5000:5000 --restart=always --name registry -v /mnt/registry:/var/lib/registry registry:2

stop_docker_registry:
	docker stop registry
	docker rm registry

destroy_docker_registry: stop_docker_registry
	rm -rf /mnt/registry

start_dockers:
	docker-compose up -d

stop_dockers:
	docker-compose down

protoc_rust_docker:
	cd protoc-rust && docker build . -t docker-local:5000/protoc-rust && docker push docker-local:5000/protoc-rust

generate_source_from_proto_file:
	docker run -v $$(pwd):/opt docker-local:5000/protoc-rust:latest protoc --rust_out=. --grpc_out=. --plugin=protoc-gen-grpc=/usr/local/cargo/bin/grpc_rust_plugin books.proto
.DEFAULT_GOAL := all

create_docker_registry:
	docker run -d -p 5000:5000 --restart=always --name registry -v /mnt/registry:/var/lib/registry registry:2

stop_docker_registry:
	docker stop registry
	docker rm registry

destroy_docker_registry: stop_docker_registry
	rm -rf /mnt/registry

start:
	docker-compose up -d

stop:
	docker-compose down

build_docker:
	cd dockers/build && docker build . -t docker-local:5000/build && docker push docker-local:5000/build

generate_source_from_proto_file: build_docker
	docker run -v $$(pwd):/opt docker-local:5000/build:latest protoc --rust_out=. --grpc_out=. --plugin=protoc-gen-grpc=/usr/local/cargo/bin/grpc_rust_plugin books.proto

build: generate_source_from_proto_file
	docker run -v $$(pwd):/opt -w /opt/src docker-local:5000/build:latest /bin/bash -c "cargo build --release"

server_docker:
	cp src/target/release/server ./dockers/server/server 
	cd dockers/server && docker build . -t docker-local:5000/server && docker push docker-local:5000/server 
	rm -f ./dockers/server/server

cli_docker:
	cp src/target/release/cli ./dockers/cli/cli 
	cd dockers/cli && docker build . -t docker-local:5000/cli && docker push docker-local:5000/cli 
	rm -f ./dockers/cli/cli

all: build server_docker cli_docker
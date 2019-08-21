.DEFAULT_GOAL := all

DOCKER_REGISTRY := docker-local:5000

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

create_pod:
	kubectl create -f kubernetes.yaml

delete_pod:
	kubectl delete -f kubernetes.yaml

build_docker:
	cd dockers/build && docker build . -t $(DOCKER_REGISTRY)/build && docker push $(DOCKER_REGISTRY)/build

generate_source_from_proto_file: build_docker
	docker run -v $$(pwd):/opt $(DOCKER_REGISTRY)/build:latest protoc --rust_out=. --grpc_out=. --plugin=protoc-gen-grpc=/usr/local/cargo/bin/grpc_rust_plugin books.proto

build: generate_source_from_proto_file
	docker run -v $$(pwd):/opt -w /opt/src $(DOCKER_REGISTRY)/build:latest /bin/bash -c "cargo build --release"

server_docker:
	cp src/target/release/server ./dockers/server/server
	cd dockers/server && docker build . -t $(DOCKER_REGISTRY)/server && docker push $(DOCKER_REGISTRY)/server
	rm -f ./dockers/server/server

cli_docker:
	cp src/target/release/cli ./dockers/cli/cli
	cd dockers/cli && docker build . -t $(DOCKER_REGISTRY)/cli && docker push $(DOCKER_REGISTRY)/cli
	rm -f ./dockers/cli/cli

all: build server_docker cli_docker
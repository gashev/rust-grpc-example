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

build_docker:
	cd build && docker build . -t docker-local:5000/build && docker push docker-local:5000/build

generate_source_from_proto_file: build_docker
	docker run -v $$(pwd):/opt docker-local:5000/build:latest protoc --rust_out=. --grpc_out=. --plugin=protoc-gen-grpc=/usr/local/cargo/bin/grpc_rust_plugin books.proto

build_server: generate_source_from_proto_file
	docker run -v $$(pwd):/opt -w /opt/server docker-local:5000/build:latest /bin/bash -c "cargo build --release"

grpc_server_docker: build_server
	cp server/target/release/server ./grpc-server 
	cd grpc-server && docker build . -t docker-local:5000/grpc-server && docker push docker-local:5000/grpc-server 
	rm -f ./grpc-server/server
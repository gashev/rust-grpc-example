create_docker_registry:
	docker run -d -p 5000:5000 --restart=always --name registry -v /mnt/registry:/var/lib/registry registry:2

stop_docker_registry:
	docker stop registry
	docker rm registry

destroy_docker_registry: stop_docker_registry
	rm -rf /mnt/registry

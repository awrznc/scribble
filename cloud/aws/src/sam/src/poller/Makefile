.PHONY: build deploy delete docker_up mount_smb unmount_smb

build:
	sam build

deploy: build
	sam deploy

delete:
	sam delete

docker_up:
	docker compose --file .docker/docker-compose.yml up --build --detach

local_run: build docker_up
	sam local invoke --docker-network docker_default; docker compose --file .docker/docker-compose.yml down

mount_container_smb:
	mkdir -p ".debug_mnt/"
	mount -t smbfs "//root:samba_password@localhost/share_example" ".debug_mnt"

unmount_container_smb:
	umount ".debug_mnt"

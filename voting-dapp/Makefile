nctl-up:
	cd nctl-docker && docker compose up

nctl-up-detach:
	cd nctl-docker && docker compose up -d

nctl-down:
	cd nctl-docker && docker compose down

nctl-restart:
	cd nctl-docker && docker compose exec -d mynctl /bin/bash "-c" "chmod +x /home/casper/restart.sh && /home/casper/restart.sh"

nctl-copy-keys:
	cd nctl-docker && docker compose cp mynctl:/home/casper/casper-node/utils/nctl/assets/net-1/users .

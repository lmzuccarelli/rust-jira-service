#!/bin/bash

set -x

USER="${REMOTE_USER:-luzuccar}"
PK="${PK_ID:?PK_ID environment variable must be set}"
MS="jira-service"
DESCRIPTION="A jira (read only) service wriiten in Rust"
REPO="git@github.com:lmzuccarelli/rust-jira-service.git"
REPO_NAME="rust-jira-service"
CLEAN=$2

create_configs() {
tee config/${MS}-config.json <<EOF
{
	"name": "${MS}",
	"description": "${DESCRIPTION}",
	"log_level": "debug",
	"base_url": "https://issues.redhat.com/rest/api/2/issue/",
	"api_key_path" :"/home/${USER}/.jira/token"
}
EOF
}

clone_build_service() {
  HOSTS=("george")
  for host in "${HOSTS[@]}"; do
    ssh -i "${PK}" "${USER}@${host}" -t "rm -rf /home/${USER}/services/${MS}"
    eval `ssh-agent`
    ssh-add ~/.ssh/id_ed25519-lz
    if [ "${CLEAN}" == "true" ];
    then
      ssh -i "${PK}" "${USER}@${host}" -tA "rm -rf /home/${USER}/Projects/${REPO_NAME} && cd /home/${USER}/Projects && git clone ${REPO} && cd ${REPO_NAME} && make build"
    else 
      ssh -i "${PK}" "${USER}@${host}" -tA "cd /home/lzuccarelli/Projects/${REPO_NAME} && rm -rf target/release/*gemini* && git pull origin main --rebase && make build"
    fi
  done
}

deploy_service() {
  HOSTS=("george")
  for host in "${HOSTS[@]}"; do
    scp -i "${PK}" config/* "${USER}@${host}:/home/${USER}/services"
    ssh -i "${PK}" "${USER}@${host}" -t "cp /home/${USER}/Projects/${REPO_NAME}/target/release/${MS} /home/${USER}/services/${MS}"
  done
}

"$@"


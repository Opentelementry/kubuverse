#!/usr/bin/env bash
set -e
cd "$(dirname "$0")/.."

# === Config ===
DOCKER_DIR=dev-support/docker
DOCKER_FILE="${DOCKER_DIR}/Dockerfile"
CONTAINER_NAME=kubuverse-dev-${USER}-$$
DOCKER_INTERACTIVE_RUN=${DOCKER_INTERACTIVE_RUN-"-i -t"}
DOCKER_HOME_DIR=${DOCKER_HOME_DIR:-/home/${USER}}

# === Docker check ===
if ! command -v docker >/dev/null 2>&1; then
    echo "Docker is required. Install it first."
    exit 1
fi

# === Build base image ===
docker build -t kubuverse-build -f $DOCKER_FILE $DOCKER_DIR

# === User mapping ===
USER_NAME=${SUDO_USER:=$USER}
USER_ID=$(id -u "$USER_NAME")
GROUP_ID=$(id -g "$USER_NAME")
DOCKER_GROUP_ID=$(getent group docker | cut -d':' -f3 || echo 1000)

# === Build user-specific image ===
docker build -t "kubuverse-build-${USER_ID}" - <<EOF
FROM kubuverse-build
RUN groupadd --non-unique -g ${GROUP_ID} ${USER_NAME} || true
RUN groupmod -g ${DOCKER_GROUP_ID} docker || true
RUN useradd -g ${GROUP_ID} -G docker -u ${USER_ID} -m ${USER_NAME} -d "${DOCKER_HOME_DIR}" || true
RUN echo "${USER_NAME} ALL=NOPASSWD: ALL" > "/etc/sudoers.d/kubuverse-${USER_ID}"
ENV HOME="${DOCKER_HOME_DIR}"
EOF

# === Docker socket mount ===
DOCKER_SOCKET_MOUNT=""
if [ -S /var/run/docker.sock ]; then
    DOCKER_SOCKET_MOUNT="-v /var/run/docker.sock:/var/run/docker.sock"
fi

# === Default command ===
COMMAND=( "$@" )
[ $# -eq 0 ] && COMMAND=("bash")

# === Ensure cache directories exist ===
mkdir -p "${HOME}/.kubu_docker_build_env/.gradle" "${HOME}/.m2"

# === Run container ===
docker run --rm ${DOCKER_INTERACTIVE_RUN} \
    --name "${CONTAINER_NAME}" \
    --network=host \
    -v "${HOME}/.m2:${DOCKER_HOME_DIR}/.m2" \
    -v "${HOME}/.gnupg:${DOCKER_HOME_DIR}/.gnupg" \
    -v "${HOME}/.kubu_docker_build_env/.gradle:${DOCKER_HOME_DIR}/.gradle" \
    -v "${PWD}:${DOCKER_HOME_DIR}/kubuverse" \
    -w "${DOCKER_HOME_DIR}/kubuverse" \
    ${DOCKER_SOCKET_MOUNT} \
    -u "${USER_ID}" \
    "kubuverse-build-${USER_ID}" "${COMMAND[@]}"

#!/bin/bash -eux

SCRIPT_DIR=$(cd $(dirname $0); pwd)
WORK_DIR="${SCRIPT_DIR}/.tmp/"
mkdir -p ${WORK_DIR}

apt update -y
apt upgrade -y

# docker
apt install -y ca-certificates curl gnupg lsb-release
mkdir -p /etc/apt/keyrings
curl -fsSL https://download.docker.com/linux/debian/gpg | gpg --dearmor -o /etc/apt/keyrings/docker.gpg
echo "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/docker.gpg] https://download.docker.com/linux/debian $(lsb_release -cs) stable" | tee /etc/apt/sources.list.d/docker.list > /dev/null
apt update -y
apt install -y docker-ce docker-ce-cli containerd.io docker-compose-plugin
sudo touch /etc/fstab
sudo update-alternatives --set iptables /usr/sbin/iptables-legacy
sudo update-alternatives --set ip6tables /usr/sbin/ip6tables-legacy

# kubectl
K8S_VERSION=v1.32.0
apt install -y curl
curl "https://dl.k8s.io/release/${K8S_VERSION}/bin/linux/amd64/kubectl" \
    -Lo ${WORK_DIR}/kubectl
install -m 0755 ${WORK_DIR}/kubectl /usr/local/bin/kubectl
kubectl version || true

# kind
KIND_VERSION=v0.22.0
curl https://kind.sigs.k8s.io/dl/${KIND_VERSION}/kind-$(uname)-amd64 \
    -Lo ${WORK_DIR}/kind
install -m 0755 ${WORK_DIR}/kind /usr/local/bin/kind
kind --version

# k9s
curl https://github.com/derailed/k9s/releases/download/v0.32.7/k9s_linux_amd64.deb \
    -Lo ${WORK_DIR}/k9s_linux_amd64.deb
apt install -y ${WORK_DIR}/k9s_linux_amd64.deb

# systemd
cp ${SCRIPT_DIR}/wsl.conf /etc/wsl.conf

# user
USERNAME=kinduser
for var in piyopiyo piyopiyo ${USERNAME} 42 0 0 0 Y; do echo $var; done | adduser ${USERNAME}
sudo usermod -aG docker ${USERNAME}

echo 'finished!'

#/bin/bash -eux

# apt install -y unzip
SCRIPT_DIR_PATH=$(cd $(dirname $0); pwd)
INSTALL_DIR_PATH=${SCRIPT_DIR_PATH}/bin/

mkdir -p ${INSTALL_DIR_PATH}
curl -o ${INSTALL_DIR_PATH}/terraform.zip -L "https://releases.hashicorp.com/terraform/1.7.4/terraform_1.7.4_linux_amd64.zip"
unzip -o ${INSTALL_DIR_PATH}/terraform.zip -d ${INSTALL_DIR_PATH}

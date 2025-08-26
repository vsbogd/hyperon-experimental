#!/bin/bash

function cleanup {
    echo "Stop DAS services"
    das-cli qa stop || true
    das-cli ab stop || true
    das-cli db stop || true
}

set -e
trap cleanup EXIT

echo "Run DAS integration test"
echo "GITHUB_WORKSPACE=${GITHUB_WORKSPACE}"
echo "PYTHON=${PYTHON}"
echo "DAS_VERSION=${DAS_VERSION}"
DB="${GITHUB_WORKSPACE}/integration_tests/das/animals.metta"
echo "DB=${DB}"
TEST="${GITHUB_WORKSPACE}/integration_tests/das/test.metta"
echo "TEST=${TEST}"
METTA="${GITHUB_WORKSPACE}/target/debug/metta-repl"
echo "METTA=${METTA}"

echo "Create Python virtual environment"
VENV=./dastoolbox
"${PYTHON}" -m venv "${VENV}"
source "${VENV}/bin/activate"

echo "Install das-cli"
TEMP_DIR=$(mktemp -d)
cd "$TEMP_DIR"
git clone https://github.com/singnet/das-toolbox.git
cd das-toolbox
git checkout tags/${DAS_VERSION}
cd das-cli/src
# TODO: should be a part of das-cli/setup.py file
"${PYTHON}" -m pip install -r requirements.txt
# TODO: -e should not be needed and temporary dir could be cleaned
# up after installation
"${PYTHON}" -m pip install -e .
cd "${GITHUB_WORKSPACE}"

echo "Configure DAS with default settings"
echo -e "\n\n\n\n\n\n\n37007\n\n\n\n\n\n\n\n\n\n\n" | das-cli config set

echo "Start databases"
das-cli db start

echo "Load knowledge base"
das-cli metta load "${DB}"

echo "Start Attention Broker"
das-cli ab start

echo "Start Query Agent"
echo "52000:52999" | das-cli qa start

echo "Run DAS module tests"
timeout 60s "${METTA}" "${TEST}"

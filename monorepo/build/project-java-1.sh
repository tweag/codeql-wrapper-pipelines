#!/bin/bash
set -x

# Check if Maven is already installed
if command -v mvn &> /dev/null; then
    echo "Maven is already installed:"
    mvn --version
else
    echo "Maven not found. Installing Maven..."
    sudo apt update
    sudo apt install -y maven
    echo "Maven installation completed:"
    mvn --version
fi

cd ../project-java-1
mvn clean package

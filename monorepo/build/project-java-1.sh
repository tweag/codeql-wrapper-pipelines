#!/bin/bash
set -x

sudo apt update
sudo apt install -y maven

cd ../project-java-1
mvn clean package

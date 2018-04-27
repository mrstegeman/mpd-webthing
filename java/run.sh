#!/bin/bash

set -e

mvn clean compile assembly:single
jar=$(find target -maxdepth 1 -name '*-with-dependencies.jar')
java -jar "${jar}"

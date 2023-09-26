#!/bin/bash

curl -X POST -F "file=@./test.sh" http://localhost:8080/upload

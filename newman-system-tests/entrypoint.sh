#!/bin/bash

nohup java -jar app.jar > app.output 2>&1 &
sleep 5
newman run postman_collection.json
TEST_RESULT=$?
kill %1
sleep 5
exit $TEST_RESULT
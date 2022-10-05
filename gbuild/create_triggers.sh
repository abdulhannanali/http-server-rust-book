#!/bin/sh
export ROOT_DIRECTORY=./gbuild

SINGLE_THREADED_CLOUD_BUILD_TRIGGER_NAME="SINGLE_THREADED_TRIGGER"
SINGLE_THREADED_CLOUD_BUILD_CONFIG_NAME=$ROOT_DIRECTORY/singlethread.yml

gcloud beta builds triggers create github \
    --repo-name=http-server-rust-book \
    --repo-owner=abdulhannanali \
    --branch-pattern=master \
    --build-config=$SINGLE_THREADED_CLOUD_BUILD_CONFIG_NAME




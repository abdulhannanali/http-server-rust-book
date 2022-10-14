#!/bin/sh
export ROOT_DIRECTORY=./gbuild

MULTI_THREADED_CLOUD_BUILD_TRIGGER_NAME="MULTI_THREADED_TRIGGER"
MULTI_THREADED_CLOUD_BUILD_CONFIG_NAME=$ROOT_DIRECTORY/multithread.yml

gcloud beta builds triggers create github \
    --repo-name=http-server-rust-book \
    --repo-owner=abdulhannanali \
    --branch-pattern=master \
    --build-config=$MULTI_THREADED_CLOUD_BUILD_CONFIG_NAME

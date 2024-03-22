#!/bin/bash

# DL alpine Rpi image
wget "https://dl-cdn.alpinelinux.org/alpine/latest-stable/releases/armhf/$(wget https://dl-cdn.alpinelinux.org/alpine/latest-stable/releases/armhf/latest-releases.yaml -qO - | yq ".[1].file" -r)"
# So we can ssh in


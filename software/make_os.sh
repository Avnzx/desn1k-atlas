#!/bin/bash
set -e
set -o pipefail

makefile() {
	OWNER="$1"
	PERMS="$2"
	FILENAME="$3"
	cat > "$FILENAME"
	chown "$OWNER" "$FILENAME"
	chmod "$PERMS" "$FILENAME"
}



# Actual script begins here

if [ "$(whoami)" != "root" ]; then
    echo "You must be root to do this"
    exit
fi


# DL alpine Rpi image
echo "[INFO]: Downloading alpine image"
if [ -e "/tmp/alpine/alpine.tar.gz" ]; then
  echo "[WARN]: Skipping alpine download since file already exists"
else
  mkdir -p /tmp/alpine
  wget "https://dl-cdn.alpinelinux.org/alpine/latest-stable/releases/armhf/$(wget https://dl-cdn.alpinelinux.org/alpine/latest-stable/releases/armhf/latest-releases.yaml -qO - | yq ".[0].file" -r)" -O /tmp/alpine/alpine.tar.gz
fi

echo "[INFO]: Extracting alpine..."
mkdir -p /tmp/alpine/build
tar -p -s --atime-preserve --same-owner --one-top-level=/tmp/alpine/build -zxf /tmp/alpine/alpine.tar.gz

# So we can ssh in (overlay)
echo "[INFO]: Downloading headless overlay"
if [ -e "/tmp/alpine/build/headless.apkovl.tar.gz" ]; then
  echo "[WARN]: Skipping download since file already exists"
else
  wget "https://is.gd/apkovl_master" -qO /tmp/alpine/build/headless.apkovl.tar.gz
fi

# TODO: Deploy credentials
makefile root:root 0555 /tmp/alpine/build/wpa_supplicant.conf <<EOF
network={
  ssid="TEST_2451"
  psk="40841320682011562111"
}
EOF


# Enable hardware PWM on both PWM0 and PWM1
makefile root:root 0555 /tmp/alpine/build/usercfg.txt <<EOF
dtoverlay=pwm-2chan
EOF

# Rearchive it all
tar -C /tmp/alpine/build -cpf /tmp/alpine/new_alpine.tar --preserve-permissions --atime-preserve --same-owner .
gzip -9 /tmp/alpine/new_alpine.tar
echo "[DONE] Created image /tmp/alpine/new_alpine.tar.gz, use deploy_os.sh to flash the built OS"

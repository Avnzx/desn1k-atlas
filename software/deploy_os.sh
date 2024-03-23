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
if [ -a "/tmp/alpine/alpine.tar.gz" ]; then
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
if [ -a "/tmp/alpine/build/headless.apkovl.tar.gz" ]; then
  echo "[WARN]: Skipping download since file already exists"
else
  wget "https://is.gd/apkovl_master" -qO /tmp/alpine/build/headless.apkovl.tar.gz
fi
# TODO: deploy credentials


# install scp
echo "[INFO]: Creating scp (install openssh) overlay"

tmp="$(mktemp -d)"
mkdir -p "$tmp"/etc/etc/init.d && mkdir -p "$tmp"/etc/runlevels/default

makefile root:root 0755 "$tmp"/etc/etc/init.d/scp_bootstrap <<EOF
#!/sbin/openrc-run

description="SCP Bootstrapping script"
name="SCP Bootstrap"

command="apk add openssh"
command_background=true
pidfile="/run/\${RC_SVCNAME}.pid"
EOF

makefile root:root 0755 "$tmp"/etc/runlevels/default/scp_bootstrap <<EOF
../../init.d/scp_bootstrap
EOF

tar -c -C "$tmp" etc | gzip -9n > /tmp/alpine/build/openssh.apkovl.tar.gz

# Rearchive it all
tar -C /tmp/alpine/build -cpf /tmp/alpine/new_alpine.tar --preserve-permissions --atime-preserve --same-owner .
gzip -9 /tmp/alpine/new_alpine.tar

#!/bin/bash
set -e
set -o pipefail

# Actual script begins here

if [ "$(whoami)" != "root" ]; then
    echo "You must be root to do this"
    exit
fi

if [ -z "$1" ]; then 
  echo "[ERR] No device specified for flashing! Please provide a device, e.g. /dev/sda"
  exit
fi

if [ ! -e "/tmp/alpine/new_alpine.tar.gz" ]; then
  echo "[ERR]: Artifacts not found! Please run make_os.sh before this script!"
  exit
fi

# Actual script begins here

echo "[INFO] Repartitioning Device: $1"
fdisk -w always $1 <<EOF
  o
  n
  p
  1
  2048
  -0
  t
  0c
  a
  w
EOF

partition="${1}1"

echo "[INFO] Making Fat32 Filesystem on $partition"
mkdosfs -F32 $partition

echo "[INFO] Mounting $partition to /media/alpine"
mkdir -p /media/alpine
mount -t vfat $partition /media/alpine

echo "[INFO] Copying files to $partition"
sudo tar -p -s --atime-preserve --no-same-owner --one-top-level=/media/alpine -zxvf /tmp/alpine/new_alpine.tar.gz

echo "[INFO] Unmounting $partition"
umount $partition

echo "[DONE] Successfully flashed OS!"

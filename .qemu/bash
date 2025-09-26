qemu-system-x86_64 -cdrom MiniOS.iso -m 2048
cd ~/minios
mkdir -p iso/live
sudo mksquashfs chroot iso/live/filesystem.squashfs -e boot
cp -r chroot/boot iso/boot
grub-mkrescue -o MiniOS.iso iso

useradd -m dev -s /bin/bash
echo "dev:dev" | chpasswd
adduser dev sudo

#/bin/bash
rm -rf disk.img
dd if=/dev/zero of=disk.img bs=3M count=24
mkfs.ext4  disk.img
# mkfs.vfat -F 32 disk.img
mkdir -p mnt
sudo mount disk.img mnt
# 根据命令行参数生成对应的测例
#sudo cp -r ./testcases/junior/* ./mnt/
#sudo cp -r ./testcases/libc-dynamic/* ./mnt/
sudo cp -r ./testcases/$1/* ./mnt/
# sudo cp -r ./testcases/ostrain/* ./mnt/
# sudo cp -r ./testcases/redis/* ./mnt/
# sudo cp -r ./testcases/sdcard/* ./mnt/
sudo umount mnt
rm -rf mnt
sudo chmod 777 disk.img

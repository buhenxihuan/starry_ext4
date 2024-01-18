test_file_name="testsuits-x86_64-linux-musl"

rm disk.img
dd if=/dev/zero of=disk.img bs=3M count=24
mkfs.vfat -F 32 disk.img
mkdir -p tmp_mnt
sudo mount disk.img tmp_mnt
wget "https://github.com/oscomp/testsuits-for-oskernel/releases/download/final-x86_64/$test_file_name.tgz"
tar zxvf "$test_file_name.tgz" 
sudo cp -r "$test_file_name"/* ./tmp_mnt/
sudo umount tmp_mnt
rm -rf tmp_mnt "$test_file_name.tgz" "$test_file_name"/
sudo chmod 777 disk.img
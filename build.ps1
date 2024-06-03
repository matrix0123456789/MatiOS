nasm boot/bootsector.asm -o tmp/bootsector.bin
nasm boot/bootloader.asm -o tmp/bootloader.bin

cd kernel
cargo build
cargo objcopy -- -O binary target/kernel.bin
cargo objcopy --bin matios-kernel -- -O elf64-x86-64 --only-keep-debug target/kernel.debug
cd ..

$img = New-Object System.IO.FileStream("floppy.img", [System.IO.FileMode]::Create, [System.IO.FileAccess]::Write)
$img.SetLength(1474560)

$bootsector = New-Object System.IO.FileStream("tmp/bootsector.bin", [System.IO.FileMode]::Open, [System.IO.FileAccess]::Read)
$img.Seek(0, [System.IO.SeekOrigin]::Begin)
$bootsector.CopyTo($img)
$bootsector.Close()

$bootloader = New-Object System.IO.FileStream("tmp/bootloader.bin", [System.IO.FileMode]::Open, [System.IO.FileAccess]::Read)
$img.Seek(512, [System.IO.SeekOrigin]::Begin)
$bootloader.CopyTo($img)
$bootloader.Close()

$kernel = New-Object System.IO.FileStream("kernel/target/kernel.bin", [System.IO.FileMode]::Open, [System.IO.FileAccess]::Read)
$img.Seek(0x1000, [System.IO.SeekOrigin]::Begin)
$kernel.CopyTo($img)
$kernel.Close()

$img.Close()
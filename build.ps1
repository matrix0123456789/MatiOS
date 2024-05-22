nasm boot/bootsector.asm -o tmp/bootsector.bin
nasm boot/bootloader.asm -o tmp/bootloader.bin

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

$img.Close()
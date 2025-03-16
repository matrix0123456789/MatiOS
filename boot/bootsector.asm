org 0x7c00
[BITS 16]
start:
        mov ax, 0xb800
        mov es, ax
        mov ax, 0
        mov ds, ax
        mov [drive], dl
;read disk structure
        mov ah, 8
        mov dl, [drive]
        int 0x13
        add dh, 1
        mov [numberOfHeads], dh
        and cl, 0x3f
        mov [numberOfSectors], cl




;load next sectors

load:
        mov ah, 0x02
        mov al, 1
        mov ch, [currentCylinder]
        mov cl, [currentSector]
        mov dh, [currentHead]
        mov dl, [drive]
        mov bx, [currentDestination]
        mov es, bx
        mov bx, 0
        int 0x13

        mov ax, [currentDestination]
        add ax, 0x20
        mov [currentDestination], ax
        mov al, [currentSector]
        add al, 1
        mov [currentSector], al
        cmp al, byte [numberOfSectors]
        jbe afterLoad
        mov byte [currentSector], 1
        mov al, [currentHead]
        add al, 1
        mov [currentHead], al

        cmp al, byte [numberOfHeads]
        jb afterLoad
        mov byte [currentHead], 0
        mov al, [currentCylinder]
        add al, 1
        mov [currentCylinder], al

afterLoad:
        mov ax, [currentDestination]
        cmp ax, 0x6000
        jb load

        jmp 0x0000:0x8200


         db 0x11
         db 0x22
         db 0x33
         db 0x44


        drive: db 0x0
        numberOfHeads: db 0
        numberOfSectors: db 0
        currentCylinder: db 0
        currentHead: db 0
        currentSector: db 1
        currentDestination: dw 0x0800


        times 510 - ($ - start) db 0
        dw 0xaa55
[BITS 16]
start:
        mov ax, 0xb800
        mov es, ax
        mov byte [es:0x0], 'H'
        mov byte [es:0x2], 'e'
        mov byte [es:0x4], 'l'
        mov byte [es:0x6], 'l'
        mov byte [es:0x8], 'o'


;load next sectors

        mov ah, 0x02
        mov al, 18
        mov ch, 0 ;cylinder
        mov cl, 1 ;sector
        mov dh, 0 ;head
        mov bx, 0x0800
        mov es, bx
        mov bx, 0
        int 0x13

        mov ah, 0x02
        mov al, 18
        mov ch, 0 ;cylinder
        mov cl, 1 ;sector
        mov dh, 1 ;head
        mov bx, 0x0a40
        mov es, bx
        mov bx, 0
        int 0x13

        mov ah, 0x02
        mov al, 18
        mov ch, 1 ;cylinder
        mov cl, 1 ;sector
        mov dh, 0 ;head
        mov bx, 0x0c80
        mov es, bx
        mov bx, 0
        int 0x13

        mov ah, 0x02
        mov al, 10
        mov ch, 1 ;cylinder
        mov cl, 1 ;sector
        mov dh, 1 ;head
        mov bx, 0x0ec0
        mov es, bx
        mov bx, 0
        int 0x13

        mov ah, 0x02
        mov al, 8
        mov ch, 1 ;cylinder
        mov cl, 11 ;sector
        mov dh, 1 ;head
        mov bx, 0x1000
        mov es, bx
        mov bx, 0
        int 0x13

        mov ah, 0x02
        mov al, 8
        mov ch, 2 ;cylinder
        mov cl, 1 ;sector
        mov dh, 0 ;head
        mov bx, 0x1100
        mov es, bx
        mov bx, 0
        int 0x13

        jmp 0x0000:0x8200

        times 510 - ($ - start) db 0
        dw 0xaa55
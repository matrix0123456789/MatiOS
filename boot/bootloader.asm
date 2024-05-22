[BITS 16]
start:
        mov ax, 0xb800
        mov es, ax
        mov byte [es:0xa], ' '
        mov byte [es:0xc], 'W'
        mov byte [es:0xe], 'o'
        mov byte [es:0x10], 'r'
        mov byte [es:0x12], 'l'
        mov byte [es:0x14], 'd'
        mov byte [es:0x16], '!'

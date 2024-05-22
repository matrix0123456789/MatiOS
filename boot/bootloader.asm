org 0x8200
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
;launching protected mode
        cli
        mov ax, 0x08
        mov es, ax


        mov eax, cr0
        or eax, 1
        mov cr0, eax
        lgdt [gdt_descriptor]

        mov ax, 0x08
        mov ds, ax
        mov es, ax
        mov fs, ax
        mov gs, ax
        mov ss, ax


        nop
        nop
        nop
        jmp dword 0x10:bits16


[BITS 16]
bits16:
        mov ax, 0xb800
        mov es, ax
        mov byte [es:0x0], '1'
        mov byte [es:0x2], '6'
        mov byte [es:0x4], 'b'
        mov byte [es:0x6], 'i'
        mov byte [es:0x8], 't'
        mov byte [es:0xa], 's'

loop1: jmp loop1
[BITS 32]
bits32:
        mov byte [0xb8000], '3'
        mov byte [0xb8002], '2'
        mov byte [0xb8004], 'b'
        mov byte [0xb8006], 'i'
        mov byte [0xb8008], 't'
        mov byte [0xb800a], 's'

loop2: jmp loop2
;gdt
gdt_start:
gdt_null:
        dd 0
        dd 0
        dd 0
        dd 0
gdt_data:
        dw 0xffff     ; Limit (bits  0-15)
        dw 0x0         ; Base (bits  0-15)
        db 0x0        ; Base (bits  16 -23)
        db 10010010b  ; 1st flags, type flags (data segment)
        db 11001111b  ; 2nd flags, Limit (bits 16-19)
        db 0x0         ; Base (bits  24 -31)

gdt_code16:
dw 0xffff
dw 0x0
db 0x0
db  10011010b
db  10001111b
db 0x0
gdt_code32:

        dw 0xffff     ; Limit (bits  0-15)
        dw 0x0         ; Base (bits  0-15)
        db 0x0         ; Base (bits  16 -23)
        db  10011010b ; 1st flags , type  flags
        db  11001111b ; 2nd flags , Limit (bits  16-19)
        db 0x0         ; Base (bits  24 -31)

gdt_code64:
        dw 0xffff     ; Limit (bits  0-15)
        dw 0x0         ; Base (bits  0-15)
        db 0x0         ; Base (bits  16 -23)
        db  10011010b ; 1st flags , type  flags
        db  10101111b ; 2nd flags , Limit (bits  16-19)
        db 0x0         ; Base (bits  24 -31)
     gdt_end:

gdt_descriptor:
dw  gdt_end  - gdt_start  - 1
dd  gdt_start
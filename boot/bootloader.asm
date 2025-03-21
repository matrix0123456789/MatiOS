org 0x8200
[BITS 16]
start:
        mov ax, 2401h
        int 15h

;read memory map
        mov ax, 0
        mov es, ax
        mov di, 0x6000
        mov ebx, 0
        mov edx, 0x534d4150

        readMemoryLoop:
        mov eax, 0xe820
        mov ecx, 24
        int 0x15

        add di, 24
        cmp ebx, 0
        jne readMemoryLoop



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
        jmp dword 0x18:bits32



loop1: jmp loop1
[BITS 32]
bits32:
        mov byte [0xb8000+160], '3'
        mov byte [0xb8002+160], '2'
        mov byte [0xb8004+160], 'b'
        mov byte [0xb8006+160], 'i'
        mov byte [0xb8008+160], 't'
        mov byte [0xb800a+160], 's'

;creating basic pagination
        mov dword [0x1000], 0x2001
        mov dword [0x2000], 0x3001
        mov dword [0x2008], 0x4001
        mov dword [0x2010], 0x5001

        mov eax, 0x0081
        mov ebx, 0x3000
paginationLoop:
        mov [ebx], eax
        add eax, 0x200000
        add ebx, 8
        cmp ebx, 0x6000
        jne paginationLoop

        mov eax, 0x1001
        mov cr3, eax

        mov eax, cr4
        or eax, 0x00000020
        mov cr4, eax
        mov ecx, 0xC0000080
        rdmsr
        or eax, 0x100
        wrmsr
        mov ebx, 0x80000001
        mov cr0, ebx

        mov esp, 0x90000
        jmp 0x20:bits64

[bits 64]
bits64:
        mov byte [0xb8000+320], '6'
        mov byte [0xb8002+320], '4'
        mov byte [0xb8004+320], 'b'
        mov byte [0xb8006+320], 'i'
        mov byte [0xb8008+320], 't'
        mov byte [0xb800a+320], 's'
        mov byte [0xb800c+320], ' '
        mov byte [0xb800e+320], 'm'
        mov byte [0xb8010+320], 'o'
        mov byte [0xb8012+320], 'd'
        mov byte [0xb8014+320], 'e'
        mov byte [0xb8016+320], '!'

        call 0x9000
loop2: jmp loop2
;gdt
gdt_start:
gdt_null:
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
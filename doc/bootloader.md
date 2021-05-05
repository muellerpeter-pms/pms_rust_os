# Warum wir einen Bootloader verwenden


## Multiboot und x86_64

Ich habe einige Versuche unternommen, die kernel per multiboot in qemu zu starten. Dabei bin ich während der Recherche auf folgende wichtige Punkte gestoßen:

- qemu könnte ab Version > 4.0 auch x86_64 elf Dateien per multiboot starten. (Leider habe ich es bis dahin nicht lauffähig bekommen)
- der multiboot- oder multiboot2-Standard sieht nur Kernels in 32bit vor. Auch wenn qemu uns also starten könnte, so dürfen wir das von grub in einer echten Umgebung nicht erwarten.

## Was ein Bootloader könne sollte

Der Prozess vom start unseres Code bis zur kernel im long-mode ist ohen Frage sehr interessant. Leider könnten wir einen solchen Bootloader nicht im aktuellen Projekt oder Workspace umsetzen, da wir ganz andere cargo-Konfigurationen benötigen würden. Auch unser linkziel zu x86_64 bereitet Probleme. Das BIOS übergibt uns die Kontrolle im Real-Mode (16 bit), von wo wir uns in den Protected Mode (32 bit) arbeiten dürften , um die Initialisierung für den Long-Mode (64 bit) vorzunehmen. Während all dieser Initialisierungsschritte haben wir andere Instruktionssätze für unseren Assembler. 
Neben den angesprochenen Problemen gibt es noch diverser andere, welche ein Bootloader lösen sollte. Ich denke damit erklärt sich auch recht gut, warum wir hier einen vordefinierten bootloader verwenden, welcher unsere kernel bereits im Long-mode aufruft.
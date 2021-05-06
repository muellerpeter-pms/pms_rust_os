# Roadmap

## Was unsere Kernel schon kann

- boot durch einen Bootloader und schreiben eines ersten Textes auf den Bildschirm
- Abbildung von print! und println!, so das wir eine komplette Ausgabe durch unsere Kernel haben können
    - println! funktioniert nun im gesamten crate
    - panic erzeugt nun debug-fähige Ausgaben
    
## Was sie als nächstes können soll.

- Ermöglichen von Tests für die Kernel
- Excepetions
    - CPU Ausnahmen
    - interrupts
        - PIC
        - APIC
    - Speichermanagement
        - physischer Speicher
        - Paging
        
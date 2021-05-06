# Tests

## Tests in der #[no_std] Umgebung

Rust benötigt für die typischen tests die std-Bibliothek. Leider haben wir diese nicht zur Verfügung. 
In Anlehnung an den Blog von [Phil Oppermann](https://os.phil-opp.com/testing/) ist es einerseits recht
kompliziert, das test-crate ohne die std-Bibliothek zu übersetzen, und birgt andererseits auch 
Unstabilitäten, wie beispielsweise die Neudefinition von panic! (Diese Aussagen habe ich ohne weitere Prüfung so hingenommen).

Statt dessen bietet sich die Möglichkeit durch 
[feature(custom_test_frameworks)](https://doc.rust-lang.org/nightly/unstable-book/language-features/custom-test-frameworks.html)
eine eigene Testumgebung herzustellen. Statt #[test] müssen wir dabei #[test-case] verwenden. 

## Tests durch Qemu ausführen

### Qemu nach erfolgreichem Test beeden

Für ein komplettes herunter fahren des Systems müssten wir Kontrolle über das APCI System haben, wovon wir noch sehr weit entfernt sind. 
Glücklicherweise wurde in qemu eine andere 
[Variante für das Beenden](https://qemu-devel.nongnu.narkive.com/UuAORLrS/patch-2-4-add-isa-debug-exit-device) eingebaut.
Sobald auf dieses Device geschrieben wird, beendet sich qemu. Der geschriebene Wert wird dabei nach einem Shift-Left als Rückgabe von Qemu verwendet. (Vgl. static void debug_exit_write) Dies nutzen wir für unsere Tests.








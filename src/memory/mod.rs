//! Handhabung des physischen und virtuellen Speichers.
//!
//! Das Modul enthält die Verwaltung des physischen, als auch des virtuellen Speichers.
//! Zusätzlich ist ein Allocator für den Kernel-eigenen Heap implementiert. Todo!()
//!
//! Die Initialisierung zeigt sich komliziert, da der physische Speichermanager sehr viel
//! Effizienter agieren kann, wenn er die freien Seiten als (!dynamische) Listen verwalten
//! kann. Dazu müsste aber der globale Allocator bereit sein, welcher wiederum den
//! physischen Speichermanager benötigt.  
//!
//! Dire Initialisierung findet deshalb in einzelnen Schritten statt.
//! Zuerst fordern wir vom physischen Speicher eine einzelne Seite an und initialisieren damit
//! den virtuellen Speichermanager inklusive globalem Allokator.
//! Danach kann der physische Speichermanager seine Strukturen erzeugen, diese dürfen 4kb (noch) nicht überschreiten.
//! nach Abschluss des Vorgangs kann der Heap-Allocator auch neue Seiten vom physischen Manager anfordern und zurück geben,
//! so dass auch größere Bereiche angefordert werden können.

use bootloader::bootinfo::MemoryMap;

mod physical;

pub use physical::PhysicalMemoryManager;

//use physical::PhysicalMemoryManager;

/// Initialisiert die Speicherverwaltung
pub fn init(_virt_memory_offset: u64, _map: &'static MemoryMap) {}

/// Allgemeine Form für Mutex<T> aber im Crate erweiterbar.
pub struct Locked<T> {
    /// innere Verarbeitung durch einen Spin-Lock
    inner: spin::Mutex<T>,
}

impl<T> Locked<T> {
    /// Erzeugt ein neues Locked Objekt.
    pub const fn new(inner: T) -> Self {
        Locked {
            inner: spin::Mutex::new(inner),
        }
    }

    /// Sperrt das Locked und gibt mit dem MutexGuard das innere als &mut frei.
    pub fn lock(&self) -> spin::MutexGuard<T> {
        self.inner.lock()
    }
}

# Demain j'arrête le café, je me met au crabe !

Présentation du langage Rust et de son écosystème.

Le but étant de faciliter le démarrage sur cette technologie pour des personnes utilisant des langages haut niveau comme Java. Dans cette présentation nous découvrirons:

* Le Langage (3 piliers)
  * Productivité : API fonctionnelle, pas de gestion manuelle de la mémoire, typage fort, inférence de type
  * Performance : Compilation native avec LLVM, ∅GC, langage bas niveau
  * Sûreté : sûreté de la mémoire, pas d’accès concurrent
  * Features présentées (et animées !) :
    * struct / trait / generics
    * ownership / borrowing / lifetime
    * memory guarantees : Box / Rc / Cell / Arc
    * macro
* L'Écosystème :
  * IDE, tests, debugger, RLS (?)
  * Build et packages manager (Cargo)
  * Librairies phares (Iron, Mio, Nom, Piston)

Puis nous terminerons sur une démo multi plateforme avec un backend tournant sur les différentes plateformes: Windows / Linux / ARM, puis un frontend en webassembly avec rust.

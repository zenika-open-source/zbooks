# Zbooks

## Choix projet

- Framework
  - [Leptos](https://leptos.dev/)
 
## Besoins

- Liste les livres
- (Liste d'eBook)
- Liste d'envie
  - Avec un système de vote pour prioriser un livre à acheter
  - Qui a proposé?
  - État de l'envie (En attente, validé, commandé, dispo, ...)
- Rechercher des livres
- Réserver des livres
  - Savoir qui réserve
  - Relance pour savoir si la personne utilise et a toujours le livre
  - Liste d'attente pour le livre réservé
- Prêt de livres (pas seulement les livres Zenika)
  - Personne à personne
  - Agence Zenika à personne
- Vu par agence
  - Gestion de prêt inter-agence

## Comment utiliser ?

Installer Rust

Installer trunk

```
cargo install trunk
```

Ajouter la target wasm
```
rustup target add wasm32-unknown-unknown
```

Lancer le projet
```
trunk serve --open
```

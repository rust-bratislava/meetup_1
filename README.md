High To Low workshop #1
=========


Téma workshopu: Syntax a Typy (String)
---------

Z high level jazykov (ako napríklad PHP alebo Python) sme zvyknutí na dynamický typ string, ktorý sa dá okamžite a ľahko manipilovať. Keďže jazyku Rust ide o performance, musíme zaobchádzať zo stringom na nižšej úrovni. Čo to znamená?

Rust má dva typy, ktoré môžu obsahovať stringové dáta. Prvým je `&str` a `String`. V rôynzch ukážkach kódu Rustu sa môžeme stretnúť s týmto zápisom:

```rust
let nazov = "Alice in wonderland.";
```
Za normálnych okolností by sme museli pri deklarácii premennej zadeklarovať aj typ. Napríklad:

```rust
let x: i32 = 5; // Deklarujeme premennu a zaroven jej typ i32
```

Rust ale automaticky uhádne typ premennej za nás, preto nemusíme typ premennej deklarovať. Už spomenutý zápis nám vytvorí premennú typu `&str`. Oba zápisy ponižšie nám preto vygenerujú premennú rovnakého typu.

```rust
let nazov = "Alice in wonderland."; 
let nazov:&str = "Alice in wonderland."; 

```

Rozdiel medzi `&str` a `String`
----------

&str sa štandardne ukladá na stack, kým String sa ukladá na heap.

# Ownership y Funciones en Rust

Este documento explica los conceptos fundamentales de **ownership** (propiedad) en Rust: cómo se mueven o copian los valores cuando se pasan a funciones.

## 🔑 Conceptos Clave

### **Move** vs **Copy**
- **Move**: Transfiere la propiedad (tipos complejos como `String`)
- **Copy**: Duplica el valor (tipos simples como `i32`, `bool`, `char`)

---

## 📦 Parte 1: Propiedad y Funciones

### Ejemplo con `String` (Move)

```rust
let s = String::from("hello");  // s es el dueño de "hello"

takes_ownership(s);  // s se MUEVE a la función
                     // s ya NO es válido aquí

// println!("{}", s);  // ❌ ERROR: s fue movido
```

**¿Qué pasa?**
1. `s` se crea y es dueño de `"hello"` en el heap
2. Al llamar `takes_ownership(s)`, la propiedad se **transfiere** a la función
3. `s` ya no existe en el scope original
4. Al terminar la función, `some_string` se destruye y libera la memoria

```rust
fn takes_ownership(some_string: String) {
    println!("{}", some_string);  // Usa el String
} // ← Aquí se llama `drop()` y se libera la memoria
```

### Ejemplo con `i32` (Copy)

```rust
let x = 5;  // x tiene el valor 5

makes_copy(x);  // x se COPIA a la función
                // x SIGUE siendo válido aquí

println!("{}", x);  // ✅ Funciona: x = 5
```

**¿Qué pasa?**
1. `x` contiene `5` (almacenado en el stack)
2. Al llamar `makes_copy(x)`, se **copia** el valor
3. `x` sigue existiendo y es válido
4. Hay dos copias independientes del valor `5`

```rust
fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);  // Usa la copia
} // ← some_integer desaparece, pero no afecta a x
```

---

## 🔄 Parte 2: Valores de Retorno y Alcance

### 1️⃣ `gives_ownership()` - Devuelve propiedad

```rust
let s1 = gives_ownership();  // s1 recibe la propiedad de "yours"
```

```rust
fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string  // Devuelve y MUEVE la propiedad al llamador
}
```

**Flujo**:
- Se crea `"yours"` dentro de la función
- Se **devuelve** y la propiedad se transfiere a `s1`
- `s1` ahora es el dueño

### 2️⃣ `takes_and_gives_back()` - Recibe y devuelve

```rust
let s2 = String::from("hello");
let s3 = takes_and_gives_back(s2);  // s2 se mueve, s3 recibe la propiedad

// println!("{}", s2);  // ❌ ERROR: s2 fue movido
println!("{}", s3);     // ✅ Funciona
```

```rust
fn takes_and_gives_back(a_string: String) -> String {
    a_string  // Devuelve el mismo String
}
```

**Flujo**:
1. `s2` se **mueve** a la función (ya no es válido)
2. La función devuelve `a_string`
3. `s3` recibe la propiedad
4. Ahora `s3` es el dueño de `"hello"`

---

## 🗑️ ¿Qué Pasa al Final del Scope?

```rust
} // Final del scope principal

// x: Se destruye (pero es Copy, no importa)
// s: Ya fue movido, no pasa nada
// s1: Se destruye y libera "yours"
// s2: Ya fue movido, no pasa nada
// s3: Se destruye y libera "hello"
```

---

## 📊 Diagrama de Flujo Completo

```
INICIO
│
├─ s = "hello" (heap)
│  └─ takes_ownership(s) → s MOVIDO → se libera en la función
│
├─ x = 5 (stack)
│  └─ makes_copy(x) → x COPIADO → x sigue válido
│
├─ s1 = gives_ownership()
│  └─ Recibe "yours" del retorno
│
├─ s2 = "hello" (heap)
│  └─ s3 = takes_and_gives_back(s2)
│     ├─ s2 MOVIDO (ya no válido)
│     └─ s3 recibe la propiedad
│
FIN → se liberan s1 y s3
```

---

## 🎯 Reglas de Oro

| Tipo | Trait | Comportamiento | Después de pasar a función |
|------|-------|----------------|----------------------------|
| `String`, `Vec`, structs | **No Copy** | **Move** | ❌ Variable inválida |
| `i32`, `bool`, `char`, `f64` | **Copy** | **Copy** | ✅ Variable válida |

### ¿Cómo evitar mover valores?

**Opción 1: Usar referencias (borrowing)**
```rust
fn takes_reference(s: &String) {
    println!("{}", s);
}

let s = String::from("hello");
takes_reference(&s);  // Presta, no mueve
println!("{}", s);    // ✅ s sigue válido
```

**Opción 2: Clonar**
```rust
let s = String::from("hello");
takes_ownership(s.clone());  // Clona el valor
println!("{}", s);           // ✅ s sigue válido
```

**Opción 3: Devolver el valor**
```rust
fn process(s: String) -> String {
    println!("{}", s);
    s  // Devuelve la propiedad
}

let s = String::from("hello");
let s = process(s);  // Recupera la propiedad
```

---

## 💡 Resumen

- **Move**: Transfiere propiedad, la variable original se invalida
- **Copy**: Duplica el valor, ambas variables son válidas
- **Return**: Puede devolver la propiedad al llamador
- **Borrowing** (`&`): La mejor forma de usar valores sin moverlos

¡Este sistema previene errores de memoria en tiempo de compilación! 🦀

---

## 📝 Código Completo de Ejemplo

```rust
fn main() {
    /* Propiedad y funciones */
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // Because i32 implements the Copy trait,
                                    // x does NOT move into the function,
                                    // so it's okay to use x afterward.

    /* Valores de retorno y alcance */
    let s1 = gives_ownership();        // gives_ownership moves its return
                                       // value into s1

    let s2 = String::from("hello");    // s2 comes into scope

    let s3 = takes_and_gives_back(s2); // s2 is moved into
                                       // takes_and_gives_back, which also
                                       // moves its return value into s3
    println!("s1 = {}, s3 = {}", s1, s3);

} // Here, x goes out of scope, then s. However, because s's value was moved,
  // nothing special happens. || Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {       // gives_ownership will move its
                                       // return value into the function
                                       // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                        // some_string is returned and
                                       // moves out to the calling
                                       // function
}

// This function takes a String and returns a String.
fn takes_and_gives_back(a_string: String) -> String {
                                       // a_string comes into
                                       // scope

    a_string  // a_string is returned and moves out to the calling function
}
```

---

## 🔗 Recursos Adicionales

- [The Rust Book - Ownership](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)
- [Rust by Example - Ownership](https://doc.rust-lang.org/rust-by-example/scope/move.html)
- [Understanding Ownership in Rust](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html)
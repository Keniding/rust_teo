fn main() {
    {                      // s is not valid here, since it's not yet declared
        let s = "hello";   // s is valid from this point forward
        println!("{}", s);
        // do stuff with s
    }                      // this scope is now over, and s is no longer valid

    // StringType. Este tipo gestiona los datos asignados en el montón y, por lo tanto, puede almacenar una cantidad de texto desconocida en tiempo de compilación.
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a string
    println!("{}", s); // this will print `hello, world`

    // Move
    let x = 5;
    let y = x;
    print!("x = {}, y = {}", x, y);

    /*
    String se compone de tres partes, que se muestran a la izquierda: un puntero a la memoria que contiene el contenido de la cadena, una longitud y una capacidad. Este grupo de datos se almacena en la pila. A la derecha se encuentra la memoria en el montón que contiene el contenido.
    Al asignar s1a s2, se copian los Stringdatos, es decir, se copian el puntero, la longitud y la capacidad de la pila. No se copian los datos del montón al que hace referencia el puntero. En otras palabras, la representación de los datos en memoria se muestra en la Figura 4-2.
     */
    let s1 = String::from("hello");
    let s2 = s1;
    print!("s2 is {}", s2); // s1 no se puede usar porque se movió y ya no se salió de su ámbito, quedando invalido para evitar doble liberación
    // Anteriormente, mencionamos que cuando una variable queda fuera de alcance, Rust llama automáticamente a la dropfunción y limpia la memoria del montón de esa variable. Sin embargo, la Figura 4-2 muestra ambos punteros de datos apuntando a la misma ubicación. Esto representa un problema: cuando s2y s1quedan fuera de alcance, ambos intentarán liberar la misma memoria. Esto se conoce como error de doble liberación y es uno de los errores de seguridad de memoria que mencionamos anteriormente. Liberar memoria dos veces puede provocar corrupción de memoria, lo que potencialmente puede generar vulnerabilidades de seguridad.
    // Para garantizar la seguridad de la memoria, después de la línea let s2 = s1; Rust s1la considera inválida. Por lo tanto, no necesita liberar nada al s1salir del ámbito. Observa lo que ocurre al intentar usar ` s1after` s2; no funcionará.
    // Si has escuchado los términos "copia superficial" y "copia profunda" al trabajar con otros lenguajes, el concepto de copiar el puntero, la longitud y la capacidad sin copiar los datos probablemente te suene a una copia superficial. Sin embargo, como Rust también invalida la primera variable, en lugar de llamarse copia superficial, se conoce como movimiento. En este ejemplo, diríamos que s1 se movió a s2.

    // Alcance y asignación
    let mut s = String::from("hello");
    println!("s1 is {}", s);
    s = String::from("ahoy");
    println!("{s}, world!");
    // Inicialmente, declaramos una variable sy la vinculamos a a Stringcon el valor "hello". Inmediatamente después, creamos una nueva variable Stringcon el valor "ahoy" y la asignamos a s. En este punto, nada hace referencia al valor original en el montón. La Figura 4-5 ilustra los datos de la pila y el montón:

    // Clone
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);
    // La razón es que tipos como los enteros, que tienen un tamaño conocido en tiempo de compilación, se almacenan completamente en la pila, por lo que las copias de los valores reales se crean rápidamente. Esto significa que no hay razón para que queramos impedir xque sean válidos después de crear la variable y. En otras palabras, no hay diferencia entre la copia profunda y la superficial, por lo que la llamada cloneno tendría ningún efecto diferente a la copia superficial habitual, y podemos omitirla.

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

    /* Referencias */
    // Sin referencias
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("s2 = {}, len = {}", s2, len);

    // Con referencia
    let s1 = String::from("hello");
    let len = calculate_length_ref(&s1);
    println!("s1 = {}, len = {}", s1, len);

    // Modificar algo prestado no permitido
    // let s = String::from("hello");
    // change(&s);
    // Así como las variables son inmutables por defecto, también lo son las referencias. No podemos modificar algo a lo que tenemos una referencia.

    /* Referencias mutables */
    let mut s = String::from("hello");
    change(&mut s);

    // Las referencias mutables tienen una restricción importante: si se tiene una referencia mutable a un valor, no se pueden tener otras referencias a ese valor. Este código, que intenta crear dos referencias mutables, sfallará:
    let mut s = String::from("hello");
    let r1 = &mut s;
    // let r2 = &mut s; // cannot borrow `s` as mutable more than once at a time [E0499] second mutable borrow occurs here
    // println!("r1 = {}, r2 = {}", r1, r2);
    println!("r1 = {}", r1);

    // Usar llaves para crear un nuevo ámbito para cerrar una referencia y continuar con la siguiente
    let mut s = String::from("hello");
    {
        let r1 = &mut s;
        println!("r1 = {}", r1);
    } // r1 goes out of scope here, so we can make a new reference with no problems.
    let r2 = &mut s;
    println!("r2 = {}", r2);

    // Referencias mutables e inmutables
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("r1 = {}, r2 = {}", r1, r2);
    let r3 = &mut s; // cannot borrow `s` as mutable because it is also borrowed as immutable [E0502] mutable borrow occurs here
    // println!("{r1}, {r2} and {r3}"); // Por esto, porque hacemos referencia a r1, r2
    // Solución definir print previo, y luego
    println!("r3 = {}", r3);
    // ¡Los alcances de las referencias inmutables r1 terminan r2 después de println! Su último uso, es decir, antes de la r3creación de la referencia mutable. Estos alcances no se superponen, por lo que se permite este código: el compilador puede determinar que la referencia ya no se utiliza en un punto anterior al final del alcance.

    // Referencias colgadas
    // En lenguajes con punteros, es fácil crear erróneamente un puntero colgante (un puntero que referencia a una ubicación en memoria que podría haber sido asignada a otra persona) al liberar memoria y conservar un puntero a dicha memoria. En Rust, por el contrario, el compilador garantiza que las referencias nunca serán colgantes: si se tiene una referencia a datos, el compilador se asegurará de que estos no salgan del ámbito antes que la referencia a ellos.
    let reference_to_nothing = no_dangle();
    println!("reference_to_nothing = {}", reference_to_nothing);

} // Here, x goes out of scope, then s. However, because s's value was moved,
// nothing special happens. || Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
// happens. s1 goes out of scope and is dropped.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
// memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{some_integer}");
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

fn calculate_length(some_string: String) -> (String, usize) {
    let length = some_string.len();
    (some_string, length)
}

/* Primero, observe que el código de tupla en la declaración de la variable y el valor de retorno de la función ha desaparecido. Segundo, observe que pasamos &s1a calculate_lengthy, en su definición, tomamos &Stringen lugar de String. Estos símbolos & representan referencias y permiten referirse a un valor sin tomar posesión de él. La Figura 4-6 ilustra este concepto. */
fn calculate_length_ref(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because s does not have ownership of what
// it refers to, the String is not dropped.

// Llamamos préstamo a la acción de crear una referencia. Como en la vida real, si alguien posee algo, puedes pedírselo prestado. Al terminar, debes devolverlo. No eres el dueño.
// Entonces, ¿qué pasa si intentamos modificar algo que tomamos prestado? Prueben el código del Listado 4-6. ¡Atención!: ¡No funciona!
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
// Primero, cambiamos sa mut. Luego, creamos una referencia mutable con &mut sdonde llamamos a la changefunción y actualizamos la firma de la función para que acepte una referencia mutable con some_string: &mut String. Esto deja muy claro que la changefunción mutará el valor que toma prestado.

// fn dangle() -> &String { // dangle returns a reference to a String
//    let s = String::from("hello"); // s is a new String
//    &s // we return a reference to the String, s
// } // Here, s goes out of scope and is dropped, so its memory goes away.
// Danger!
fn no_dangle() -> String {
    let s = String::from("hello");
    s
}
// En cualquier momento, puede tener una referencia mutable o cualquier cantidad de referencias inmutables.
// Las referencias siempre deben ser válidas.

What Is Ownership?
Ownership is a set of rules that govern how a Rust program manages memory. All programs have to manage the way they use a computer’s memory while running. Some languages have garbage collection that regularly looks for no-longer-used memory as the program runs; in other languages, the programmer must explicitly allocate and free the memory. Rust uses a third approach: Memory is managed through a system of ownership with a set of rules that the compiler checks. If any of the rules are violated, the program won’t compile. None of the features of ownership will slow down your program while it’s running.

Because ownership is a new concept for many programmers, it does take some time to get used to. The good news is that the more experienced you become with Rust and the rules of the ownership system, the easier you’ll find it to naturally develop code that is safe and efficient. Keep at it!

When you understand ownership, you’ll have a solid foundation for understanding the features that make Rust unique. In this chapter, you’ll learn ownership by working through some examples that focus on a very common data structure: strings.

The Stack and the Heap
Many programming languages don’t require you to think about the stack and the heap very often. But in a systems programming language like Rust, whether a value is on the stack or the heap affects how the language behaves and why you have to make certain decisions. Parts of ownership will be described in relation to the stack and the heap later in this chapter, so here is a brief explanation in preparation.

Both the stack and the heap are parts of memory available to your code to use at runtime, but they are structured in different ways. The stack stores values in the order it gets them and removes the values in the opposite order. This is referred to as last in, first out (LIFO). Think of a stack of plates: When you add more plates, you put them on top of the pile, and when you need a plate, you take one off the top. Adding or removing plates from the middle or bottom wouldn’t work as well! Adding data is called pushing onto the stack, and removing data is called popping off the stack. All data stored on the stack must have a known, fixed size. Data with an unknown size at compile time or a size that might change must be stored on the heap instead.

The heap is less organized: When you put data on the heap, you request a certain amount of space. The memory allocator finds an empty spot in the heap that is big enough, marks it as being in use, and returns a pointer, which is the address of that location. This process is called allocating on the heap and is sometimes abbreviated as just allocating (pushing values onto the stack is not considered allocating). Because the pointer to the heap is a known, fixed size, you can store the pointer on the stack, but when you want the actual data, you must follow the pointer. Think of being seated at a restaurant. When you enter, you state the number of people in your group, and the host finds an empty table that fits everyone and leads you there. If someone in your group comes late, they can ask where you’ve been seated to find you.

Pushing to the stack is faster than allocating on the heap because the allocator never has to search for a place to store new data; that location is always at the top of the stack. Comparatively, allocating space on the heap requires more work because the allocator must first find a big enough space to hold the data and then perform bookkeeping to prepare for the next allocation.

Accessing data in the heap is generally slower than accessing data on the stack because you have to follow a pointer to get there. Contemporary processors are faster if they jump around less in memory. Continuing the analogy, consider a server at a restaurant taking orders from many tables. It’s most efficient to get all the orders at one table before moving on to the next table. Taking an order from table A, then an order from table B, then one from A again, and then one from B again would be a much slower process. By the same token, a processor can usually do its job better if it works on data that’s close to other data (as it is on the stack) rather than farther away (as it can be on the heap).

When your code calls a function, the values passed into the function (including, potentially, pointers to data on the heap) and the function’s local variables get pushed onto the stack. When the function is over, those values get popped off the stack.

Keeping track of what parts of code are using what data on the heap, minimizing the amount of duplicate data on the heap, and cleaning up unused data on the heap so that you don’t run out of space are all problems that ownership addresses. Once you understand ownership, you won’t need to think about the stack and the heap very often. But knowing that the main purpose of ownership is to manage heap data can help explain why it works the way it does.

Ownership Rules
First, let’s take a look at the ownership rules. Keep these rules in mind as we work through the examples that illustrate them:

Each value in Rust has an owner.
There can only be one owner at a time.
When the owner goes out of scope, the value will be dropped.
Variable Scope
Now that we’re past basic Rust syntax, we won’t include all the fn main() { code in the examples, so if you’re following along, make sure to put the following examples inside a main function manually. As a result, our examples will be a bit more concise, letting us focus on the actual details rather than boilerplate code.

As a first example of ownership, we’ll look at the scope of some variables. A scope is the range within a program for which an item is valid. Take the following variable:

let s = "hello";
The variable s refers to a string literal, where the value of the string is hardcoded into the text of our program. The variable is valid from the point at which it’s declared until the end of the current scope. Listing 4-1 shows a program with comments annotating where the variable s would be valid.

    {                      // s is not valid here, since it's not yet declared
        let s = "hello";   // s is valid from this point forward

        // do stuff with s
    }                      // this scope is now over, and s is no longer valid
Listing 4-1: A variable and the scope in which it is valid
In other words, there are two important points in time here:

When s comes into scope, it is valid.
It remains valid until it goes out of scope.
At this point, the relationship between scopes and when variables are valid is similar to that in other programming languages. Now we’ll build on top of this understanding by introducing the String type.

The String Type
To illustrate the rules of ownership, we need a data type that is more complex than those we covered in the “Data Types” section of Chapter 3. The types covered previously are of a known size, can be stored on the stack and popped off the stack when their scope is over, and can be quickly and trivially copied to make a new, independent instance if another part of code needs to use the same value in a different scope. But we want to look at data that is stored on the heap and explore how Rust knows when to clean up that data, and the String type is a great example.

We’ll concentrate on the parts of String that relate to ownership. These aspects also apply to other complex data types, whether they are provided by the standard library or created by you. We’ll discuss non-ownership aspects of String in Chapter 8.

We’ve already seen string literals, where a string value is hardcoded into our program. String literals are convenient, but they aren’t suitable for every situation in which we may want to use text. One reason is that they’re immutable. Another is that not every string value can be known when we write our code: For example, what if we want to take user input and store it? It is for these situations that Rust has the String type. This type manages data allocated on the heap and as such is able to store an amount of text that is unknown to us at compile time. You can create a String from a string literal using the from function, like so:

let s = String::from("hello");
::El operador de dos puntos nos permite crear un espacio de nombres para esta from función en particular bajo el Stringtipo, en lugar de usar un nombre como string_from. Analizaremos esta sintaxis con más detalle en la sección "Métodos" del Capítulo 5, y cuando tratemos el espacio de nombres con módulos en "Rutas para hacer referencia a un elemento en el árbol de módulos" del Capítulo 7.

Este tipo de cadena se puede mutar:

    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{s}"); // this will print `hello, world!`
Entonces, ¿cuál es la diferencia? ¿Por qué se pueden Stringmutar, pero los literales no? La diferencia radica en cómo estos dos tipos gestionan la memoria.

Memoria y asignación
En el caso de un literal de cadena, conocemos su contenido en tiempo de compilación, por lo que el texto se codifica directamente en el ejecutable final. Por eso los literales de cadena son rápidos y eficientes. Sin embargo, estas propiedades solo se deben a su inmutabilidad. Desafortunadamente, no podemos asignar un bloque de memoria al binario por cada fragmento de texto cuyo tamaño se desconoce en tiempo de compilación y cuyo tamaño podría cambiar durante la ejecución del programa.

Con el Stringtipo, para admitir un fragmento de texto mutable y ampliable, necesitamos asignar una cantidad de memoria en el montón, desconocida en tiempo de compilación, para almacenar el contenido. Esto significa:

La memoria debe solicitarse al asignador de memoria en tiempo de ejecución.
Necesitamos una forma de devolver esta memoria al asignador cuando hayamos terminado con nuestro String.
Nosotros realizamos la primera parte: cuando llamamos a String::from, su implementación solicita la memoria que necesita. Esto es prácticamente universal en los lenguajes de programación.

Sin embargo, la segunda parte es diferente. En lenguajes con recolector de basura (GC) , este rastrea y limpia la memoria que ya no se usa, sin que tengamos que preocuparnos por ello. En la mayoría de los lenguajes sin GC, es nuestra responsabilidad identificar cuándo la memoria ya no se usa y ejecutar código para liberarla explícitamente, tal como lo hacíamos para solicitarla. Hacer esto correctamente ha sido históricamente un problema de programación complejo. Si lo olvidamos, desperdiciaremos memoria. Si lo hacemos demasiado pronto, tendremos una variable no válida. Si lo hacemos dos veces, también es un error. Necesitamos emparejar exactamente uno allocatecon exactamente uno free.

Rust toma un camino diferente: la memoria se devuelve automáticamente cuando la variable que la contiene queda fuera del ámbito. Aquí hay una versión de nuestro ejemplo de ámbito del Listado 4-1 usando un Stringliteral en lugar de uno de cadena:

    {
        let s = String::from("hello"); // s is valid from this point forward

        // do stuff with s
    }                                  // this scope is now over, and s is no
                                       // longer valid
Existe un punto natural en el que podemos devolver la memoria que Stringnecesitamos al asignador: cuando sse sale del ámbito. Cuando una variable se sale del ámbito, Rust llama a una función especial. Esta función se llama drop, y es donde el autor de Stringpuede colocar el código para devolver la memoria. Rust llama dropautomáticamente al cerrar la llave.

Nota: En C++, este patrón de desasignación de recursos al final de la vida útil de un elemento se denomina a veces " Adquisición de Recursos en Inicialización" (RAII) . Esta dropfunción en Rust le resultará familiar si ha utilizado patrones RAII.

Este patrón tiene un profundo impacto en la forma en que se escribe el código de Rust. Puede parecer simple ahora, pero el comportamiento del código puede ser inesperado en situaciones más complejas cuando queremos que múltiples variables usen los datos que hemos asignado en el montón. Exploremos algunas de estas situaciones.


Variables y datos que interactúan con Move
En Rust, varias variables pueden interactuar con los mismos datos de diferentes maneras. El Listado 4-2 muestra un ejemplo con un entero.

    let x = 5;
    let y = x;
Listado 4-2 : Asignación del valor entero de la variable xay
Probablemente podamos adivinar qué hace esto: "Vincular el valor 5a x; luego, hacer una copia del valor en xy vincularla a y". Ahora tenemos dos variables, x y y, y ambas son iguales a 5. Esto es precisamente lo que ocurre, porque los enteros son valores simples con un tamaño fijo conocido, y estos dos 5valores se colocan en la pila.

Ahora veamos la Stringversión:

    let s1 = String::from("hello");
    let s2 = s1;
Esto parece muy similar, por lo que podríamos suponer que funciona de la misma manera: es decir, la segunda línea copiaría el valor en s1y lo vincularía a s2. Pero esto no es exactamente lo que sucede.

Observe la Figura 4-1 para ver qué sucede en Stringel subsistema. A Stringse compone de tres partes, que se muestran a la izquierda: un puntero a la memoria que contiene el contenido de la cadena, una longitud y una capacidad. Este grupo de datos se almacena en la pila. A la derecha se encuentra la memoria en el montón que contiene el contenido.

Dos tablas: la primera tabla contiene la representación de s1 en la pila, compuesta por su longitud (5), capacidad (5) y un puntero al primer valor de la segunda tabla. La segunda tabla contiene la representación de los datos de cadena en el montón, byte a byte.

Figura 4-1: La representación en memoria de un String valor "hello"ligado a un objeto.s1

La longitud indica la cantidad de memoria, en bytes, que el contenido del archivo Stringestá utilizando actualmente. La capacidad indica la cantidad total de memoria, en bytes, que el archivo Stringha recibido del asignador. La diferencia entre longitud y capacidad es importante, pero no en este contexto, por lo que, por ahora, se puede ignorar la capacidad.

Al asignar s1a s2, se copian los Stringdatos, es decir, se copian el puntero, la longitud y la capacidad de la pila. No se copian los datos del montón al que hace referencia el puntero. En otras palabras, la representación de los datos en memoria se muestra en la Figura 4-2.

Tres tablas: las tablas s1 y s2 representan aquellas cadenas en la pila, respectivamente, y ambas apuntan a los mismos datos de cadena en el montón.

Figura 4-2: La representación en memoria de la variable s2que tiene una copia del puntero, longitud y capacidad des1

La representación no se parece a la Figura 4-3, que es como se vería la memoria si Rust también copiara los datos del montón. Si Rust hiciera esto, la operación s2 = s1podría ser muy costosa en términos de rendimiento en tiempo de ejecución si los datos en el montón fueran grandes.

Cuatro tablas: dos tablas que representan los datos de la pila para s1 y s2, y cada una apunta a su propia copia de datos de cadena en el montón.

Figura 4-3: Otra posibilidad de lo que s2 = s1podría pasar si Rust también copiara los datos del montón

Anteriormente, mencionamos que cuando una variable queda fuera de alcance, Rust llama automáticamente a la dropfunción y limpia la memoria del montón de esa variable. Sin embargo, la Figura 4-2 muestra ambos punteros de datos apuntando a la misma ubicación. Esto representa un problema: cuando s2y s1quedan fuera de alcance, ambos intentarán liberar la misma memoria. Esto se conoce como error de doble liberación y es uno de los errores de seguridad de memoria que mencionamos anteriormente. Liberar memoria dos veces puede provocar corrupción de memoria, lo que potencialmente puede generar vulnerabilidades de seguridad.

Para garantizar la seguridad de la memoria, después de la línea let s2 = s1;, Rust s1la considera inválida. Por lo tanto, no necesita liberar nada al s1salir del ámbito. Observa lo que ocurre al intentar usar ` s1after` s2; no funcionará.

¡Este código no se compila!
let s1 = String::from("hello");
let s2 = s1;

    println!("{s1}, world!");
Recibirás un error como este porque Rust te impide usar la referencia invalidada:

$ cargo run
Compiling ownership v0.1.0 (file:///projects/ownership)
error[E0382]: borrow of moved value: `s1`
--> src/main.rs:5:15
|
2 |     let s1 = String::from("hello");
|         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
3 |     let s2 = s1;
|              -- value moved here
4 |
5 |     println!("{s1}, world!");
|               ^^^^ value borrowed here after move
|
= note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider cloning the value if the performance cost is acceptable
|
3 |     let s2 = s1.clone();
|                ++++++++

For more information about this error, try `rustc --explain E0382`.
error: could not compile `ownership` (bin "ownership") due to 1 previous error
Si has escuchado los términos " copia superficial" y "copia profunda" al trabajar con otros lenguajes, el concepto de copiar el puntero, la longitud y la capacidad sin copiar los datos probablemente te suene a una copia superficial. Sin embargo, como Rust también invalida la primera variable, en lugar de llamarse copia superficial, se conoce como movimiento . En este ejemplo, diríamos que s1 se movió a s2. Por lo tanto, lo que realmente sucede se muestra en la Figura 4-4.

Tres tablas: las tablas s1 y s2 representan respectivamente las cadenas en la pila, y ambas apuntan a los mismos datos de cadena en el montón. La tabla s1 está inactiva porque ya no es válida; solo se puede usar s2 para acceder a los datos del montón.

Figura 4-4: La representación en memoria después de s1haber sido invalidada

¡Eso soluciona nuestro problema! Con solo s2válido, cuando se sale del ámbito, solo esto liberará la memoria, y listo.

Además, esto implica una decisión de diseño: Rust nunca creará automáticamente copias "profundas" de tus datos. Por lo tanto, se puede asumir que cualquier copia automática será económica en términos de rendimiento en tiempo de ejecución.

Alcance y asignación
Lo contrario también aplica a la relación entre el alcance, la propiedad y la memoria liberada mediante la dropfunción. Al asignar un valor completamente nuevo a una variable existente, Rust llamará dropy liberará la memoria del valor original inmediatamente. Considere este código, por ejemplo:

    let mut s = String::from("hello");
    s = String::from("ahoy");

    println!("{s}, world!");
Inicialmente, declaramos una variable sy la vinculamos a a Stringcon el valor "hello". Inmediatamente después, creamos una nueva variable Stringcon el valor "ahoy" y la asignamos a s. En este punto, nada hace referencia al valor original en el montón. La Figura 4-5 ilustra los datos de la pila y el montón:

Una tabla que representa el valor de la cadena en la pila, que apunta a la segunda pieza de datos de la cadena (ahoy) en el montón, con los datos de la cadena original (hello) en gris porque ya no se puede acceder a ellos.

Figura 4-5: La representación en memoria después de que el valor inicial ha sido reemplazado en su totalidad

La cadena original queda inmediatamente fuera de alcance. Rust ejecutará la drop función en ella y su memoria se liberará inmediatamente. Al imprimir el valor al final, será "ahoy, world!".


Variables y datos que interactúan con Clone
Si queremos copiar en profundidad los datos del montón de String, no solo los de la pila, podemos usar un método común llamado clone. Analizaremos la sintaxis de los métodos en el Capítulo 5, pero como los métodos son una característica común en muchos lenguajes de programación, probablemente ya los hayas visto.

He aquí un ejemplo del clonemétodo en acción:

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {s1}, s2 = {s2}");
Esto funciona bien y produce explícitamente el comportamiento que se muestra en la Figura 4-3, donde los datos del montón se copian.

Cuando ves una llamada a clone, sabes que se está ejecutando código arbitrario, que puede ser costoso. Es un indicador visual de que algo diferente está sucediendo.

Datos de solo pila: Copiar
Hay otro detalle que aún no hemos abordado. Este código que usa enteros (parte del cual se mostró en el Listado 4-2) funciona y es válido:

    let x = 5;
    let y = x;

    println!("x = {x}, y = {y}");
Pero este código parece contradecir lo que acabamos de aprender: no tenemos una llamada a clone, pero xsigue siendo válido y no se movió a y.

La razón es que tipos como los enteros, que tienen un tamaño conocido en tiempo de compilación, se almacenan completamente en la pila, por lo que las copias de los valores reales se crean rápidamente. Esto significa que no hay razón para que queramos impedir xque sean válidos después de crear la variable y. En otras palabras, no hay diferencia entre la copia profunda y la superficial, por lo que la llamada cloneno tendría ningún efecto diferente a la copia superficial habitual, y podemos omitirla.

Rust cuenta con una anotación especial llamada Copytrait que podemos aplicar a tipos almacenados en la pila, como los enteros (hablaremos más sobre traits en el Capítulo 10 ). Si un tipo implementa el Copy trait, las variables que lo usan no se mueven, sino que se copian fácilmente, lo que las hace válidas tras asignarlas a otra variable.

Rust no nos permite anotar un tipo Copysi este, o alguna de sus partes, ha implementado la Dropcaracterística. Si el tipo requiere que ocurra algo especial cuando el valor queda fuera del ámbito y añadimos la Copyanotación, obtendremos un error de compilación. Para saber cómo añadir la Copyanotación a tu tipo para implementar la característica, consulta "Características Derivables" en el Apéndice C.

Entonces, ¿qué tipos implementan el Copyatributo? Puedes consultar la documentación del tipo dado para asegurarte, pero como regla general, cualquier grupo de valores escalares simples puede implementar `` Copy, y nada que requiera asignación o sea algún tipo de recurso puede implementar Copy``. Estos son algunos de los tipos que implementan ` Copy`:

Todos los tipos de enteros, como u32.
El tipo booleano, bool, con valores truey false.
Todos los tipos de punto flotante, como f64.
El tipo de carácter, char.
Tuplas, si solo contienen tipos que también implementan Copy. Por ejemplo, (i32, i32)implementa Copy, pero (i32, String)no lo hace.
Propiedad y funciones
La mecánica para pasar un valor a una función es similar a la de asignar un valor a una variable. Pasar una variable a una función implica mover o copiar, al igual que la asignación. El Listado 4-3 incluye un ejemplo con algunas anotaciones que muestran dónde entran y salen del ámbito las variables.

Nombre de archivo: src/main.rs
fn main() {
let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // Because i32 implements the Copy trait,
                                    // x does NOT move into the function,
                                    // so it's okay to use x afterward.

} // Here, x goes out of scope, then s. However, because s's value was moved,
// nothing special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
// memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.
Listado 4-3 : Funciones con propiedad y alcance anotados
Si intentáramos usar sdespués de llamar a takes_ownership, Rust generaría un error de compilación. Estas comprobaciones estáticas nos protegen de errores. Intenta añadir código a mainesos usos spara xver dónde puedes usarlos y dónde las reglas de propiedad te lo impiden.

Valores de retorno y alcance
Los valores devueltos también pueden transferir la propiedad. El Listado 4-4 muestra un ejemplo de una función que devuelve un valor, con anotaciones similares a las del Listado 4-3.

Nombre de archivo: src/main.rs
fn main() {
let s1 = gives_ownership();        // gives_ownership moves its return
// value into s1

    let s2 = String::from("hello");    // s2 comes into scope

    let s3 = takes_and_gives_back(s2); // s2 is moved into
                                       // takes_and_gives_back, which also
                                       // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
// happens. s1 goes out of scope and is dropped.

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
Listado 4-4 : Transferencia de la propiedad de los valores de retorno
La propiedad de una variable sigue siempre el mismo patrón: asignar un valor a otra variable la mueve. Cuando una variable que incluye datos en el montón queda fuera del alcance, el valor se borrará, dropa menos que la propiedad de los datos se haya movido a otra variable.

Si bien esto funciona, tomar y devolver la propiedad con cada función es un poco tedioso. ¿Qué pasa si queremos que una función use un valor, pero no que tome la propiedad? Es bastante molesto que todo lo que pasamos también tenga que devolverse si queremos usarlo de nuevo, además de cualquier dato resultante del cuerpo de la función que también queramos devolver.

Rust nos permite devolver múltiples valores usando una tupla, como se muestra en el Listado 4-5.

Nombre de archivo: src/main.rs
fn main() {
let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{s2}' is {len}.");
}

fn calculate_length(s: String) -> (String, usize) {
let length = s.len(); // len() returns the length of a String

    (s, length)
}
Listado 4-5 : Devolución de la propiedad de los parámetros
Pero esto es demasiada ceremonia y mucho trabajo para un concepto que debería ser común. Por suerte, Rust cuenta con una función para usar un valor sin transferir la propiedad: las referencias.

Referencias y préstamos
El problema con el código de tupla del Listado 4-5 es que debemos devolver `the` Stringa la función que lo llama para poder seguir usándolo Stringdespués de llamar a `` calculate_length, ya que `` Stringse movió a ` calculate_length`. En su lugar, podemos proporcionar una referencia al Stringvalor. Una referencia es como un puntero, ya que es una dirección que podemos seguir para acceder a los datos almacenados en ella; esos datos pertenecen a otra variable. A diferencia de un puntero, se garantiza que una referencia apuntará a un valor válido de un tipo específico durante su vigencia.

Aquí se explica cómo definiría y utilizaría una calculate_lengthfunción que tiene una referencia a un objeto como parámetro en lugar de tomar posesión del valor:

Nombre de archivo: src/main.rs
fn main() {
let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");
}

fn calculate_length(s: &String) -> usize {
s.len()
}
Primero, observe que todo el código de tupla en la declaración de la variable y el valor de retorno de la función ha desaparecido. Segundo, observe que pasamos &s1a calculate_lengthy, en su definición, tomamos &Stringen lugar de String. Estos símbolos & representan referencias y permiten referirse a un valor sin tomar posesión de él. La Figura 4-6 ilustra este concepto.

Tres tablas: la tabla para s contiene solo un puntero a la tabla para s1. La tabla para s1 contiene los datos de la pila para s1 y apunta a los datos de cadena en el montón.

Figura 4-6: Diagrama de &String scómo señalar String s1

Nota: Lo opuesto a referenciar mediante using &es desreferenciar , lo cual se logra con el operador de desreferencia *. Veremos algunos usos del operador de desreferencia en el Capítulo 8 y analizaremos los detalles de la desreferenciación en el Capítulo 15.

Echemos un vistazo más de cerca a la llamada de función aquí:

    let s1 = String::from("hello");

    let len = calculate_length(&s1);
La &s1sintaxis permite crear una referencia que remite al valor de, s1 pero no lo posee. Dado que la referencia no lo posee, el valor al que apunta no se eliminará cuando la referencia deje de usarse.

Asimismo, la firma de la función suele &indicar que el tipo del parámetro ses una referencia. Añadamos algunas anotaciones explicativas:

fn calculate_length(s: &String) -> usize { // s is a reference to a String
s.len()
} // Here, s goes out of scope. But because s does not have ownership of what
// it refers to, the String is not dropped.
El ámbito de validez de la variable ses el mismo que el de cualquier parámetro de función, pero el valor al que apunta la referencia no se descarta al sdejar de usarse, ya que sno tiene propiedad. Cuando las funciones tienen referencias como parámetros en lugar de los valores reales, no es necesario devolver los valores para devolver la propiedad, ya que nunca la tuvieron.

Llamamos préstamo a la acción de crear una referencia . Como en la vida real, si alguien posee algo, puedes pedírselo prestado. Al terminar, debes devolverlo. No eres el dueño.

Entonces, ¿qué pasa si intentamos modificar algo que tomamos prestado? Prueben el código del Listado 4-6. ¡Atención!: ¡No funciona!

Nombre de archivo: src/main.rs
¡Este código no se compila!
fn main() {
let s = String::from("hello");

    change(&s);
}

fn change(some_string: &String) {
some_string.push_str(", world");
}
Listado 4-6 : Intento de modificar un valor prestado
Aquí está el error:

$ cargo run
Compiling ownership v0.1.0 (file:///projects/ownership)
error[E0596]: cannot borrow `*some_string` as mutable, as it is behind a `&` reference
--> src/main.rs:8:5
|
8 |     some_string.push_str(", world");
|     ^^^^^^^^^^^ `some_string` is a `&` reference, so the data it refers to cannot be borrowed as mutable
|
help: consider changing this to be a mutable reference
|
7 | fn change(some_string: &mut String) {
|                         +++

For more information about this error, try `rustc --explain E0596`.
error: could not compile `ownership` (bin "ownership") due to 1 previous error
Así como las variables son inmutables por defecto, también lo son las referencias. No podemos modificar algo a lo que tenemos una referencia.

Referencias mutables
Podemos arreglar el código del Listado 4-6 para permitirnos modificar un valor prestado con solo unos pequeños ajustes que utilizan, en su lugar, una referencia mutable :

Nombre de archivo: src/main.rs
fn main() {
let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
some_string.push_str(", world");
}
Primero, cambiamos sa mut. Luego, creamos una referencia mutable con &mut sdonde llamamos a la changefunción y actualizamos la firma de la función para que acepte una referencia mutable con some_string: &mut String. Esto deja muy claro que la changefunción mutará el valor que toma prestado.

Las referencias mutables tienen una restricción importante: si se tiene una referencia mutable a un valor, no se pueden tener otras referencias a ese valor. Este código, que intenta crear dos referencias mutables, sfallará:

Nombre de archivo: src/main.rs
¡Este código no se compila!
let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;

    println!("{r1}, {r2}");
Aquí está el error:

$ cargo run
Compiling ownership v0.1.0 (file:///projects/ownership)
error[E0499]: cannot borrow `s` as mutable more than once at a time
--> src/main.rs:5:14
|
4 |     let r1 = &mut s;
|              ------ first mutable borrow occurs here
5 |     let r2 = &mut s;
|              ^^^^^^ second mutable borrow occurs here
6 |
7 |     println!("{r1}, {r2}");
|               ---- first borrow later used here

For more information about this error, try `rustc --explain E0499`.
error: could not compile `ownership` (bin "ownership") due to 1 previous error
Este error indica que este código no es válido porque no podemos tomar prestado scomo mutable más de una vez. El primer préstamo mutable está en [nombre del archivo] r1y debe durar hasta que se use en [ println!nombre del archivo], pero entre la creación de esa referencia mutable y su uso, intentamos crear otra referencia mutable en [nombre del archivo] r2que toma prestados los mismos datos que [nombre del archivo] r1.

La restricción que impide múltiples referencias mutables a los mismos datos simultáneamente permite la mutación, pero de forma muy controlada. Es algo con lo que los nuevos usuarios de Rust tienen dificultades, ya que la mayoría de los lenguajes permiten mutar cuando se desee. La ventaja de esta restricción es que Rust puede evitar las carreras de datos en tiempo de compilación. Una carrera de datos es similar a una condición de carrera y ocurre cuando se presentan estos tres comportamientos:

Dos o más punteros acceden a los mismos datos al mismo tiempo.
Se está utilizando al menos uno de los punteros para escribir los datos.
No se utiliza ningún mecanismo para sincronizar el acceso a los datos.
Las carreras de datos causan un comportamiento indefinido y pueden ser difíciles de diagnosticar y solucionar cuando intentas rastrearlas en tiempo de ejecución; Rust previene este problema al negarse a compilar código con carreras de datos.

Como siempre, podemos usar llaves para crear un nuevo ámbito, lo que permite múltiples referencias mutables, pero no simultáneas :

    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;
Rust aplica una regla similar para combinar referencias mutables e inmutables. Este código genera un error:

¡Este código no se compila!
let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM

    println!("{r1}, {r2}, and {r3}");
Aquí está el error:

$ cargo run
Compiling ownership v0.1.0 (file:///projects/ownership)
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
--> src/main.rs:6:14
|
4 |     let r1 = &s; // no problem
|              -- immutable borrow occurs here
5 |     let r2 = &s; // no problem
6 |     let r3 = &mut s; // BIG PROBLEM
|              ^^^^^^ mutable borrow occurs here
7 |
8 |     println!("{r1}, {r2}, and {r3}");
|               ---- immutable borrow later used here

For more information about this error, try `rustc --explain E0502`.
error: could not compile `ownership` (bin "ownership") due to 1 previous error
¡Uf! Tampoco podemos tener una referencia mutable mientras tengamos una inmutable al mismo valor.

Los usuarios de una referencia inmutable no esperan que el valor cambie repentinamente. Sin embargo, se permiten múltiples referencias inmutables porque nadie que simplemente lea los datos puede influir en la lectura de los datos de otros.

Tenga en cuenta que el alcance de una referencia comienza desde su introducción y continúa hasta su último uso. Por ejemplo, este código se compilará porque el último uso de las referencias inmutables está en el println!, antes de introducir la referencia mutable:

    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{r1} and {r2}");
    // Variables r1 and r2 will not be used after this point.

    let r3 = &mut s; // no problem
    println!("{r3}");
Los alcances de las referencias inmutables r1terminan r2después de println! su último uso, es decir, antes de la r3creación de la referencia mutable. Estos alcances no se superponen, por lo que se permite este código: el compilador puede determinar que la referencia ya no se utiliza en un punto anterior al final del alcance.

Aunque los errores de préstamo pueden ser frustrantes a veces, recuerda que es el compilador de Rust quien señala un posible error con antelación (en tiempo de compilación, no de ejecución) y te muestra exactamente dónde está el problema. Así, no tendrás que buscar por qué tus datos no son lo que creías.

Referencias colgantes
En lenguajes con punteros, es fácil crear erróneamente un puntero colgante (un puntero que referencia a una ubicación en memoria que podría haber sido asignada a otra persona) al liberar memoria y conservar un puntero a dicha memoria. En Rust, por el contrario, el compilador garantiza que las referencias nunca serán colgantes: si se tiene una referencia a datos, el compilador se asegurará de que estos no salgan del ámbito antes que la referencia a ellos.

Intentemos crear una referencia colgante para ver cómo Rust las evita con un error en tiempo de compilación:

Nombre de archivo: src/main.rs
¡Este código no se compila!
fn main() {
let reference_to_nothing = dangle();
}

fn dangle() -> &String {
let s = String::from("hello");

    &s
}
Aquí está el error:

$ cargo run
Compiling ownership v0.1.0 (file:///projects/ownership)
error[E0106]: missing lifetime specifier
--> src/main.rs:5:16
|
5 | fn dangle() -> &String {
|                ^ expected named lifetime parameter
|
= help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
help: consider using the `'static` lifetime, but this is uncommon unless you're returning a borrowed value from a `const` or a `static`
|
5 | fn dangle() -> &'static String {
|                 +++++++
help: instead, you are more likely to want to return an owned value
|
5 - fn dangle() -> &String {
5 + fn dangle() -> String {
|

error[E0515]: cannot return reference to local variable `s`
--> src/main.rs:8:5
|
8 |     &s
|     ^^ returns a reference to data owned by the current function

Some errors have detailed explanations: E0106, E0515.
For more information about an error, try `rustc --explain E0106`.
error: could not compile `ownership` (bin "ownership") due to 2 previous errors
Este mensaje de error se refiere a una característica que aún no hemos abordado: los tiempos de vida. Analizaremos los tiempos de vida en detalle en el Capítulo 10. Sin embargo, si ignoramos la información sobre los tiempos de vida, el mensaje contiene la clave del problema de este código:

this function's return type contains a borrowed value, but there is no value
for it to be borrowed from
Echemos un vistazo más de cerca a qué sucede exactamente en cada etapa de nuestro danglecódigo:

Nombre de archivo: src/main.rs
¡Este código no se compila!
fn dangle() -> &String { // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope and is dropped, so its memory goes away.
// Danger!
Dado que sse crea dentro de [nombre del archivo] , al finalizar dangleel código de [ nombre del archivo], se desasignará. Pero intentamos devolver una referencia a él. Esto significa que esta referencia estaría apuntando a un [nombre del archivo] no válido . ¡Eso es un error! Rust no nos permite hacerlo.danglesString

La solución aquí es devolverlo Stringdirectamente:

fn no_dangle() -> String {
let s = String::from("hello");

    s
}
Esto funciona sin problemas. Se transfiere la propiedad y no se desasigna nada.

Las reglas de referencias
Recapitulemos lo que hemos discutido sobre las referencias:

En cualquier momento, puede tener una referencia mutable o cualquier cantidad de referencias inmutables.
Las referencias siempre deben ser válidas.
A continuación, veremos un tipo diferente de referencia: las rebanadas.
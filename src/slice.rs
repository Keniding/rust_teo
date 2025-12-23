pub fn slice() {
    // Slice o rebanada
    // Las porciones permiten referenciar una secuencia contigua de elementos en una colección . Una porción es un tipo de referencia, por lo que no tiene propiedad.
    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5

    s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but s no longer has any content that we
    // could meaningfully use with the value 5, so word is now totally invalid!
    println!("{}", word);
    // Problemas:
    // Este programa se compila sin errores y también lo haría si usáramos word después de llamar a s.clear(). Dado wordque no está conectado al estado de s , wordaún contiene el valor 5. Podríamos usar ese valor 5con la variable spara intentar extraer la primera palabra, pero esto sería un error, ya que el contenido de sha cambiado desde que lo guardamos 5en word.
    // Preocuparse por la worddesincronización del índice con los datos ses tedioso y propenso a errores. Gestionar estos índices es aún más complejo si escribimos una second_wordfunción. Su firma debería ser así:
    // fn second_word(s: &String) -> (usize, usize) {
    // Ahora rastreamos un índice inicial y uno final, y tenemos aún más valores calculados a partir de datos de un estado específico, pero que no están vinculados a él. Tenemos tres variables no relacionadas que deben mantenerse sincronizadas.
    // Afortunadamente, Rust tiene una solución para este problema: porciones de cadenas.

    // String Slices
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{}, {}", hello, world);
    // En lugar de una referencia a el segmento String, hellose refiere a una parte del segmento String, especificada en el [0..5]bit adicional. Creamos segmentos usando un rango entre corchetes, especificando [starting_index..ending_index], donde starting_indexes la primera posición del segmento y ending_indexes uno más que la última posición. Internamente, la estructura de datos del segmento almacena la posición inicial y la longitud del segmento, que corresponde a ending_indexmenos starting_index. Por lo tanto, en el caso de let world = &s[6..11];, worldsería un segmento que contiene un puntero al byte en el índice 6 de scon un valor de longitud de 5.

    // Uso de ..x para mejorar la sintaxis
    let s = String::from("hello");
    let slice = &s[0..2];
    println!("{}, {}", slice, s);
    let slice = &s[..2];
    println!("{}", slice);
    // Similar para últimos dígitos
    let len = s.len();
    let slice = &s[3..len];
    println!("{}", slice);
    let slice = &s[3..];
    println!("{}", slice);
    // O del completo
    let slice = &s[0..len];
    println!("{}", slice);
    let slice = &s[..];
    println!("{}", slice);

    let mut  s = String::from("hello world");
    // Fn with slice
    first_world_with_slice(&s);

    s.clear();
    println!("{}", s);

    // Use str
    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole.
    let word = first_world_with_slice_str(&my_string[0..6]);
    println!("{}", word);
    let word = first_world_with_slice_str(&my_string[..]);
    println!("{}", word);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s.
    let word = first_world_with_slice_str(&my_string);
    println!("{}", word);
    let  my_string_literal = "hello world";
    // `first_word` works on slices of string literals, whether partial or
    // whole.
    let word = first_world_with_slice_str(&my_string_literal[0..5]);
    println!("{}", word);
    let word = first_world_with_slice_str(&my_string_literal[..]);
    println!("{}", word);
    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_world_with_slice_str(my_string_literal);
    println!("{}", word);

    // Otras rebanadas/slice
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
    
    // Los conceptos de propiedad, préstamo y porciones garantizan la seguridad de la memoria en los programas Rust durante la compilación. El lenguaje Rust te permite controlar el uso de la memoria, al igual que otros lenguajes de programación de sistemas. Sin embargo, al permitir que el propietario de los datos los limpie automáticamente cuando este deja de estar dentro del alcance, no es necesario escribir ni depurar código adicional para obtener este control.
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_world_with_slice(s: &String) -> &str {
// Si tenemos una porción de cadena, podemos pasarla directamente. Si tenemos un String, podemos pasar una porción de Stringo una referencia a String. Esta flexibilidad aprovecha las coerciones de desreferencia
// fn first_world_with_slice(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn first_world_with_slice_str(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
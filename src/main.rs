/*
 * Reto #3
 * ¿ES UN NÚMERO PRIMO?
 * Fecha publicación enunciado: 17/01/22
 * Fecha publicación resolución: 24/01/22
 * Dificultad: MEDIA
 *
 * Enunciado: Escribe un programa que se encargue de comprobar si un número es o no primo.
 * Hecho esto, imprime los números primos entre 1 y 100.
 *
 * Información adicional:
 * - Usa el canal de nuestro discord (https://mouredev.com/discord) "🔁reto-semanal" para preguntas, dudas o prestar ayuda a la acomunidad.
 * - Puedes hacer un Fork del repo y una Pull Request al repo original para que veamos tu solución aportada.
 * - Revisaré el ejercicio en directo desde Twitch el lunes siguiente al de su publicación.
 * - Subiré una posible solución al ejercicio el lunes siguiente al de su publicación.
 *
 */

fn main() {
    for number in 1..101 {
        if is_prime(number) {
            println!("{}", number)
        }
    }
}

fn is_prime(number: i64) -> bool {
    let mut sum_numbers: i64 = 0;

    for current_number in 1..number+1 {
       if number % current_number == 0 {
          sum_numbers += 1;
       }
    }

    if sum_numbers == 2 {
       return true
    }

   return false
}

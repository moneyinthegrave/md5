use std::mem;

mod utils;
mod registrar;

enum Action {
    Register,
    Login,
}

// TODO
// 1. Rewrite login process similar (no the same) to Registrar
// 2. Move to crypto to separate module
// 3. Proper error types instead of Strings
// 4. Write proper module docs/function docs and README

fn main() {
    if let Err(e) = run() {
        println!("Got error: {:?}", e);
    }
}

fn run() -> Result<(), String> {
    let action = Action::parse_cli()?;
    action.run()
}

impl Action {
    fn parse_cli() -> Result<Self, String> {
        println!(
            "Введите номер действия:
            1 Регистрация
            2 Вход"
        );
        let action = utils::read_stdin()?;
        match action.as_str() {
            "1" => Ok(Action::Register),
            "2" => Ok(Action::Login),
            _ => Err("Unknown action".to_string()),
        }
    }

    fn run(&self) -> Result<(), String> {
        match self {
            Action::Register => registrar::Registrar::register(),
            Action::Login => todo!(),
        }
    }
}

// fn main_1() {
//     /*
//     // Откуда эти тестовые варианты?
//     assert!(md5_utf8("") == "d41d8cd98f00b204e9800998ecf8427e");
//     assert!(md5_utf8("abc") == "900150983cd24fb0d6963f7d28e17f72");
//     assert!(md5_utf8("message digest") == "f96b697d7cb7938d525a2f31aaf161d0");
//     assert!(md5_utf8("abcdefghijklmnopqrstuvwxyz") == "c3fcd3d76192e4007dfb496cca67e13b");
//     assert!(md5_utf8("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789") == "d174ab98d277d9f5a5611c2c9f419d9f");
//     assert!(md5_utf8("12345678901234567890123456789012345678901234567890123456789012345678901234567890") == "57edf4a22be3c955ac49da2e2107b67a");
//     */
//
//     println!(
//         "Введите номер действия:
//         1 Регистрация
//         2 Вход"
//     );
//     let mut good_login = false;
//     let mut num = 0;
//     let mut bad_login = 0; // счетчик ошибок
//     let mut action = String::new(); // выбор в меню
//     io::stdin().read_line(&mut action);
//     let path_admin = "admin_dock.txt"; // данные для админа
//     let path_user = "user_dock.txt"; // данные для обычного пользователя
//
//     if "2" == action.trim() {
//         // вход
//         while good_login != true {
//             println!("Введите логин:\t");
//             let mut login_authorization = String::new();
//             io::stdin().read_line(&mut login_authorization);
//             // пароль
//             println!("Введите пароль:\t");
//             let mut password_authorization = String::new();
//             io::stdin().read_line(&mut password_authorization);
//             // уровень доступа
//
//             println!("init {}", num);
//             //сравнить данные из файла с тем что ввел пользователь
//             let file = File::open(users_hashed_credentials_path).unwrap();
//             let reader = BufReader::new(file);
//             // чтение файла построчно используя lines() итератор из std::io::BufRead
//             for (index, line) in reader.lines().enumerate() {
//                 // enumerate перечисление — итератор
//                 let line = line.unwrap(); // игнорировать ошибки
//                 if line.contains(&md5_utf8(&*login_authorization.trim()))
//                     && line.contains(&md5_utf8(&*password_authorization.trim()))
//                 {
//                     println!("успешный вход");
//                     good_login = true;
//                     if line.contains("admin") {
//                         num = 1;
//                         println!("change {}", num);
//                     } else {
//                         num = 2;
//                         println!("change {}", num);
//                     }
//                 }
//             }
//
//             // если данные верны то по уровню выдать файлы admin_dock и user_dock
//             // 1 = админ
//             println!("before if {}", num);
//             if good_login == true && num == 1 {
//                 // данные для пользователя
//                 let file = File::open(path_admin).unwrap();
//                 let reader = BufReader::new(file);
//                 for (index, line) in reader.lines().enumerate() {
//                     let line = line.unwrap();
//                     println!("{}. {}", index + 1, line);
//                 }
//             }
//             // 2 = пользователь
//             else if good_login == true && num == 2 {
//                 // данные для обычного пользователя
//                 let file = File::open(path_user).unwrap();
//                 let reader = BufReader::new(file);
//                 for (index, line) in reader.lines().enumerate() {
//                     let line = line.unwrap();
//                     println!("{}. {}", index + 1, line);
//                 }
//             } else {
//                 println!("Неправильный логин/пароль");
//             }
//             if good_login == false {
//                 bad_login += 1;
//             }
//
//             // если ошибка 3 раза то заблокировать ввод
//             if bad_login == 3 {
//                 let block = Duration::from_secs(1);
//                 for sec_left in (0..60).rev() {
//                     println!("блокировка {}с", sec_left);
//                     std::thread::sleep(block);
//                 }
//                 bad_login = 0;
//             }
//         }
//         println!("Успешный вход.\t");
//     } else {
//         panic!("Ошибка");
//     }
// }

fn md5(mut msg: Vec<u8>) -> (u32, u32, u32, u32) {
    let bitcount = msg.len().saturating_mul(8) as u64;
    // pub const fn saturating_mul(self, rhs: u8) -> u8
    // Насыщающее целочисленное умножение. Вычисляет self * rhs, насыщая числовые границы вместо переполнения.
    // Насыщенность арифметическая, в арифметике, разновидность арифметики, при которой все операции ограничены в заданном диапазоне;

    // Добавление битов заполнения
    msg.push(0b10000000); // добавление единичного бита в поток
    while (msg.len() * 8) % 512 != 448 {
        msg.push(0u8); // добавление нулевых битов в поток
    }

    // Добавление длины (по 64 бит) до 512 бит
    // Типаж Extend заполняет этот пробел, позволяя расширять коллекцию(Vec), включая содержимое этого итератора.
    // При расширении коллекции с помощью уже существующего ключа эта запись
    // допускающих несколько записей с одинаковыми ключами, эта запись вставляется.
    msg.extend(&[
        bitcount as u8,
        (bitcount >> 8) as u8,
        (bitcount >> 16) as u8,
        (bitcount >> 24) as u8,
        (bitcount >> 32) as u8,
        (bitcount >> 40) as u8,
        (bitcount >> 48) as u8,
        (bitcount >> 56) as u8,
    ]);

    // Инициализация буфера
    /* Буфер из четырех слов (A, B, C, D) используется для промежуточных вычислений.
    Порядок байтов little-endian
    Здесь каждый из A, B, C, D является 32-битным регистром. */
    let mut A = 0x67452301u32; // word A: 01 23 45 67
    let mut B = 0xefcdab89u32; // word B: 89 ab cd ef
    let mut C = 0x98badcfeu32; // word C: fe dc ba 98
    let mut D = 0x10325476u32; // word D: 76 54 32 10

    // Обрабротка сообщения блоками по 16 слов
    /* Определение 4ых вспомогательных функций */
    // Введём функции от трёх параметров — слов, результатом также будет слово
    let F = |X: u32, Y: u32, Z: u32| -> u32 { X & Y | !X & Z };
    let G = |X: u32, Y: u32, Z: u32| -> u32 { X & Z | Y & !Z };
    let H = |X: u32, Y: u32, Z: u32| -> u32 { X ^ Y ^ Z };
    let I = |X: u32, Y: u32, Z: u32| -> u32 { Y ^ (X | !Z) };

    /* На этом шаге используется таблица T [1 ... 64] из 64 элементов, построенная на основе синусоидальной функции. */
    let T = [
        0x00000000, // разрешить использование в качестве 1-индексированной таблицы
        0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee, 0xf57c0faf, 0x4787c62a, 0xa8304613,
        0xfd469501, 0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be, 0x6b901122, 0xfd987193,
        0xa679438e, 0x49b40821, 0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa, 0xd62f105d,
        0x02441453, 0xd8a1e681, 0xe7d3fbc8, 0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed,
        0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a, 0xfffa3942, 0x8771f681, 0x6d9d6122,
        0xfde5380c, 0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70, 0x289b7ec6, 0xeaa127fa,
        0xd4ef3085, 0x04881d05, 0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665, 0xf4292244,
        0x432aff97, 0xab9423a7, 0xfc93a039, 0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1,
        0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1, 0xf7537e82, 0xbd3af235, 0x2ad7d2bb,
        0xeb86d391,
    ];

    /* Обработайте каждый блок из 16 слов. (поскольку 1 слово составляет 4 байта, то 16 слов составляют 64 байта) */
    for mut block in msg.chunks_exact_mut(64) {
        // Возвращает итератор по элементам(64) chunk_size среза за раз, начиная с начала среза.
        // Срез позволяет ссылаться на смежную последовательность элементов из коллекции, вместо полной коллекции.

        /* Копирование блока в Х. */
        #![allow(unused_mut)] // увеличение объема разрешения позволяет избежать предупреждения
        let mut X = unsafe { mem::transmute::<&mut [u8], &mut [u32]>(&mut block) };
        // Копирует биты из исходного значения в целевое значение, а затем забывает оригинал
        // Похоже на memcpy из С (копирование одной области памяти в другую)

        /* Сохранить регистры A, B, C, D */
        let AA = A;
        let BB = B;
        let DD = D;

        /* Round 1.  Пусть [abcd k s i] обозначают операцию
        a = b + ((a + F(b,c,d) + X[k] + T[i]) <<< s). */
        macro_rules! op1 { // Декларативные макросы(макросы на примере)
            ($a:ident,$b:ident,$c:ident,$d:ident,$k:expr,$s:expr,$i:expr) => { // идентификация и выражение
            // pub const fn wrapping_sub(self, rhs: usize) -> usize
            // Упаковочное (модульное) дополнение. Вычисляет self + rhs, охватывая границу типа.
                $a = $b.wrapping_add(
                    ($a.wrapping_add(F($b, $c, $d))
                        .wrapping_add(X[$k])
                        .wrapping_add(T[$i]))
                    .rotate_left($s), // битовый сдвиг
                /*  Поворачивает фрагмент на месте таким образом, что первые средние элементы фрагмента перемещаются
                    в конец, а последние элементы self.len () - mid перемещаются вперед. После вызова rotate_left элемент,
                    ранее находившийся в середине индекса, станет первым элементом в срезе.     */
                )
            };
        }

        /* Выполнить следующие 16 операций. */
        op1!(A, B, C, D, 0, 7, 1);
        op1!(D, A, B, C, 1, 12, 2);
        op1!(C, D, A, B, 2, 17, 3);
        op1!(B, C, D, A, 3, 22, 4);

        op1!(A, B, C, D, 4, 7, 5);
        op1!(D, A, B, C, 5, 12, 6);
        op1!(C, D, A, B, 6, 17, 7);
        op1!(B, C, D, A, 7, 22, 8);

        op1!(A, B, C, D, 8, 7, 9);
        op1!(D, A, B, C, 9, 12, 10);
        op1!(C, D, A, B, 10, 17, 11);
        op1!(B, C, D, A, 11, 22, 12);

        op1!(A, B, C, D, 12, 7, 13);
        op1!(D, A, B, C, 13, 12, 14);
        op1!(C, D, A, B, 14, 17, 15);
        op1!(B, C, D, A, 15, 22, 16);

        /* Round 2. Пусть [abcd k s i] обозначают операцию
        a = b + ((a + G(b,c,d) + X[k] + T[i]) <<< s). */
        macro_rules! op2 {
            ($a:ident,$b:ident,$c:ident,$d:ident,$k:expr,$s:expr,$i:expr) => {
                $a = $b.wrapping_add(
                    ($a.wrapping_add(G($b, $c, $d))
                        .wrapping_add(X[$k])
                        .wrapping_add(T[$i]))
                    .rotate_left($s),
                )
            };
        }

        /* Выполнить следующие 16 операций. */
        op2!(A, B, C, D, 1, 5, 17);
        op2!(D, A, B, C, 6, 9, 18);
        op2!(C, D, A, B, 11, 14, 19);
        op2!(B, C, D, A, 0, 20, 20);

        op2!(A, B, C, D, 5, 5, 21);
        op2!(D, A, B, C, 10, 9, 22);
        op2!(C, D, A, B, 15, 14, 23);
        op2!(B, C, D, A, 4, 20, 24);

        op2!(A, B, C, D, 9, 5, 25);
        op2!(D, A, B, C, 14, 9, 26);
        op2!(C, D, A, B, 3, 14, 27);
        op2!(B, C, D, A, 8, 20, 28);

        op2!(A, B, C, D, 13, 5, 29);
        op2!(D, A, B, C, 2, 9, 30);
        op2!(C, D, A, B, 7, 14, 31);
        op2!(B, C, D, A, 12, 20, 32);

        /* Round 3. Пусть [abcd k s t] обозначают операцию
        a = b + ((a + H(b,c,d) + X[k] + T[i]) <<< s). */
        macro_rules! op3 {
            ($a:ident,$b:ident,$c:ident,$d:ident,$k:expr,$s:expr,$i:expr) => {
                $a = $b.wrapping_add(
                    ($a.wrapping_add(H($b, $c, $d))
                        .wrapping_add(X[$k])
                        .wrapping_add(T[$i]))
                    .rotate_left($s),
                )
            };
        }

        /* Выполнить следующие 16 операций. */
        op3!(A, B, C, D, 5, 4, 33);
        op3!(D, A, B, C, 8, 11, 34);
        op3!(C, D, A, B, 11, 16, 35);
        op3!(B, C, D, A, 14, 23, 36);

        op3!(A, B, C, D, 1, 4, 37);
        op3!(D, A, B, C, 4, 11, 38);
        op3!(C, D, A, B, 7, 16, 39);
        op3!(B, C, D, A, 10, 23, 40);

        op3!(A, B, C, D, 13, 4, 41);
        op3!(D, A, B, C, 0, 11, 42);
        op3!(C, D, A, B, 3, 16, 43);
        op3!(B, C, D, A, 6, 23, 44);

        op3!(A, B, C, D, 9, 4, 45);
        op3!(D, A, B, C, 12, 11, 46);
        op3!(C, D, A, B, 15, 16, 47);
        op3!(B, C, D, A, 2, 23, 48);

        /* Round 4. Пусть [abcd k s t] обозначают операцию
        a = b + ((a + I(b,c,d) + X[k] + T[i]) <<< s). */
        macro_rules! op4 {
            ($a:ident,$b:ident,$c:ident,$d:ident,$k:expr,$s:expr,$i:expr) => {
                $a = $b.wrapping_add(
                    ($a.wrapping_add(I($b, $c, $d))
                        .wrapping_add(X[$k])
                        .wrapping_add(T[$i]))
                    .rotate_left($s),
                )
            };
        }

        /* Выполнить следующие 16 операций. */
        op4!(A, B, C, D, 0, 6, 49);
        op4!(D, A, B, C, 7, 10, 50);
        op4!(C, D, A, B, 14, 15, 51);
        op4!(B, C, D, A, 5, 21, 52);

        op4!(A, B, C, D, 12, 6, 53);
        op4!(D, A, B, C, 3, 10, 54);
        op4!(C, D, A, B, 10, 15, 55);
        op4!(B, C, D, A, 1, 21, 56);

        op4!(A, B, C, D, 8, 6, 57);
        op4!(D, A, B, C, 15, 10, 58);
        op4!(C, D, A, B, 6, 15, 59);
        op4!(B, C, D, A, 13, 21, 60);

        op4!(A, B, C, D, 4, 6, 61);
        op4!(D, A, B, C, 11, 10, 62);
        op4!(C, D, A, B, 2, 15, 63);
        op4!(B, C, D, A, 9, 21, 64);

        /* . . . увеличить каждый из четырех регистров на значение
        это было до того, как этот блок был запущен.) */

        A = A.wrapping_add(AA);
        B = B.wrapping_add(BB);
        D = D.wrapping_add(DD);
    }
    (
        // Изменение порядка байтов целого числа на обратный.
        A.swap_bytes(),
        B.swap_bytes(),
        C.swap_bytes(),
        D.swap_bytes(),
    )
}

fn md5_utf8(smsg: &str) -> String {
    let mut msg = vec![0u8; 0]; // создание вектора
    msg.extend(smsg.as_bytes()); // as_bytes преобразует фрагмент строки в фрагмент байта.
    let (A, B, C, D) = md5(msg);
    format!("{:08x}{:08x}{:08x}{:08x}", A, B, C, D) // Вызывает панику, если реализация признака форматирования возвращает ошибку.
}

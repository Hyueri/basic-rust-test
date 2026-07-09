
//basic / awalan rust isiny gini
fn main() {
    println!("Hello, world!");

//ga akan bisa dirun karena ada 3 main function (yajelas lah ya, aneh bet kalo nyoba dirun)


//pake println! macro munculin line kebawah

    println!("ohayo, sekai!");
    println!("test again!");
    println!("try better next time!");


//pake print! macro munculin line ke samping
//kalo mau line kebawah harus pake \n

    print!("ohayo, sekai!");
    print!("test again!\n");
    print!("try better next time!\n");





//Variabel

//Let
/*digunakan untuk mendeklarasikan variabel, tapi sifatnya immutable (tidak bisa diubah). Contohnya*/ 

    let name = "Aria";
    println!("My First Name is: {}", name);

//{} digunakan untuk menampilkan value dari variabel name, dan {} akan diganti dengan value dari variabel name.

//contoh 2
    let name = "Aria";
    let age = "21";
    println!("{} is {} years old.", name, age);

//gampangnya {} jadi placeholder, urutannya dari atas ke bawah sesuai urutan variablemu.

//mutable variable 
//pakai keyword mut biar bisa diubah value nya
    let mut x = 10;
    println!("before: {}", x);
    x = 20;
    println!("after: {}", x);


//tipe data
/* ga kaya C/Java, rust ga perlu deklarasi tipe data, tapi bisa juga deklarasi tipe data biar lebih jelas. Contohnya*/

//tanpa deklarasi 

    let my_num = 5; //integer
    let my_float = 5.0; //float
    let my_letter = 'A'; //char
    let my_string = "Hello"; //string
    let my_bool = true; //boolean

    println!("my_num: {}", my_num);
    println!("my_float: {}", my_float);
    println!("my_letter: {}", my_letter);
    println!("my_string: {}", my_string);
    println!("my_bool: {}", my_bool);

    
//dengan deklarasi

    let num: i32 = 5; //integer
    let float: f64 = 5.0; //float
    let letter: char = 'A'; //char
    let string: &str = "Hello"; //string
    let boole: bool = true; //boolean

    println!("num: {}", num);
    println!("float: {}", float);
    println!("letter: {}", letter);
    println!("string: {}", string);
    println!("boole: {}", boole);

/*i32 sm f64 itu tipe data integer dan float, char itu tipe data karakter, &str itu tipe data string, bool itu tipe data boolean (true or false).*/

//Constants
//const digunakan untuk mendeklarasikan variabel yang nilainya tidak bisa diubah, dan harus di deklarasi. Contohnya

    const BIRTH_YEAR: i32 = 2002;
    const MINUTE: i32 = 60;

    println!("I was born in {}.", BIRTH_YEAR);
    println!("There are {} minutes in an hour.", MINUTE);

//tanpa deklarasi yang ada error
//disarankan pakai uppercase letter, ya karena pada gitu sih mending ikutin dah

//Operators
//Operator digunakan untuk melakukan operasi pada variabel.

//aritmetic operators

    let add = 5 + 4;
    let sub = 8 - 4;
    let mul = 4 * 2;
    let div = 8 / 2;
    let rem = 15 % 3;

    println!("5 + 4 = {}", add);
    println!("8 - 4 = {}", sub);
    println!("4 * 2 = {}", mul);
    println!("8 / 2 = {}", div);
    println!("15 % 3 = {}", rem);


//assignment operators
//assignment operators digunakan untuk mengubah value dari variabel. Contohnya
    let mut x = 5;
    x += 2; //x = x + 2
    println!("x += 2: {}", x);
    x -= 3; //x = x - 3
    println!("x -= 3: {}", x);
    x *= 2; //x = x * 2
    println!("x *= 2: {}", x);
    x /= 2; //x = x / 2
    println!("x /= 2: {}", x);
    x %= 3; //x = x % 3
    println!("x %= 3: {}", x);

//comparison operators
//comparison operators digunakan untuk membandingkan value dari variabel. Contohnya
    let x = 4;
    let y = 12;

    println!("x = 4, y = 12");
    println!("x == y: {}", x == y); //false, == operator untuk membandingkan apakah x sama dengan y
    println!("x != y: {}", x != y); //true, != operator untuk membandingkan apakah x tidak sama dengan y
    println!("x > y: {}", x > y); //false, > operator untuk membandingkan apakah x lebih besar dari y
    println!("x < y: {}", x < y); //true, < operator untuk membandingkan apakah x lebih kecil dari y
    println!("x >= y: {}", x >= y); //false, >= operator untuk membandingkan apakah x lebih besar atau sama dengan y
    println!("x <= y: {}", x <= y); //true, <= operator untuk membandingkan apakah x lebih kecil atau sama dengan y

//logical operators
//logical operators biasanya digunakan untuk boolean.

    let logged_in = true;
    let admin_permission = false;

    println!("logged_in = true, admin_permission = false");
    println!("logged_in && admin_permission: {}", logged_in && admin_permission); //false, && operator untuk membandingkan apakah logged_in dan admin_permission sama-sama true
    println!("logged_in || admin_permission: {}", logged_in || admin_permission); //true, || operator untuk membandingkan apakah logged_in atau admin_permission salah satu true
    println!("!logged_in: {}", !logged_in); //false, ! operator untuk membandingkan apakah logged_in false

//boolean operators
//biasa digunakan untuk data yang hanya butuh 2 value true/false, on/off, yes/no

//contoh penggunaan yg simpel

    let is_raining = true;
    let is_sunny = false;

    println!("is_raining = true, is_sunny = false");
    println!("is it raining? : {}", is_raining); //true
    println!("is it sunny? : {}", is_sunny); //false

//boolean comparison

/* kadang ga harus true false, bisa juga buat membandingkan value dari variabel, misal untuk menentukan apakah umur seseorang sudah cukup untuk memiliki SIM (Surat Izin Mengemudi) atau belum. Contohnya */

    let age = 15;
    let driving_license = age >= 18;

    println!("age = 15, driving_license = age >= 18");
    println!("is your age eligible for a driving license? : {}", driving_license);

//boolean pakai if statement

    let logged_in = true;

    if logged_in {
        println!("Welcome back, Boss!");
    } else {
        println!("You are not logged in, please log in first.");
    }

//if..else

//if then else 

    let age = 17;

    if age >= 18 {
        println!("You are eligible for a driving license.");
    } else {
        println!("You are not eligible for a driving license.");
    }

//else if / elif

    let grade = 75;

    if grade >= 90 {
        println!("You got an A.");
    } else if grade >= 80 {
        println!("You got a B.");
    } else if grade >= 70 {
        println!("You got a C.");
    } else if grade >= 60 {
        println!("You got a D.");
    } else {
        println!("You got an F.");
    }

//if as expression

    let clock = 10;
    let time_of_day = if clock < 12 {
        "morning"
    } else if clock < 18 {
        "afternoon"
    } else {
        "evening"
    };

    println!("time of day: {}", time_of_day);

// Match
//kalo banyak pilihan, daripada nulis elif banyak banget mending pake match

    let no_button = 3;

    match no_button {
        1 => println!("Button 1 pressed."),
        2 => println!("Button 2 pressed."),
        3 => println!("Button 3 pressed."),
        4 => println!("Button 4 pressed."),
        5 => println!("Button 5 pressed."),
        6 => println!("Button 6 pressed."),
        _ => println!("Invalid button pressed."),
    }

//multi match
//pake operator | untuk menggabungkan beberapa pilihan

    let day = 4;

    match day {
        1 | 2 | 3 | 4 | 5 => println!("It's weekday."),
        6 | 7 => println!("It's weekend."),
        _ => println!("Invalid day."),
    }

//match with returned value
//match bisa juga mengembalikan value, jadi bisa disimpan ke variabel

    let day = 6;

    let day_type = match day {
        1 | 2 | 3 | 4 | 5 => "weekday",
        6 | 7 => "weekend",
        _ => "invalid day",
    };

    println!("day type: {}", day_type);


//Loops
//dipake untuk melakukan perulangan, ada 3 jenis loop di rust, yaitu loop, while, dan for. contoh



//    loop {
//        println!("This loops forever, ya know?.");
//    }

//kasih break biar ga looping terus

    let mut count = 0;

    loop {
        count += 1;
        println!("count: {}", count);

        if count == 5 {
            break;
        }
    }

//selain cuma ngeloop, bisa juga buat returned value

    let mut count = 1;
    let result = loop {
        println!("Hit!");
        if count == 5 {
            break count;
        }
        count += 1;
    };
    println!("The button was hit {} times.", result);

// while loop
// looping selama kondisi terpenuhi

    let mut count = 1;

    while count < 5 {
        println!("count: {}", count);
        count += 1;
    }

//stop while loop pake break

    let mut num = 1;

    while num < 5 {
        println!("count: {}", num);
        if num == 3 {
            break;
        }
        num += 1;
    }

//skip value pake continue

    let mut num = 1;

    while num <= 8 {
        if num == 3 {
            num += 1;
            continue;
        }
        println!("No. {}", num);
        num += 1;
    }
    println!("did i accidentally skip a number?");
    

//for loop
// dipake kalo tau jumlah perulangannya, misal mau looping dari 1 sampe 5, pake for loop aja

    for num in 1..6 {
        println!("num. {}", num);
    }

//cuma ngeprint 1 - 5, karena range nya 1..6, kalo mau sampe 6 pake 1..=6 alias inclusive range

    for num in 1..=6 {
        println!("num. {}", num);
    }

//for loop continue and break

    for num in 1..=8 {
        if num == 3 {
            continue;
        }
        if num == 6 {
            break;
        }
        println!("num. {}", num);
    }
    println!("now tell me, what number did i skip? and what number did i stop at?");

//Functions
//blok kode yang hanya bisa jalan kalo dipanggil

//basic function

/*    fn func_name() {
        println!("This is a function.");
    } */


//Cara panggil function

    fn greet() {
        println!("Hello, World!");
    }

    greet(); //panggil function greet

//function with parameters

    fn call(name: &str) {
        println!("greeting, {}!", name);
    }

    call("Aria"); //panggil function call dengan parameter "Aria"

//return value func
//pake simbol -> di function header

    fn plus(x: i32, y: i32) -> i32 {
        x + y // bisa juga ditulis  return x + y;
    }
    let result = plus(5, 10);
    println!("5 + 10 = {}", result);


////////////////////////////////////////////////////////////////////////////////


//Scope and Shadowing
/*scope itu batasan, karena variabel itu hidup di block yang didalamnya >>> {}
contoh :

#variable didalam function

    fn myfunc() {
        let x = 5; //x hidup di block ini
        println!("x = {}", x); //bisa karena dia didalam scope / {}
}
    myfunc();
    println!("x = {}", x); //error, karena x ga hidup di block ini

#variable didalam block

    let score = 5; //score hidup di block ini
    {
        let score = 10; //score hidup di block ini
        println!("score = {}", score); //bisa karena dia didalam scope / {}
    }
    println!("score = {}", score); //bisa karena dia didalam scope / {}

*/

// di rust, bisa deklarasi variable baru dengan nama sama menggunakan let. ini disebut shadowing, contoh :

    let x = 5;
    let x = 10; //shadowing
    println!("Shadowing: {}", x);// 10 yang keprint,variable pertama tidak bisa diakses karena langsung tertimpa variable kedua. karena rust tidak membolehkan nama yang sama pada setiap variable 

/*bukti 
    ||
    \/

warning: unused variable: `x`                                                                                  
   --> src\main.rs:429:9
    |
429 |     let x = 5;
    |         ^ help: if this is intentional, prefix it with an underscore: `_x`
    |
    = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default
*/

//selain itu, bisa juga dipakai didalam block

    let x = 5;
    {
        let x = 10; //shadowing
        println!("inside block: {}", x); 
    }
    println!("outside block: {}", x); 

//String
/*

ada dua cara nulis string

&str dan string 

&str dipakai kalo itu udah fix
string dipakai kalo butuh berubah ubah 

*/

//contoh

    let greet: &str="hello";

    println!("{}", greet);


// buat string pakai to_string() atau String::from() func

    let text1 ="hunt begins".to_string();
    println!("{}", text1);

    let text2 = String::from("Hunt Begins");
    println!("{}", text2);

//ubah value string
//string itu mutable, jadi bisa pake mut. 
// pake .push_str() untuk nambah text
// pakai .push() kalo cuma 1 huruf

    let mut greet = String::from("I Am Thou,");
    greet.push_str(" Thou Art I...");
    println!("{}", greet);
    
    let mut speak =String::from("Persona");
    speak.push('!');
    println!("{}", speak);    

//Concatenate Strings
//nyambungin banyak string sekaligus pakai "format!"

    let w1 = String::from("Heed");
    let w2 = String::from("My");
    let w3 = String::from("Command!");
    
    let result = format!("{} {} {}", w1, w2, w3);
    println!("{}", result);

//String length 
//nyari banyak huruf dalam 1 string

    let name = String::from("Big Boss");
    println!("banyak huruf dari nama '{}': {}", name, name.len());

//ownership
//setiap value punya owner masing masing, dan biasanya itu variable

    let x = String::from("dokkaebi");
    let y = x ;//value dipindah dari x ke y, x jadi tidak valid

    println!("{}", y);

//tapi jika yang dipakai adalah nomor simpel, maka tidak akan invalid melainkan akan melakukan copy

    let a = 6;
    let b = a;

    println!("a {}", a);//valid semua
    println!("b {}", b);//valid semua

//clone 
//bedanya ya sesuai nama cuma ngeclone ga ngilangin value lain

    let r = 4;
    let f = r.clone();

    println!("r {}", r);
    println!("f {}", f);

}
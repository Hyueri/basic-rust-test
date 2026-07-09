//ga akan bisa dirun karena ada 3 main function (yajelas lah ya, aneh bet kalo nyoba dirun)

//basic / awalan rust isiny gini
fn main() {
    println!("Hello, world!");



//pake println! macro munculin line kebawah

    println!("ohayo, sekai!");
    println!("test again!");
    println!("try better next time!");


//pake print! macro munculin line ke samping
//kalo mau line kebawah harus pake \n

    print!("ohayo, sekai!");
    print!("test again!\n");
    print!("try better next time!\n");



///////////////////////////////////////////////////////////////////////

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


//////////////////////////////////////////////////////////////////////

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

}
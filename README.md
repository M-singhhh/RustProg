# Rust Programming 
//genrally there are two ways we can control heap memory 
1. garbage collector(compiler manages memory auto )

2.  manually allocate or deallocate memory (has security and bug issues) rust has diffrent approch for managing memory there is part in rust's compiler called borrow checker which implements 2 rules:-
i.) ownership ii.) borrowing

i--ownership :- 

1.each value in rust has an owner 

2. there can be only owner at a time 

                  ex - fn main (){
                         let s1 = string::from("hellow !!");
                         let s2 = s1 ;
                         println!("{s1}") //error borrow of s1 has moved to s2 
                         }
                         # If we want to print s1 we either can print s2 or can give s2 = s1.clone() ;}this will give ownership of clone of s1 not of s1  

  3. when the owner goes out of scope the value will be dropped 
  //these rules also applies to function parameter passing
                
                ex- fn main (){
                         let s1 = string::from("hellow !!");
                         let s2 = s1 ;
                         printo(s2) //when printo is called the ownership of s2 is passed to str 
                         printo(s2) //error s2 doesnt have ownership of value 
                          }

                    fn printo(str :: string ){
                      println({"str"});
                      }//from str goes out of scope so value from str is dropped so nor s1 has value neither s2 or str from here on 
                  
#the good practice is to give refrence as a prameter to the fucntion by usage of & operator :-
                  
                      fn main (){
                         let s1 = string::from("hellow !!");
                         let s2 = s1 ;
                         printo(&s2) //we will pass the refrence of s2 to the function not the owner ship just like the concept of pointers in c 
                         printo(&s2)
                          }

                    fn printo(str :: &string ){ //refrence value is borrowed by str and str borrowed refrenced ownership will drop after scope   
                      println({"str"});
                      }//ownership return to original owner from here on 
#this technique is called borrowing it is similar to real life borrowing when passed a refrence to variable it will temporary borrow value and ownership of value it has 2 rules :-

1.at any given time you can have 1 mutable refrence or any number of immutable refrence and at defult any refrence is immutable cause you cant edit and read at same time cause 

B reads score = 10
A changes score = 50
B continues calculation assuming 10 //Program bug — called data race and rust frobids it 

  case 1 :
  
                    let s1 = string::from("hellow !!");
                    let r1 = &s1 ; //immutable refrence we can have any number of these 
 case 2 : 
 
                    let mut s2 = string::from("hellow !!");
                    let r2 = &s2 ;{reading }
                    let r3 = &mut s2 ; {editing}  // Compiler says NO. Because: Someone is reading while someone might modify. 
case 3 :

                    let mut x = 5;
                    let r1 = &mut x;
                    let r2 = &mut x; // two people cant edit at the same time thats why only one mutable refrence 
case 4 : 

                    let mut x = 5;
                    {
                    let r1 = &x;
                    println!("{}", r1);
                    } // r1 dies here
                    let r2 = &mut x; this is totally alowed cause reader died because of it is out of scope now therfore we can use x again as a mutable refrence 
                    
                  
//Python is Dynamically Linked. When you run a script, it doesn't "own" the math logic; it just asks the Python Interpreter (already installed on your OS) to do it.
Rust is Statically Linked by default. It pulls the entire LLVM backend, your ndarray dependency, and the std library into a single Self-Contained Executable.
and rust also maps whole bunch of extra meta data so that easy for debugging for the user so the rust code when implemented may bloat cause it prioritizes your safety and speed of development over file size. so we use cargo build --release to shrink all the code and builds into single optimized build it tells the compiler to act as optimizer rather than helpping at debugging 

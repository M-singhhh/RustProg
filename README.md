# Rust Programming 
//Python is Dynamically Linked. When you run a script, it doesn't "own" the math logic; it just asks the Python Interpreter (already installed on your OS) to do it.
Rust is Statically Linked by default. It pulls the entire LLVM backend, your ndarray dependency, and the std library into a single Self-Contained Executable.
and rust also maps whole bunch of extra meta data so that easy for debugging for the user so the rust code when implemented may bloat cause it prioritizes your safety and speed of development over file size. so we use cargo build --release to shrink all the code and builds into single optimized build it tells the compiler to act as optimizer rather than helpping at debugging 

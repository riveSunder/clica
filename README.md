# Command Line Interface Cellular Automata (CLICA)

A naive implementation of elementary CA in Rust


```
                                                @                                                
                                               @ @                                               
                                              @   @                                              
                                             @ @ @ @                                             
                                            @       @                                            
                                           @ @     @ @                                           
                                          @   @   @   @                                          
                                         @ @ @ @ @ @ @ @                                         
                                        @               @                                        
                                       @ @             @ @                                       
                                      @   @           @   @                                      
                                     @ @ @ @         @ @ @ @                                     
                                    @       @       @       @                                    
                                   @ @     @ @     @ @     @ @                                   
                                  @   @   @   @   @   @   @   @                                  
                                 @ @ @ @ @ @ @ @ @ @ @ @ @ @ @ @                                 
                                @                               @                                
                               @ @                             @ @                               
                              @   @                           @   @                              
                             @ @ @ @                         @ @ @ @                             
                            @       @                       @       @                            
                           @ @     @ @                     @ @     @ @                           
                          @   @   @   @                   @   @   @   @                          
                         @ @ @ @ @ @ @ @                 @ @ @ @ @ @ @ @                         
                        @               @               @               @                        
                       @ @             @ @             @ @             @ @                       
                      @   @           @   @           @   @           @   @                      
                     @ @ @ @         @ @ @ @         @ @ @ @         @ @ @ @                     
                    @       @       @       @       @       @       @       @                    
                   @ @     @ @     @ @     @ @     @ @     @ @     @ @     @ @                   
                  @   @   @   @   @   @   @   @   @   @   @   @   @   @   @   @                  
                 @ @ @ @ @ @ @ @ @ @ @ @ @ @ @ @ @ @ @ @ @ @ @ @ @ @ @ @ @ @ @ @                 
# rule 90
```

## Getting started

If you haven't previously set up Rust and its dependencies, check out [the Rust book](https://doc.rust-lang.org/book/ch01-01-installation.html).

Clone the repo:

```
git clone git@github.com:riveSunder/clica.git
cd clica
```

build and run (debug)

```
Cargo build
./target/debug/clica -d 71 -r 110 -s 48 -o output.txt
```

or build and run (release)

```
Cargo build --release
# run the CA simulator
./target/release/clica -d 71 -r 110 -s 48 -o output.txt
```

```
# output.txt

                                                @                                                
                                               @@                                                
                                              @@@                                                
                                             @@ @                                                
                                            @@@@@                                                
                                           @@   @                                                
                                          @@@  @@                                                
                                         @@ @ @@@                                                
                                        @@@@@@@ @                                                
                                       @@     @@@                                                
                                      @@@    @@ @                                                
                                     @@ @   @@@@@                                                
                                    @@@@@  @@   @                                                
                                   @@   @ @@@  @@                                                
                                  @@@  @@@@ @ @@@                                                
                                 @@ @ @@  @@@@@ @                                                
                                @@@@@@@@ @@   @@@                                                
                               @@      @@@@  @@ @                                                
                              @@@     @@  @ @@@@@                                                
                             @@ @    @@@ @@@@   @                                                
                            @@@@@   @@ @@@  @  @@                                                
                           @@   @  @@@@@ @ @@ @@@                                                
                          @@@  @@ @@   @@@@@@@@ @                                                
                         @@ @ @@@@@@  @@      @@@                                                
                        @@@@@@@    @ @@@     @@ @                                                
                       @@     @   @@@@ @    @@@@@                                                
                      @@@    @@  @@  @@@   @@   @                                                
                     @@ @   @@@ @@@ @@ @  @@@  @@                                                
                    @@@@@  @@ @@@ @@@@@@ @@ @ @@@                                                
                   @@   @ @@@@@ @@@    @@@@@@@@ @                                                
                  @@@  @@@@   @@@ @   @@      @@@                                                
                 @@ @ @@  @  @@ @@@  @@@     @@ @                                                
                @@@@@@@@ @@ @@@@@ @ @@ @    @@@@@                                                
               @@      @@@@@@   @@@@@@@@   @@   @                                                
              @@@     @@    @  @@      @  @@@  @@                                                
             @@ @    @@@   @@ @@@     @@ @@ @ @@@                                                
            @@@@@   @@ @  @@@@@ @    @@@@@@@@@@ @                                                
           @@   @  @@@@@ @@   @@@   @@        @@@                                                
          @@@  @@ @@   @@@@  @@ @  @@@       @@ @                                                
         @@ @ @@@@@@  @@  @ @@@@@ @@ @      @@@@@                                                
        @@@@@@@    @ @@@ @@@@   @@@@@@     @@   @                                                
       @@     @   @@@@ @@@  @  @@    @    @@@  @@                                                
      @@@    @@  @@  @@@ @ @@ @@@   @@   @@ @ @@@                                                
     @@ @   @@@ @@@ @@ @@@@@@@@ @  @@@  @@@@@@@ @                                                
    @@@@@  @@ @@@ @@@@@@      @@@ @@ @ @@     @@@                                                
   @@   @ @@@@@ @@@    @     @@ @@@@@@@@@    @@ @                                                
  @@@  @@@@   @@@ @   @@    @@@@@       @   @@@@@                                                
 @@ @ @@  @  @@ @@@  @@@   @@   @      @@  @@   @                                                
```

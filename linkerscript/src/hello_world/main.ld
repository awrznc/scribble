TARGET("binary");
INPUT("./hello.txt");
INPUT("./world.txt");
OUTPUT("hello_world.txt");

SECTIONS {
    . : {
        ./hello.txt;
        BYTE(0x2c);
        BYTE(0x20);
        ./world.txt;
        BYTE(0x21);
    }
}

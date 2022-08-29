/**
  * ld -T ./bmp.ld /dev/null -o image.bmp
  **/

TARGET("binary");

SECTIONS {
    .file_header : {
        BYTE(0x42);
        BYTE(0x4D);

        LONG(SIZEOF(.file_header) + SIZEOF(.info_header) + SIZEOF(.data));
        SHORT(0x0);
        SHORT(0x0);

        LONG(SIZEOF(.file_header));
    }

    .info_header : {
        LONG(40);
        LONG(1);
        LONG(1);
        SHORT(1);
        SHORT(8);

        LONG(0);
        LONG(SIZEOF(.data));
        LONG(0);
        LONG(0);
        LONG(0);
        LONG(0);
    }

    .data : {
        BYTE(0xFF);
        BYTE(0xFF);
        BYTE(0xFF);
        BYTE(0x0);
    }
}

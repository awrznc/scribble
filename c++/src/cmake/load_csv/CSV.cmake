# csv(
#   GET <out-var>
#   READ <filename>
#   INDEX <column-index>
#   [
#     HEADER <boolean>
#     SEQUENCE <sequence-char>
#   ]
# )

function( csv )
  cmake_parse_arguments(CSV "" "GET;READ;INDEX" "HEADER;SEQUENCE" ${ARGN})

  set( SEQUENCE "," )
  if( CSV_SEQUENCE )
    set( SEQUENCE ${CSV_SEQUENCE} )
  endif()

  file( READ ${CSV_READ} CSV_STRING )
  string( REPLACE "\n" ";" CSV_LINES ${CSV_STRING} )

  if( CSV_HEADER )
    list(REMOVE_AT CSV_LINES 0)
  endif()

  foreach( CSV_LINE ${CSV_LINES} )
    string( REPLACE ${SEQUENCE} ";" CSV_LINE_ELEM ${CSV_LINE} )
    list( GET CSV_LINE_ELEM ${CSV_INDEX} TARGET_STRING)
    list( APPEND TARGET_STRINGS ${TARGET_STRING} )
  endforeach( CSV_LINE )

  set(${CSV_GET} ${TARGET_STRINGS} PARENT_SCOPE)
endfunction( csv )

cmake_minimum_required(VERSION 3.10)
project(read_csv)

include( ${CMAKE_CURRENT_SOURCE_DIR}/CSV.cmake )

csv(
  # requires
  GET RESULTS
  READ ${CMAKE_CURRENT_SOURCE_DIR}/table.csv
  INDEX 1

  # options
  HEADER true
  SEQUENCE ","
)

# check
foreach( CSV_FILE ${RESULTS} )
  message( target: ${CSV_FILE} )
endforeach( CSV_FILE )
# =>
# target:a.txt
# target:b.txt

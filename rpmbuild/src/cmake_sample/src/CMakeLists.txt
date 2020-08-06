# CMakeのバージョン指定
cmake_minimum_required(VERSION 3.1)

# プロジェクトの設定
project(test_cmake C)

# sample.cをコンパイルして共有ライブラリlibsample.soを作成
add_library(sample SHARED sample/src/sample.c)

# 共有ライブラリのリンカを作成しないようにする
set(CMAKE_SKIP_RPATH TRUE)

# 実行ファイルの作成
add_executable(sample_app main.c)

# sample_appを作成する際にlibsample.soをリンク
target_link_libraries(sample_app sample)

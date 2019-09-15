# CMakeのバージョン指定
cmake_minimum_required(VERSION 3.1)

# プロジェクトの設定
project(hello_world CXX)

# ライブラリの取り込み
add_subdirectory(./hoge)

# 実行ファイルの作成
add_executable(main_app main.cpp)

# hoge ライブラリを main_app にリンク
target_link_libraries(main_app hoge)

# CMakeのバージョン指定
cmake_minimum_required(VERSION 3.1)

# プロジェクトの設定
project(sample_lib C)

# ライブラリの作成
add_library(sample SHARED ./src/sample.c)

# インクルードディレクトリの指定
target_include_directories(sample INTERFACE ./include)

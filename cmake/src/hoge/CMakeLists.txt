# CMakeのバージョン指定
cmake_minimum_required(VERSION 3.1)

# プロジェクトの設定
project(hoge_lib
  VERSION 1.0.0
  DESCRIPTION "Hoge library"
  HOMEPAGE_URL "https://example.com"
  LANGUAGES CXX)

# ライブラリの作成
add_library(hoge STATIC ./src/hoge.cpp)

# ビルドプロパティの指定（c++11規格で実行する）
target_compile_features(hoge PRIVATE cxx_std_11)

# インクルードディレクトリの指定
target_include_directories(hoge INTERFACE ./include)

# ライブラリのプロパティ設定
set_target_properties(hoge
  PROPERTIES
  VERSION ${PROJECT_VERSION})

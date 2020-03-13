# CMakeのバージョン指定
cmake_minimum_required(VERSION 3.1)

# プロジェクトの設定
project(sample C)

# ライブラリの取り込み
add_subdirectory(./sample)

# 追加
# link_directories(./)
set(CMAKE_SKIP_RPATH TRUE)

# 実行ファイルの作成
add_executable(sample_app main.c)

# sample ライブラリを sample_app にリンク
target_link_libraries(sample_app sample)

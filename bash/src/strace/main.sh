
# Debianにはプリインストールされていないためインストールする必要あり
# apt install -y strace

# システムコールの確認
strace curl localhost:8080

# ネットワーク関連のものに絞り込むことも可能
strace -e network curl localhost:8080
# 例えば、Ruby の Sinatra などを strace すると bind などの情報を確認することができる

# システムコールの回数や時間がわかる
strace -c curl localhost:8080

# echo の情報を表示してみる
strace -c echo hello         
# hello
# % time     seconds  usecs/call     calls    errors syscall
# ------ ----------- ----------- --------- --------- ----------------
#  27.23    0.000562          80         7           mmap
#  11.48    0.000237          59         4           mprotect
#  10.80    0.000223          55         4           close
#  10.47    0.000216         108         2           openat
#   8.58    0.000177          59         3           brk
#   7.80    0.000161          53         3           fstat
#   7.56    0.000156         156         1           write
#   5.47    0.000113         113         1           execve
#   3.00    0.000062          62         1         1 access
#   2.96    0.000061          61         1           munmap
#   2.52    0.000052          52         1           read
#   2.13    0.000044          44         1           arch_prctl
# ------ ----------- ----------- --------- --------- ----------------
# 100.00    0.002064                    29         1 total

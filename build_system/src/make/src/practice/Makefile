# === 変数 ===

hello_string = 'Hello'
world_string = 'World'
ninja_string = 'Ninja'

# === ルール ===

hello.txt:
	bash -c "echo $(hello_string), $(world_string)! >& ./$@"

hello2.txt:
	bash -c "echo $(hello_string), $(ninja_string)! >& ./$@"

all: hello.txt hello2.txt
	cat hello.txt hello2.txt


# === デフォルトターゲット ===

.DEFAULT_GOAL := all

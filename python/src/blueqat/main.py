from blueqat import Circuit

# 初期化.ゲート1.ゲート2.run(shots=測定回数)
print( Circuit().x[0].m[0].run(shots=100) )

print( Circuit().y[0].m[0].run(shots=100) )
print( Circuit().z[0].m[0].run(shots=100) )
print( Circuit().h[0].m[0].run(shots=100) )

# 測定前の量子状態
print( Circuit().h[0].run() )

# 測定後の量子状態
print( Circuit().h[0].m[0].run() )

# CXゲート
print( Circuit().x[0].cx[0,1].m[:].run(shots=100) )
print( Circuit().x[0,1].cx[0,1].m[:].run(shots=100) )

# 変数に代入できる
c = Circuit()
c.h[0].x[0]

# 状態
print( c.to_unitary() )

# 2量子ビットの重ね合わせ
# 量子ビット 1 --H--m--
# 量子ビット 2 --H--m--
print( Circuit().h[0,1].m[:].run(shots=100) )

# 量子もつれ
# 量子ビット 1 --H-+-m--
# 量子ビット 2 ----+-m--
print( Circuit().h[0].cx[0,1].m[:].run(shots=100) )

# 足し算（左読み）
c = Circuit()
print( c.h[0,1].cx[0,2].cx[1,2].ccx[0,1,3].m[:].run(shots=100) )

# トフォリゲート
## (0,0)の場合
c = Circuit()
c.h[2].cx[1,2].tdg[2].cx[0,2].t[2].cx[1,2].tdg[2].cx[0,2]
c.t[1,2].h[2].cx[0,1].t[0].tdg[1].cx[0,1].m[:]
print( c.run(shots=100) )

## (1,0)の場合
c = Circuit()
c.x[1]
c.h[2].cx[1,2].tdg[2].cx[0,2].t[2].cx[1,2].tdg[2].cx[0,2]
c.t[1,2].h[2].cx[0,1].t[0].tdg[1].cx[0,1].m[:]
print( c.run(shots=100) )

## (0,1)の場合
c = Circuit()
c.x[0]
c.h[2].cx[1,2].tdg[2].cx[0,2].t[2].cx[1,2].tdg[2].cx[0,2]
c.t[1,2].h[2].cx[0,1].t[0].tdg[1].cx[0,1].m[:]
print( c.run(shots=100) )

## (1,1)の場合（ターゲットビット0->1）
c = Circuit()
c.x[0,1]
c.h[2].cx[1,2].tdg[2].cx[0,2].t[2].cx[1,2].tdg[2].cx[0,2]
c.t[1,2].h[2].cx[0,1].t[0].tdg[1].cx[0,1].m[:]
print( c.run(shots=100) )

## (1,1)の場合（ターゲットビット1->0）
c = Circuit()
c.x[:]
c.h[2].cx[1,2].tdg[2].cx[0,2].t[2].cx[1,2].tdg[2].cx[0,2]
c.t[1,2].h[2].cx[0,1].t[0].tdg[1].cx[0,1].m[:]
print( c.run(shots=100) )

## 重ね合わせ
c = Circuit()
c.h[0,1]
c.h[2].cx[1,2].tdg[2].cx[0,2].t[2].cx[1,2].tdg[2].cx[0,2]
c.t[1,2].h[2].cx[0,1].t[0].tdg[1].cx[0,1].m[:]
print( c.run(shots=100) )

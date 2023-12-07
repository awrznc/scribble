! コメントは `!` を接頭辞に付ける

! サブモジュール例
subroutine increment(a)
  ! 暗黙の型宣言禁止は以下の構文で可能になる
  implicit none

  integer a
  a = a + 1
end subroutine increment

! 関数例
integer function return_zero
  return_zero = 0
  return
end function return_zero

! モジュール例
module color
  private
    integer :: red = 0
    integer :: green = 0
    integer :: blue = 0
  public print_color, init_color
  contains
    ! 初期化
    subroutine init_color(r, g, b)
      integer r
      integer g
      integer b
      red = r
      green = g
      blue = b
    end subroutine init_color

    ! 出力
    subroutine print_color
      print '("red: ", i0, ", green: ", i0, ", blue: ", i0)', red, green, blue
    end subroutine print_color
end module color

! Fortran には「宣言セクション」と「実行セクション」がある。
! 「実行セクション」の後に「宣言セクション」を入れることはできない。
! 違反している場合はコンパイラに怒られる。
program main
  ! ~~~ 宣言セクション ~~~

  ! `color` モジュールの利用
  use color

  ! 変数宣言
  integer a1
  real n1

  ! ~~~ 実行セクション ~~~

  ! 代入
  a1 = 1
  n1 = 0.1

  ! 出力
  print *, 'Hello, Fortran!'
  ! =>  Hello, Fortran!
  print '("a1=", i0, ", n1=", f5.3)', a1, n1
  ! =>  a1=1, n1=0.100

  ! TODO: 四則演算
  a1 = 2
  print '("a1=", i0)', a1

  ! 条件分岐
  !   .LT. <  ( Less Than )
  !   .LE. <= ( Less than or Equal to )
  !   .EQ. == ( EQual to )
  !   .NE. != ( Not Equal to )
  !   .GT. >  ( Greater Than )
  !   .GE. >= ( Greater than or Equal to )
  if ( a1 .EQ. 12 ) then
    print *, 'a1 == 12'
  else if ( a1 .EQ. 42 ) then
    print *, 'a1 == 42'
  else
    print *, 'a1 != 12 && a1 != 42'
    ! => a1 != 12 && a1 != 42
  endif

  ! 繰り返し
  doi: do i=0,2
    print '("i=", i0)', i
  enddo doi
  ! =>
  ! i=0
  ! i=1
  ! i=2

  ! === サブプログラム ===

  ! サブルーチン（値は返さない）
  a1 = 2
  call increment(a1)
  print '("a1=", i0)', a1
  ! => a1=3

  ! 関数
  print '("return_zero()=", i0)', return_zero()
  ! => return_zero()=0

  ! モジュール
  call init_color(255, 127, 0)
  call print_color()
  ! => red: 255, green: 127, blue: 0
end program main

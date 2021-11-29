-- 注意: PostgreSQL前提のコメント

-- Databaseの作成 ----------------------------------------------------------------
-- Databaseを切り分けたい場合は、以下のコマンドで新規にDatabaseを作成することができる。
-- ただし、ソースコード上でDatabaseを切り替えることはできないようなので、ここは手作業で行うこと。
-- CREATE DATABASE practice_db;

-- テーブルの作成 --------------------------------------------------------------------
CREATE TABLE practice_table(
    id          serial primary key,

    -- PostgreSQLではtextのデフォルト値を設定できるが、MySQLでは設定できない
    title       text,
    price       int default 0,
    create_at   timestamp default CURRENT_TIMESTAMP,
    update_at   timestamp default CURRENT_TIMESTAMP
);


-- データの作成 ---------------------------------------------------------------------
INSERT INTO practice_table( title, price ) VALUES( 'Apple', 100 );
INSERT INTO practice_table( title, price ) VALUES( 'Banana', 80 );
INSERT INTO practice_table( title, price ) VALUES( 'Cake', 1200 );
-- => INSERT 0 1
-- [INSERT 0] までが1つ以上の行が挿入された場合に返されるメッセージ。
-- [1] は追加した行数を表す。


-- データの参照 ---------------------------------------------------------------------
SELECT * FROM practice_table;

    -- 表示を絞る --

        -- idのみ表示
        SELECT id FROM practice_table;

    -- 条件文 --

        -- [priceの値が90よりも大きいデータ]かつ[titleの値が'Cake'であるデータ]のみ抽出
        SELECT * FROM practice_table WHERE price > 90 AND title = 'Cake';

        -- 直近1時間以内に更新したデータを抽出
        SELECT * FROM practice_table WHERE update_at >= (NOW() - INTERVAL '1 HOUR');

        -- MySQLの場合は以下のように記述しないとエラーになる
        -- SELECT * FROM practice_table WHERE update_at >= (NOW() - INTERVAL 1 HOUR);


    -- ソート --

        -- 昇順（小さい方が上）
        SELECT * FROM practice_table ORDER BY id ASC;

        -- 降順（大きい方が上）
        SELECT * FROM practice_table ORDER BY id DESC;


-- データの更新 ---------------------------------------------------------------------
UPDATE practice_table SET price = 50, update_at = CURRENT_TIMESTAMP WHERE title = 'Apple';
-- => UPDATE 1


-- データの削除 ---------------------------------------------------------------------
DELETE FROM practice_table WHERE title = 'Banana';
-- => DELETE 1


-- テーブルの結合 --------------------------------------------------------------------

    -- 別のDatabaseの作成 --
    CREATE TABLE sample_table(
        id              serial primary key,
        title           text,
        release_date    timestamp default '2000-01-01 00:00:00',
        create_at       timestamp default CURRENT_TIMESTAMP,
        update_at       timestamp default CURRENT_TIMESTAMP
    );

    -- データを作成 --
    INSERT INTO sample_table( title, release_date ) VALUES( 'Apple',  '2021-12-01 12:00:00' );
    INSERT INTO sample_table( title, release_date ) VALUES( 'Banana', '2021-11-25 15:10:00' );
    INSERT INTO sample_table( title, release_date ) VALUES( 'Cake',   '2021-12-05 07:00:10' );

    -- 結合 --

        -- practice_table と sample_table を結合して参照
        SELECT practice_table.title, price, release_date
            FROM practice_table, sample_table
            WHERE practice_table.title=sample_table.title;
        -- =>
        -- title  | price |    release_date     
        -- --------+-------+---------------------
        -- Apple  |   100 | 2021-12-01 12:00:00
        -- Banana |    80 | 2021-11-25 15:10:00
        -- Cake   |  1200 | 2021-12-05 07:00:10
        -- (3 rows)


-- テーブルの削除 --------------------------------------------------------------------
DROP TABLE practice_table;
DROP TABLE sample_table;


-- Databaseの削除 ----------------------------------------------------------------
-- DROP DATABASE practice_db;

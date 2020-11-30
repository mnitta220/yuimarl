DROP TABLE IF EXISTS YM_EVENTVIEW_SPREAD;
DROP TABLE IF EXISTS YM_EVENT;
DROP TABLE IF EXISTS YM_EVENTVIEWSET;
DROP TABLE IF EXISTS YM_EVENTSET;
DROP TABLE IF EXISTS YM_EVENTVIEW;
DROP TABLE IF EXISTS YM_PARTY_RELATION;
DROP TABLE IF EXISTS YM_PARTY;
DROP TABLE IF EXISTS YM_GOODS_CATEGORY;
DROP TABLE IF EXISTS YM_ORG_CATEGORY;
DROP TABLE IF EXISTS YM_PREFECTURE;
DROP TABLE IF EXISTS YM_USER;
DROP TABLE IF EXISTS YM_GENERATED_ID;

-- ジェネレートIDテーブル
CREATE TABLE YM_GENERATED_ID (
    KEY_NAME            VARCHAR(30) PRIMARY KEY,    -- キー
    VALUE               INTEGER     NOT NULL        -- 値
);

-- YUIMARLユーザーテーブル
CREATE TABLE YM_USER (
    USER_NO             INTEGER PRIMARY KEY,        -- User No.
    USER_ID             VARCHAR(10),                -- ユーザーID
    PASSWORD            VARCHAR(128) NOT NULL,      -- パスワード
    PARTY               INTEGER,                    -- Party
    ACCOUNT_GROUP       VARCHAR(30),                -- アカウントグループ
    DEL_FLG             CHAR(1) DEFAULT '0',        -- 削除フラグ（1:削除済み）
    REGIST_USER         INTEGER,                    -- 登録ユーザー(User No.)
    REGIST_TIME         TIMESTAMP,                  -- 登録日時
    UPDATE_USER         INTEGER,                    -- 更新ユーザー(User No.)
    UPDATE_TIME         TIMESTAMP,                  -- 更新日時
    AUTH_MAP            VARCHAR(30),                -- 権限マップ
    AUTH_REGIST_USER    INTEGER,                    -- 権限登録ユーザー(User No.)
    AUTH_REGIST_TIME    TIMESTAMP,                  -- 権限登録日時
    AUTH_UPDATE_USER    INTEGER,                    -- 権限更新ユーザー(User No.)
    AUTH_UPDATE_TIME    TIMESTAMP,                  -- 権限更新日時
    VERSION_NO          INTEGER                     -- バージョン番号
);

-- 都道府県テーブル
CREATE TABLE YM_PREFECTURE (
    PREFECTURE_NO       INTEGER PRIMARY KEY,        -- 都道府県No.
    NAME                VARCHAR(10),                -- 名称
    DEL_FLG             CHAR(1) DEFAULT '0'         -- 削除フラグ（1:削除済み）
);

-- 組織種類テーブル
CREATE TABLE YM_ORG_CATEGORY (
    CATEGORY_NO         INTEGER PRIMARY KEY,        -- カテゴリNo.
    NAME                VARCHAR(50) NOT NULL,       -- 名前
    PARTY_TYPE          CHAR(1) NOT NULL,           -- Party種別 1:人, 2:組織, 3:法人, 4:物品
    DEL_FLG             CHAR(1) DEFAULT '0'         -- 削除フラグ（1:削除済み）
);

-- 物品種類テーブル
CREATE TABLE YM_GOODS_CATEGORY (
    CATEGORY_NO         INTEGER PRIMARY KEY,        -- カテゴリNo.
    NAME                VARCHAR(50),                -- 名前
    NAME_KANA           VARCHAR(50),                -- 名前ヨミ
    PARENT_CATEGORY     INTEGER,                    -- 親カテゴリNo.
    DEL_FLG             CHAR(1) DEFAULT '0',        -- 削除フラグ（1:削除済み）
    REGIST_USER         INTEGER,                    -- 登録ユーザー(User No.)
    REGIST_TIME         TIMESTAMP,                  -- 登録日時
    UPDATE_USER         INTEGER,                    -- 更新ユーザー(User No.)
    UPDATE_TIME         TIMESTAMP,                  -- 更新日時
    VERSION_NO          INTEGER                     -- バージョン番号
);

-- PARTYテーブル
CREATE TABLE YM_PARTY (
    PARTY_NO            INTEGER PRIMARY KEY,        -- Party No.
    PARTY_TYPE          CHAR(1) NOT NULL,           -- Party種別 1:人, 2:組織, 3:法人, 4:物品
    NAME                VARCHAR(50),                -- 名前
    NAME_KANA           VARCHAR(50),                -- 名前ヨミ
    ZIP_CODE            VARCHAR(10),                -- 郵便番号
    PREFECTURE          INTEGER,                    -- 都道府県
    CITY                VARCHAR(10),                -- 市町村
    ADDRESS1            VARCHAR(50),                -- 住所1
    ADDRESS2            VARCHAR(50),                -- 住所2
    PHONE_NO            VARCHAR(20),                -- 電話番号
    FAX_NO              VARCHAR(20),                -- FAX番号
    MAIL_ADDRESS1       VARCHAR(50),                -- メールアドレス1
    MAIL_ADDRESS2       VARCHAR(50),                -- メールアドレス2
    START_DATE          DATE,                       -- 開始日
    END_DATE            DATE,                       -- 終了日
    MEMO                VARCHAR(4096),              -- メモ
    LAST_NAME           VARCHAR(20),                -- 姓
    FIRST_NAME          VARCHAR(20),                -- 名
    LAST_NAME_KANA      VARCHAR(30),                -- 姓ヨミ
    FIRST_NAME_KANA     VARCHAR(30),                -- 名ヨミ
    GENDER              INTEGER,                    -- 性別
    CELL_PHONE_NUMBER   VARCHAR(20),                -- 携帯電話番号
    CATEGORY            INTEGER,                    -- 組織種類
    PERSON_COUNT        INTEGER,                    -- 所属者数
    URL                 VARCHAR(60),                -- URL
    CAPITAL             BIGINT,                     -- 資本金
    ACCOUNT_MONTH       INTEGER,                    -- 決算月
    PARTNER_FLG         CHAR(1) DEFAULT '0',        -- 取引先フラグ（1:取引先）
    BANK                VARCHAR(40),                -- 銀行
    BRANCH              VARCHAR(40),                -- 支店
    ACCOUNT_TYPE        CHAR(1),                    -- 口座種別(1:普通, 2:当座)
    ACCOUNT             VARCHAR(40),                -- 口座
    CUTOFF              INTEGER,                    -- 締日(31:月末)
    CATEGORY_NO         INTEGER,                    -- 物品種類
    DEL_FLG             CHAR(1) DEFAULT '0',        -- 削除フラグ（1:削除済み）
    REGIST_USER         INTEGER,                    -- 登録ユーザー(User No.)
    REGIST_TIME         TIMESTAMP,                  -- 登録日時
    UPDATE_USER         INTEGER,                    -- 更新ユーザー(User No.)
    UPDATE_TIME         TIMESTAMP,                  -- 更新日時
    VERSION_NO          INTEGER                     -- バージョン番号
);

-- PARTY関連テーブル
CREATE TABLE YM_PARTY_RELATION (
    RELATION_NO         INTEGER PRIMARY KEY,        -- Party関連No.
    RELATION_TYPE       INTEGER,                    -- Party関連種別
    PARTY1              INTEGER,                    -- Party1(Party No.)
    ROLE1               VARCHAR(30),                -- 役割1
    PARTY2              INTEGER,                    -- Party2(Party No.)
    ROLE2               VARCHAR(30),                -- 役割2
    TERM_FROM           DATE,                       -- 開始日
    TERM_TO             DATE,                       -- 終了日
    REGIST_USER         INTEGER,                    -- 登録ユーザー(User No.)
    REGIST_TIME         TIMESTAMP,                  -- 登録日時
    UPDATE_USER         INTEGER,                    -- 更新ユーザー(User No.)
    UPDATE_TIME         TIMESTAMP,                  -- 更新日時
    VERSION_NO          INTEGER                     -- バージョン番号
);

-- イベントビューテーブル
CREATE TABLE YM_EVENTVIEW (
    EVENTVIEW_NO        INTEGER PRIMARY KEY,        -- イベントビューNo.
    OWNER               INTEGER,                    -- オーナー(User No.)
    NAME                VARCHAR(100),               -- 名前
    SEQ                 INTEGER,                    -- 表示順
    JAPANESE_HOLIDAY    CHAR(1) DEFAULT '0',        -- 日本の休日（1:日本の休日を使用する）
    SPREAD_YEARS        VARCHAR(4096),              -- 展開年(yyyy,yyyy,・・・)
    DEL_FLG             CHAR(1) DEFAULT '0',        -- 削除フラグ（1:削除済み）
    REGIST_USER         INTEGER,                    -- 登録ユーザー(User No.)
    REGIST_TIME         TIMESTAMP,                  -- 登録日時
    UPDATE_USER         INTEGER,                    -- 更新ユーザー(User No.)
    UPDATE_TIME         TIMESTAMP,                  -- 更新日時
    VERSION_NO          INTEGER                     -- バージョン番号
);

-- イベントセットテーブル
CREATE TABLE YM_EVENTSET (
    EVENTSET_NO         INTEGER PRIMARY KEY,        -- イベントセットNo.（1:日本の休日・記念日）
    EVENTSET_TYPE       CHAR(1) DEFAULT '0',        -- イベントセット型 1:プロジェクト
    NAME                VARCHAR(100),               -- 名前
    TERM_FROM           DATE,                       -- 期間開始日
    TERM_TO             DATE,                       -- 期間終了日
    CLIENT              INTEGER,                    -- クライアント(Party No.)
    PRODUCER            INTEGER,                    -- プロデューサー(Party No.)
    SUNDAY              CHAR(1) DEFAULT '1',        -- 日曜日（1:実施, 0:実施しない）
    SATURDAY            CHAR(1) DEFAULT '1',        -- 土曜日（1:実施, 0:実施しない）
    HOLIDAY             CHAR(1) DEFAULT '1',        -- 祝日（1:実施, 0:実施しない）
    INCLUDE_DAYS        VARCHAR(2048),              -- 稼動日のリスト(yyyyMMdd,yyyyMMdd,・・・)
    EXCLUDE_DAYS        VARCHAR(2048),              -- 非稼動日のリスト(yyyyMMdd,yyyyMMdd,・・・)
    MEMO                VARCHAR(4096),              -- メモ
    UPDATE_AUTH         VARCHAR(2048),              -- 更新権限ユーザーのリスト(User No.,User No.,・・・)
    OWNER               INTEGER,                    -- オーナー(User No.)
    DEL_FLG             CHAR(1) DEFAULT '0',        -- 削除フラグ（1:削除済み）
    REGIST_USER         INTEGER,                    -- 登録ユーザー(User No.)
    REGIST_TIME         TIMESTAMP,                  -- 登録日時
    UPDATE_USER         INTEGER,                    -- 更新ユーザー(User No.)
    UPDATE_TIME         TIMESTAMP,                  -- 更新日時
    VERSION_NO          INTEGER                     -- バージョン番号
);

-- イベントビューセットテーブル
CREATE TABLE YM_EVENTVIEWSET (
    EVENTVIEW           INTEGER,                    -- イベントビューNo.
    EVENTSET            INTEGER,                    -- イベントセットNo.
    AUTH_OWNER          CHAR(1) DEFAULT '0',        -- オーナー権限（1:オーナー, 0:オーナーでない）
    AUTH_UPDATE         CHAR(1) DEFAULT '0',        -- 更新権限（1:更新可, 0:更新不可）
    REGIST_USER         INTEGER,                    -- 登録ユーザー(User No.)
    REGIST_TIME         TIMESTAMP,                  -- 登録日時
    UPDATE_USER         INTEGER,                    -- 更新ユーザー(User No.)
    UPDATE_TIME         TIMESTAMP,                  -- 更新日時
    VERSION_NO          INTEGER,                    -- バージョン番号
    PRIMARY KEY(EVENTVIEW, EVENTSET)
);

-- イベントテーブル
CREATE TABLE YM_EVENT (
    EVENT_NO            INTEGER PRIMARY KEY,        -- イベントNo.
    EVENTSET_NO         INTEGER,                    -- イベントセットNo.
    EVENT_CLASS         CHAR(1) DEFAULT '2',        -- イベント型 1:休日・記念日, 2:単イベント, 3:日イベント, 4:週イベント, 5:月イベント, 6:年イベント, 7:タスク
    EVENT_TYPE          INTEGER DEFAULT 0,          -- 詳細種別 1:毎年の休日, 2:不定期の休日, 3:ハッピーマンデー, 4:国民の休日, 5:振替休日, 6:毎年の記念日, 7:毎年のM月第W週日曜日
    NAME                VARCHAR(50),                -- 名前
    TERM_FROM           VARCHAR(8),                 -- 期間開始日(yyyyMMdd)
    TERM_TO             VARCHAR(8),                 -- 期間終了日(yyyyMMdd)
    START_TIME          VARCHAR(4) DEFAULT ' ',     -- 開始時刻(HHmm)
    END_TIME            VARCHAR(4) DEFAULT ' ',     -- 終了時刻(HHmm)
    PLACE               VARCHAR(50),                -- 場所
    AFFILIATE           VARCHAR(2048),              -- 関係者
    SUNDAY              CHAR(1) DEFAULT '1',        -- 日曜日（1:実施, 0:実施しない）
    MONDAY              CHAR(1) DEFAULT '1',        -- 月曜日（1:実施, 0:実施しない）
    TUESDAY             CHAR(1) DEFAULT '1',        -- 火曜日（1:実施, 0:実施しない）
    WEDNESDAY           CHAR(1) DEFAULT '1',        -- 水曜日（1:実施, 0:実施しない）
    THURSDAY            CHAR(1) DEFAULT '1',        -- 木曜日（1:実施, 0:実施しない）
    FRIDAY              CHAR(1) DEFAULT '1',        -- 金曜日（1:実施, 0:実施しない）
    SATURDAY            CHAR(1) DEFAULT '1',        -- 土曜日（1:実施, 0:実施しない）
    HOLIDAY             CHAR(1) DEFAULT '1',        -- 祝日（1:実施, 0:実施しない）
    EXCEPT_MOVE         CHAR(1) DEFAULT '0',        -- 例外日の扱い（0:中止, 1:前にずらす, 2:後ろにずらす）
    INVALID_MOVE        CHAR(1) DEFAULT '0',        -- 無効日の扱い（0:中止, 1:前にずらす, 2:後ろにずらす）
    INCLUDE_DAYS        VARCHAR(2048),              -- 実施日のリスト(yyyyMMdd,yyyyMMdd,・・・)
    EXCLUDE_DAYS        VARCHAR(2048),              -- 除外日のリスト(yyyyMMdd,yyyyMMdd,・・・)
    OPEN_OTHERS         CHAR(1) DEFAULT '0',        -- 公開（0:他者に非公開, 1:他者に公開, 2:他者には「予定あり」と表示）
    COLOR               VARCHAR(6),                 -- 表示色（16進数RGB, 例: 白=FFFFFF）
    MEMO                VARCHAR(4096),              -- メモ
    REP_INTERVAL        INTEGER,                    -- 繰り返し単位
    REP_DAY             INTEGER,                    -- 日
    REP_WEEK            INTEGER,                    -- 週
    REP_MONTH           INTEGER,                    -- 月
    LABEL               VARCHAR(10),                -- ラベル
    LABEL_START         INTEGER,                    -- ラベル開始数値
    DAY_COUNT           INTEGER,                    -- 日数
    PRE_TASK            INTEGER,                    -- 先行タスク
    PROGRESS            INTEGER,                    -- 進捗率
    FINISHED            CHAR(1) DEFAULT '0',        -- 完了フラグ（1:完了済み）
    DELAY_MAIL_S        CHAR(1) DEFAULT '0',        -- 開始遅延メール送信（1:送信）
    DELAY_MAIL_E        CHAR(1) DEFAULT '0',        -- 終了遅延メール送信（1:送信）
    NOTICE_MAILS        VARCHAR(2048),              -- メール通知のリスト(yyyyMMddHHmm,yyyyMMddHHmm,・・・)
    DEL_FLG             CHAR(1) DEFAULT '0',        -- 削除フラグ（1:削除済み）
    REGIST_USER         INTEGER,                    -- 登録ユーザー(User No.)
    REGIST_TIME         TIMESTAMP,                  -- 登録日時
    UPDATE_USER         INTEGER,                    -- 更新ユーザー(User No.)
    UPDATE_TIME         TIMESTAMP,                  -- 更新日時
    VERSION_NO          INTEGER                     -- バージョン番号
);

-- イベントビュー展開テーブル
CREATE TABLE YM_EVENTVIEW_SPREAD (
    EVENTVIEW_NO        INTEGER,                    -- イベントビューNo.
    SPREAD_DATE         VARCHAR(8),                 -- 展開日付(yyyyMMdd)
    EVENTSET_NO         INTEGER,                    -- イベントセットNo.
    EVENT_NO            INTEGER,                    -- イベントNo.
    START_TIME          VARCHAR(4) DEFAULT ' ',     -- 開始時刻(HHmm)
    END_TIME            VARCHAR(4) DEFAULT ' ',     -- 終了時刻(HHmm)
    EVENT_CLASS         CHAR(1) DEFAULT '2',        -- イベント型 1:休日・記念日, 2:単イベント, 3:日イベント, 4:週イベント, 5:月イベント, 6:年イベント, 7:タスク
    EVENT_TYPE          INTEGER DEFAULT 0,          -- 詳細種別 1:毎年の休日, 2:不定期の休日, 3:ハッピーマンデー, 4:毎年の記念日, 5:不定期の記念日
    NAME                VARCHAR(50),                -- 名前
    EVENT_TERM          CHAR(1) DEFAULT '0',        -- 複数日イベント 0:単日, 1:開始日, 2:中間, 3:終了日
    COLOR               VARCHAR(6),                 -- 表示色（16進数RGB, 例: 白=FFFFFF）
    PRIMARY KEY(EVENTVIEW_NO, SPREAD_DATE, EVENTSET_NO, EVENT_NO)
);

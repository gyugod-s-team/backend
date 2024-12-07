-- Your SQL goes here

CREATE TABLE user
(
  user_id       VARCHAR(36)  NOT NULL,
  nickname      VARCHAR(50)  NOT NULL,
  email         VARCHAR(100) NOT NULL,
  phone_number  VARCHAR(50)  NOT NULL,
  password_hash TEXT         NOT NULL,
  last_login_at TIMESTAMP    NULL    ,
  created_at    TIMESTAMP    NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at    TIMESTAMP    NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  PRIMARY KEY (user_id)
) COMMENT '유저';

-- Add migration script here

-- 创建meta_info表
CREATE TABLE `meta_info` (
    `id` VARCHAR(16) NOT NULL,
    `version` VARCHAR(16) NOT NULL,
    `database` VARCHAR(32) NOT NULL DEFAULT 'SQLITE',
    `initialized` BOOLEAN NOT NULL DEFAULT false
);

-- 初始化表 `meta_info`
INSERT INTO `meta_info` VALUES (
    'meta', '0.1.0', 'sqlite', false
);

-- 创建user表 
CREATE TABLE `user` (
    `uid` UUID PRIMARY KEY,
    `username` VARCHAR(64) UNIQUE NOT NULL,
    `password` VARCHAR(256) NOT NULL,
    `email` VARCHAR(256) UNIQUE NOT NULL,
    `active_rid` UUID,
    `created` DATETIME NOT NULL DEFAULT CURRENT_DATE,
    `changed` DATETIME NOT NULL DEFAULT CURRENT_DATE
);

-- 创建role表
CREATE TABLE `role` (
    `rid` UUID PRIMARY KEY,
    `name` VARCHAR(64) UNIQUE NOT NULL,
    `desc` VARCHAR(2048) NOT NULL,
    `is_base` BOOLEAN NOT NULL DEFAULT false,
    `created` DATETIME NOT NULL DEFAULT CURRENT_DATE,
    `changed` DATETIME NOT NULL DEFAULT CURRENT_DATE
);

INSERT INTO `role` (`rid`, `name`, `desc`, `is_base`) VALUES (
    '01916F2A-9935-71CB-8DAE-D37DA6B6FC39', 'SuperUser', '超级用户', true
), (
    '01916F2F-DE60-719F-B184-DDF0AA0ED604', 'Administrator', '管理员', true
), (
    '01916F31-01E6-73F0-8806-8588B3083690', 'Poster', '发帖人', true
), (
    '01916F33-10A1-7308-83EA-843BACD17818', 'Guest', '访客', true
), (
    '01917036-8EDD-719F-A011-31C36BA9F7BE', 'Public', '公开', true
);
-- 创建api表
CREATE TABLE `api` (
    `aid` UUID PRIMARY KEY,
    `name` VARCHAR(64) UNIQUE NOT NULL,
    `path` VARCHAR(2048) NOT NULL,
    `method` VARCHAR(8) NOT NULL,
    `code` VARCHAR(256) UNIQUE NOT NULL,
    `created` DATETIME NOT NULL DEFAULT CURRENT_DATE,
    `changed` DATETIME NOT NULL DEFAULT CURRENT_DATE
);

CREATE UNIQUE INDEX `uni_path_method` ON `api` (`path`, `method`);

-- 创建menu表
CREATE TABLE `menu` (
    `mid` UUID PRIMARY KEY,
    `href` VARCHAR(2048) UNIQUE NOT NULL,
    `name` VARCHAR(126) UNIQUE NOT NULL,
    `component` VARCHAR(2048),
    `is_visible` BOOLEAN NOT NULL DEFAULT true,
    `status` VARCHAR(64) NOT NULL,
    `keep_alive` BOOLEAN NOT NULL DEFAULT false,
    `order` INT NOT NULL DEFAULT 0,
    `pid` UUID,
    `created` DATETIME NOT NULL DEFAULT CURRENT_DATE,
    `changed` DATETIME NOT NULL DEFAULT CURRENT_DATE
);

-- 创建taxonomy表
CREATE TABLE `taxonomy` (
    `tid` UUID PRIMARY KEY,
    `name` VARCHAR(512) UNIQUE NOT NULL,
    `path` VARCHAR(2048) UNIQUE NOT NULL,
    `cover` VARCHAR(2048),
    `desc` VARCHAR(2048) NOT NULL DEFAULT '',
    `created` DATETIME NOT NULL DEFAULT CURRENT_DATE,
    `changed` DATETIME NOT NULL DEFAULT CURRENT_DATE
);

INSERT INTO `taxonomy` (`tid`, `name`, `path`, `desc`) VALUES (
    '01917263-C9C5-7178-A60B-5BA9436E8D5D', '根节点', '/', '所有taxonomy的根'
);

-- 创建node_type表
CREATE TABLE `node_type` (
    `ntid` UUID PRIMARY KEY,
    `name` VARCHAR(64) UNIQUE NOT NULL,
    `schema` TEXT NOT NULL
);

-- 创建node表
CREATE TABLE `node` (
    `nid` UUID PRIMARY KEY,
    `vid` UUID UNIQUE NOT NULL,
    `ntid` UUID NOT NULL,
    `language` VARCHAR(8) NOT NULL,
    `title` VARCHAR(256) NOT NULL,
    `sticky` VARCHAR(32) NOT NULL,
    `uid` UUID NOT NULL,
    `status` VARCHAR(32) NOT NULL,
    `created` DATETIME NOT NULL DEFAULT CURRENT_DATE,
    `changed` DATETIME NOT NULL DEFAULT CURRENT_DATE,
    `comment` VARCHAR(32) NOT NULL,
    `tnid` UUID,
    `data` JSON,
    `doc_id` UUID NOT NULL
);

CREATE UNIQUE INDEX `uni_uid_title` ON `node` (`uid`, `title`);

-- 创建node_reversion表
CREATE TABLE `node_reversion` (
    `vid` UUID PRIMARY KEY,
    `nid` UUID NOT NULL,
    `uid` UUID NOT NULL,
    `title` VARCHAR(256) NOT NULL,
    `data` JSON,
    `log` VARCHAR(512) NOT NULL DEFAULT '',
    `timestamp` DATETIME NOT NULL DEFAULT CURRENT_DATE
);

-- 创建node_mtm_taxonomy表
CREATE TABLE `node_mtm_taxonomy` (
    `nid` UUID NOT NULL,
    `tid` UUID NOT NULL,
    PRIMARY KEY (`nid`, `tid`)
);

-- 创建role_mtm_api表
CREATE TABLE `role_mtm_api` (
    `rid` UUID NOT NULL,
    `aid` UUID NOT NULL,
    PRIMARY KEY (`rid`, `aid`)
);

-- 创建role_mtm_menu表
CREATE TABLE `role_mtm_menu` (
    `rid` UUID NOT NULL,
    `mid` UUID NOT NULL,
    PRIMARY KEY (`rid`, `mid`)
);

-- 创建user_mtm_role表
CREATE TABLE `user_mtm_role` (
    `uid` UUID NOT NULL,
    `rid` UUID NOT NULL,
    PRIMARY KEY (`uid`, `rid`)
);

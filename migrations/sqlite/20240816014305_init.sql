-- Add migration script here

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

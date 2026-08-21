# sce_app_mini-runtime（脱机运行时）

星火编辑器（SCE）脱机运行时：无需编辑器快速调试与发布本地游戏项目工程。

核心能力：**B 模式（本地工程调试）完全脱机运行**——不启动星火编辑器/对战平台，一键把本地 SCE 项目跑起来：

- 载荷自举同步（官方 update-info + OSS 通道，版本跟随服务器）
- 运行时按项目 api_version 组装切换（默认「星火编辑器-api\<N\> 运行时」，另保留对战平台测试/正式环境）
- 调试 staging 生成（白名单拷贝 + 入口包装）
- 控制协议客户端（assign_host → 上传工程 → 起局 → host 日志）
- 凭证库（多账号收割/回写编辑器/验证/扫码自登录/userid 登录态自动获取）
- 游戏进程拉起与截屏自验

## 安装

通过宿主 [bgd_sce_tools](https://github.com/woaye168/bgd_sce_tools) 的「应用市场」安装（宿主需配置 GitHub Token，仓库为私有）。宿主启动应用时透传 `--project-path <项目根>`。

## 快速开始（GUI）

1. 「凭证」页：**导入编辑器凭证**（或扫码自登录），凭证名留空自动按 `{userid}_{时间}_{env}_{类型}` 命名。
2. 「凭证」页：对凭证点 **刷新登录态**——起一次脱机客户端真实登录，自动抓回 userid/昵称写回凭证库。
3. 「调试」页：选择运行时（默认 星火编辑器-\<项目api\>），userid 自动从凭证带入（可手改）。
4. 点 **启动调试**：运行时未就绪会自动先下载载荷（首次约 150MB），随后 assign_host→上传→起局→拉起游戏窗口。

## CLI

```
sce_app_mini-runtime auth list
sce_app_mini-runtime auth verify [凭证名]
sce_app_mini-runtime auth import <凭证名> <项目路径>
sce_app_mini-runtime auth login <项目路径>
sce_app_mini-runtime auth refresh <凭证名> [--runtime <载荷目录>]

sce_app_mini-runtime payload sync [--project <路径>] [--runtime <载荷目录>] [--api 13] [--kind editor-13|tester_test|tester_prod] [--dry-run]

sce_app_mini-runtime debug start --project <路径> --user <userid> [--runtime <载荷目录>] [--cred <凭证名>] [--kind ...] [--hold <秒>]
sce_app_mini-runtime debug stop [--staging <暂存目录> | --runtime <载荷目录> --project <路径>]

sce_app_mini-runtime capture [--title <窗口标题子串>] [--out <输出.png>]
```

## 文档

- `doc/research/`——逆向研究知识库（控制协议/载荷包体系/运行时/凭证与 userid/WSS 明文抓取，索引见 README）
- `doc/requirements/`——版本需求文档
- `AGENTS.md`——开发与修改约定

## 构建与发布

```bash
cargo check && cargo check --examples
cargo build --release
git tag v0.x.y && git push origin v0.x.y   # CI 构建并上传 exe + app-release.json + base_assets.7z
```

版本号唯一来源是 git tag；本应用无自我更新，版本更新由宿主应用市场负责。

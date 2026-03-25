# 指令宝 (CmdVault)

**指令宝** (CmdVault) 是一款轻量级桌面应用，用于保存、搜索和快速复制常用命令。

[中文文档](README_zh.md) | [English Documentation](README.md)

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Platform](https://img.shields.io/badge/platform-Windows-lightgrey.svg)
![Tech Stack](https://img.shields.io/badge/stack-Tauri%20%2B%20Svelte%20%2B%20SQLite-orange.svg)

## ✨ 功能特性

- **命令管理** - 轻松创建、编辑、删除和查看命令
- **模糊搜索** - 实时过滤，快速找到所需命令
- **全局快捷键** - 使用 `Win+Shift+C` 快速唤起（可自定义）
- **键盘导航** - 方向键导航，Enter 复制，Esc 关闭
- **数据导出/导入** - 通过 JSON 文件备份和恢复命令
- **GitHub 同步** - 使用 GitHub Gist 跨设备同步命令
- **轻量快速** - 基于 Tauri 构建，资源占用极小
- **现代 UI** - 简洁直观的界面，支持深色主题

## 🚀 快速开始

### 前置要求

- Node.js 18+
- Rust (用于 Tauri)
- npm 或 pnpm

### 安装步骤

```bash
# 克隆仓库
git clone https://github.com/yourusername/CmdVault.git
cd CmdVault

# 安装依赖
npm install

# 启动开发环境
npm run tauri:dev

# 构建生产版本
npm run tauri:build
```

## 📖 使用指南

### 基本操作

| 操作 | 快捷键 |
|------|--------|
| 打开应用 | `Win+Shift+C` |
| 搜索命令 | 在搜索框输入 |
| 导航列表 | `↑` / `↓` |
| 复制命令 | `Enter` |
| 关闭应用 | `Esc` |
| 新建命令 | `Ctrl+N` |
| 编辑命令 | `Ctrl+E` |
| 删除命令 | `Del` |
| 打开设置 | `⚙` 按钮 |

### 使用流程

1. 按下全局快捷键打开指令宝
2. 输入关键词过滤命令
3. 使用方向键选择命令
4. 按 `Enter` 复制到剪贴板
5. 应用自动关闭

## 🛠️ 技术栈

- **前端**: [Svelte 5](https://svelte.dev/) + [Vite](https://vitejs.dev/)
- **后端**: [Tauri v2](https://tauri.app/) (Rust)
- **数据库**: [SQLite](https://www.sqlite.org/)
- **搜索**: [Fuse.js](https://fusejs.io/) (模糊搜索)
- **同步**: GitHub Gist API

## 📁 项目结构

```
CmdVault/
├── src/                    # 前端源代码
│   ├── lib/
│   │   ├── components/     # Svelte 组件
│   │   ├── stores/         # 状态管理
│   │   └── utils/          # 工具函数
│   ├── App.svelte
│   └── main.ts
├── src-tauri/              # 后端 (Rust) 源代码
│   ├── src/
│   │   ├── commands/       # Tauri 命令
│   │   ├── db/             # 数据库操作
│   │   └── lib.rs
│   └── tauri.conf.json
└── package.json
```

## 🔧 配置说明

### 自定义全局快捷键

1. 打开设置（点击齿轮图标）
2. 输入您喜欢的快捷键组合
3. 点击"保存"应用设置

### GitHub 同步设置

1. 进入设置
2. 输入您的 GitHub Personal Access Token
3. 输入您的 Gist ID（留空可创建新的）
4. 点击"同步"按钮同步命令

## 📸 截图

### 主界面
![主界面](docs/screenshots/main.png)

### 命令编辑器
![命令编辑器](docs/screenshots/editor.png)

### 设置面板
![设置面板](docs/screenshots/settings.png)

## 🤝 贡献指南

欢迎贡献！请随时提交 Pull Request。

1. Fork 本仓库
2. 创建您的特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交您的更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 提交 Pull Request

## 📝 许可证

本项目采用 MIT 许可证 - 详见 [LICENSE](LICENSE) 文件

## 🙏 致谢

- [Tauri](https://tauri.app/) - 出色的桌面框架
- [Svelte](https://svelte.dev/) - 增强型 Web 应用框架
- [Fuse.js](https://fusejs.io/) - 强大的模糊搜索库
- 所有贡献者和支持者

## 📬 联系方式

- **项目名称**: 指令宝 (CmdVault)
- **版本**: 0.1.0
- **仓库**: [GitHub](https://github.com/yourusername/CmdVault)

---

Made with ❤️ by CmdVault Team

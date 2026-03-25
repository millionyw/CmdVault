# CmdVault

**CmdVault** (指令宝) is a lightweight desktop application for storing, searching, and quickly copying frequently used commands.

[中文文档](README_zh.md) | [English Documentation](README.md)

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Platform](https://img.shields.io/badge/platform-Windows-lightgrey.svg)
![Tech Stack](https://img.shields.io/badge/stack-Tauri%20%2B%20Svelte%20%2B%20SQLite-orange.svg)

## ✨ Features

- **Command Management** - Create, edit, delete, and view your commands with ease
- **Fuzzy Search** - Real-time filtering to find commands instantly
- **Global Hotkey** - Quick access with `Win+Shift+C` (customizable)
- **Keyboard Navigation** - Navigate with arrow keys, copy with Enter, close with Esc
- **Data Export/Import** - Backup and restore commands via JSON files
- **GitHub Sync** - Sync your commands across devices using GitHub Gist
- **Lightweight & Fast** - Built with Tauri for minimal resource usage
- **Modern UI** - Clean and intuitive interface with dark theme support

## 🚀 Quick Start

### Prerequisites

- Node.js 18+ 
- Rust (for Tauri)
- npm or pnpm

### Installation

```bash
# Clone the repository
git clone https://github.com/yourusername/CmdVault.git
cd CmdVault

# Install dependencies
npm install

# Start development server
npm run tauri:dev

# Build for production
npm run tauri:build
```

## 📖 Usage

### Basic Operations

| Action | Shortcut |
|--------|----------|
| Open App | `Win+Shift+C` |
| Search Commands | Type in search box |
| Navigate List | `↑` / `↓` |
| Copy Command | `Enter` |
| Close App | `Esc` |
| New Command | `Ctrl+N` |
| Edit Command | `Ctrl+E` |
| Delete Command | `Del` |
| Open Settings | `⚙` button |

### Workflow

1. Press the global hotkey to open CmdVault
2. Type keywords to filter commands
3. Use arrow keys to select a command
4. Press `Enter` to copy to clipboard
5. The app will auto-close after copying

## 🛠️ Tech Stack

- **Frontend**: [Svelte 5](https://svelte.dev/) + [Vite](https://vitejs.dev/)
- **Backend**: [Tauri v2](https://tauri.app/) (Rust)
- **Database**: [SQLite](https://www.sqlite.org/)
- **Search**: [Fuse.js](https://fusejs.io/) (fuzzy search)
- **Sync**: GitHub Gist API

## 📁 Project Structure

```
CmdVault/
├── src/                    # Frontend source code
│   ├── lib/
│   │   ├── components/     # Svelte components
│   │   ├── stores/         # State management
│   │   └── utils/          # Utility functions
│   ├── App.svelte
│   └── main.ts
├── src-tauri/              # Backend (Rust) source code
│   ├── src/
│   │   ├── commands/       # Tauri commands
│   │   ├── db/             # Database operations
│   │   └── lib.rs
│   └── tauri.conf.json
└── package.json
```

## 🔧 Configuration

### Custom Global Hotkey

1. Open Settings (click the gear icon)
2. Enter your preferred hotkey combination
3. Click "Save" to apply

### GitHub Sync Setup

1. Go to Settings
2. Enter your GitHub Personal Access Token
3. Enter your Gist ID (or leave blank to create new)
4. Click "Sync" to sync your commands

## 📸 Screenshots

### Main Interface
![Main Interface](docs/screenshots/main.png)

### Command Editor
![Command Editor](docs/screenshots/editor.png)

### Settings Panel
![Settings](docs/screenshots/settings.png)

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [Tauri](https://tauri.app/) - For the amazing desktop framework
- [Svelte](https://svelte.dev/) - For the cybernetically enhanced web app framework
- [Fuse.js](https://fusejs.io/) - For the powerful fuzzy search
- All contributors and supporters

## 📬 Contact

- **Project Name**: CmdVault (指令宝)
- **Version**: 0.1.0
- **Repository**: [GitHub](https://github.com/yourusername/CmdVault)

---

Made with ❤️ by CmdVault Team

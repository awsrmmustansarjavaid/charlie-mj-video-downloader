# Github Repo Structure

Your GitHub repository should now look like this

```
.github/
└── workflows/
    ├── ci.yml
    └── windows-release.yml

app/
├── package.json
├── package-lock.json
├── src/
└── src-tauri/
    ├── Cargo.toml
    ├── src/
    └── tauri.conf.json
```

How the two workflows work

```
                    GitHub Repository
                           │
             ┌─────────────┴─────────────┐
             │                           │
             ▼                           ▼
          ci.yml              windows-release.yml
             │                           │
       ┌─────┴─────┐                     │
       │           │                     │
       ▼           ▼                     ▼
   Frontend      Rust              Tauri Build
    npm ci     cargo check              │
       │           │                    ▼
       ▼           ▼             Windows Application
      Build       Check                  │
                                         ▼
                                  NSIS Windows Installer
                                         │
                                         ▼
                                      .exe
                                         │
                                         ▼
                                  GitHub Artifact
```

---

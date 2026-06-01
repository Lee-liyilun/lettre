# Tauri + Vue 3

This template should help get you started developing with Tauri + Vue 3 in Vite. The template uses Vue 3 `<script setup>` SFCs, check out the [script setup docs](https://v3.vuejs.org/api/sfc-script-setup.html#sfc-script-setup) to learn more.

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Vue - Official](https://marketplace.visualstudio.com/items?itemName=Vue.volar) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## Template created! To get started run:
```  
cd tool
npm install --registry=https://registry.npmmirror.com/
npm run tauri android init
```

## For Desktop development, run:
```
npm run tauri dev
```

## For Android development, run:
```
npm run tauri android dev
```

## 打包命令
```
1、Linux需要安装包
apt install -y libwebkit2gtk-4.1-dev build-essential curl wget file libxdo-dev libssl-dev libayatana-appindicator3-dev librsvg2-dev

npm run tauri build

如果提示找不到命令，也可以直接使用 cargo tauri build
签名
$env:TAURI_SIGNING_PRIVATE_KEY = Get-Content C:\Users\10729\.tauri\lettre.key -Raw
$env:TAURI_SIGNING_PRIVATE_KEY_PASSWORD = ""
```

## 自动打包发布
```
1、修改版本号
src-tauri/tauri.conf.json

2、提交代码
git add .
git commit -m 'v0.1.1'
git push github master:main

3、打tag推送
git tag v0.2.0
git push github v0.2.0
```

# <img src="app-icon.png" width="25" alt="app-icon.png"> 笺影-Lettre 

> 保持热爱，奔赴山海

## Template created! To get started run:
```  
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
package.json（可以不改，强迫症除外）

2、提交代码
git add .
git commit -m 'v0.1.1'
git push github master:main

3、打tag推送
git tag v0.2.0
git push github v0.2.0
```

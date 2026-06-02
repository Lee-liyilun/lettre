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
npm run tauri build

如果提示找不到命令，也可以直接使用 cargo tauri build
```

## 自动打包发布
```
1、修改版本号
src-tauri/tauri.conf.json
package.json

2、提交代码
git add .
git commit -m 'v0.2.1'
git push github master:main

3、打tag推送
git tag v0.2.1
git push github v0.2.1
```

## 版本更新

### v0.2.2
> 😄Good Day
1、完成了GitHub的CI/CD，同步到gitee不搞了手动吧，反正用的人不多🥺
2、优化自动更新提示框
3、我写的简财变成了密语😅，从git历史中恢复了✌️

### v0.2.1
> 🎉同志们我来啦
1、印记：工作留痕，未来打算接入大模型做日报、周报、月报
2、密语：保留一些客户或者项目信息，进行了加密处理，保存好key哦，丢了就没了
3、简财：简单的理财计算器。免责申明：本工具仅用于个人学习与技术研究，不构成任何投资建议。😁
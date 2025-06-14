<!-- src里main.rs变动了, routes多了vidio.rs, blog.rs, models多了vidio.rs, blog.rs

数据直接更新在ws给的数据库里了

用于测试的前端代码是statics里的video.html, blog.html

cargo run成功就可以看到效果
 -->

## 最终版

0. 前端所有模块均已完成。包含前端源码（frontend-source），和前端打包结果（frontend-package/dist）
1. 对后端进行了较多修改，但最终未能成功整合代码，所以干脆分为两个服务器：处于 8001 端口（lgr 分支）的 AI 模块服务器、处于 8000 端口（czg 分支）的其余模块服务器
2. 为了成功运行对应的服务，您需要：

   1. `cargo run`运行`rust_space`、`rust_space_ai`两个 Rocket 服务。
   2. 对于 `rust_space`的判题服务模块，要求运行的操作系统上安装有 docker。
   3. 对于前端，需要先调整`frontend-package/conf/nginx.conf`文件：

   ```nginx
        location /api1 {
            rewrite ^/api1(/.*)$ $1 break;
            proxy_pass http://192.168.200.130:8000; # 替换为你的 API 服务地址，如果你在Windows本地运行，请将地址改为localhost:8000
            proxy_set_header Host $host;
            proxy_set_header X-Real-IP $remote_addr;
            proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
            proxy_set_header X-Forwarded-Proto $scheme;
        }

        location /api2 {
            rewrite ^/api2(/.*)$ $1 break;
            proxy_pass http://192.168.200.130:8001; # 替换为你的 AI API 服务地址，如果你在Windows本地运行，请将地址改为localhost:8001
            proxy_set_header Host $host;
            proxy_set_header X-Real-IP $remote_addr;
            proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
            proxy_set_header X-Forwarded-Proto $scheme;
        }
   ```

   随后在 Windows 环境下运行反向代理 nginx.exe，使用浏览器访问 localhost:8080 即可。

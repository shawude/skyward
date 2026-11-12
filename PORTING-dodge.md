# 一加13（dodge）移植 Skyward

配置已经从 ColorOS 17 的全量包里逐项读出来了，不再靠猜。

## 解析结果

包：`PJZ110_17.0.0.103(CN01)`（Android 17，`CP2A.260605.016`，2026-10-08 构建）

| 项 | 值 | 来源 |
| --- | --- | --- |
| 屏幕 | 1440×3168，density 640，6.82" | `my_product/build.prop` |
| 触摸 | Synaptics S3910，`synaptics_tcm_hbp@0` | `dtbo` |
| 触摸模块 | `oplus_bsp_tp_hbp_syna_s3910` | `vendor_dlkm` 的 `modules.load` |
| WLAN | `qcom,cnss-peach`，`qca_cld3_peach_v2` | `dtbo`、`vendor_dlkm` |
| 密钥服务 | `keymint-service-qti`、`gatekeeper-service-qti` | `vendor/bin/hw` |
| weaver | `weaver-service.nxp`（eSE1 在 `secure_element-service.qti`） | `odm/bin/hw` |
| 振动 | `qcom-hv-haptics` | `vendor_dlkm` |
| 原厂 recovery | 100 MiB，SHA-256 `406ef664…16b016` | `recovery` 分区 |

改动过的文件：

- [devices/dodge.toml](/root/workspace/skyward/devices/dodge.toml)：设备配置
- [devices/dodge/recovery.img.zip](/root/workspace/skyward/devices/dodge/recovery.img.zip)：原厂 recovery 分区（压缩后 46 MB）
- [crates/skyward-config/tests/dodge.rs](/root/workspace/skyward/crates/skyward-config/tests/dodge.rs)：配置解析测试
- [crates/skyward-config/src/lib.rs](/root/workspace/skyward/crates/skyward-config/src/lib.rs)：放宽 `[stock] package` 校验，允许带 query 参数、按哈希命名的包
- [.github/workflows/build.yml](/root/workspace/skyward/.github/workflows/build.yml)：有 `devices/<code>/recovery.img.zip` 时优先用它，不再下载 OTA

## 为什么 recovery 镜像进仓库

一加 OTA 直链带签名，`Expires` 只有几分钟（这条 `02:42:49` 就过期了），CI 无法复用。所以把取出来的 recovery 分区压缩放进 `devices/dodge/recovery.img.zip`，CI 直接解压用，不再依赖直链。

想换成直连方式也可以：重新取一条签名链接，把 `[stock].package` 换成它，并删掉 `devices/dodge/recovery.img.zip`。

## 构建

把仓库 fork 到自己的 GitHub 账号，把上面 5 个文件的内容同步过去（`devices/dodge/recovery.img.zip` 用网页的 “Upload files” 上传），push 后 **Actions → Build** 会自动跑：

- `check`：跑测试，包括 `dodge.toml` 的解析
- `recovery`：产出 `skyward-<commit>-dodge-PJZ110_17.0.0.103(CN01).zip`

产物在 Actions 页面的 artifacts 里。

## 刷入（需要一台电脑）

```sh
unzip skyward-<commit>-dodge-PJZ110_17.0.0.103(CN01).zip
fastboot flash recovery skyward-<commit>-dodge-PJZ110_17.0.0.103(CN01).img
fastboot reboot recovery
```

第一次只验证：屏幕方向、触摸、音量/电源键、`adb devices`、能否进 fastbootd。

## 还没确认的项

- `corner_radius`：暂时填 100，等真机看圆角
- `[touch]`：一加的 hbp 层可能需要、也可能不需要写 sysfs，先留空
- `[wlan] probe`：ColorOS 的 captive portal 地址还是占位
- `[wipe] logs`：ColorOS 的日志目录还没列
- 在线更新、双系统：后者要改 `skyward-systems` 里认 HyperOS 的部分
- 解密：keymint/gatekeeper/weaver 已配好，但要真机验证

如果没有电脑，镜像可以先构建出来放着，刷入找机会借电脑。

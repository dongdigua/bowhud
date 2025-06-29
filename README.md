# BowHUD - Minecraft 弓箭瞄准镜 for Wayland
大量参考了 [MC狙击标尺，给萌新弓箭手的分划板](https://www.bilibili.com/video/BV1vV4y1Z7jh)，
但是使用 gtk cairo 动态绘制分划板

## 精度 TODO
- 平射：
- +5:
- -5:

## args
`$1` 屏幕分辨率：Minecraft 窗口的短边像素数。Default：1080

## commands
- `fov(\d+)` 设置 fov
- `[+-](\d+)` 设置目标高度
- `eff` 适应弓箭瞄准时的视场角变化
- `@(bow|crossbow)` 不同弹射物初速度

## Video
[[MC x Wayland] 给萌新弓箭手的狙击标尺，但是 GTK Layer Shell](https://www.bilibili.com/video/BV1QvgfzfEMu)

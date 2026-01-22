# Antigravity-Manager 代码修复 Walkthrough

**修改日期**: 2026-01-02

## 修改概述

已按照 `docs/` 目录中的文档要求修复新拉取的 Antigravity-Manager 代码，使其与旧版 v3.3.8 功能一致。

## 修改内容

### 1. 图像生成端点 (`handle_images_generations`)

**文件**: [openai.rs](file:///H:/AI/Antigravity-Manager/src-tauri/src/proxy/handlers/openai.rs)

**新增功能**:
- `quality` → `imageSize` 映射 (standard→1K, medium→2K, hd→4K)
- `responseModalities: ["IMAGE"]` 参数
- `imageConfig.imageSize` 参数

```diff
+ let image_size = match quality {
+     "hd" => "4K",
+     "medium" => "2K",
+     _ => "1K",
+ };

  "generationConfig": {
      "candidateCount": 1,
+     "responseModalities": ["IMAGE"],
      "imageConfig": {
          "aspectRatio": aspect_ratio,
+         "imageSize": image_size
      }
  }
```

---

### 2. 图像编辑端点 (`handle_images_edits`)

**新增功能**:
- `image2`, `image3` 多参考图支持
- `quality` 参数解析
- `size` → `aspectRatio` 映射
- `quality` → `imageSize` 映射
- 图像生成模式 `generationConfig`

```diff
+ let mut image2_data: Option<String> = None;
+ let mut image3_data: Option<String> = None;
+ let mut quality = "standard".to_string();

+ } else if name == "image2" { ... }
+ } else if name == "image3" { ... }
+ } else if name == "quality" { ... }

+ let aspect_ratio = match size.as_str() { ... };
+ let image_size = match quality.as_str() { ... };

  "generationConfig": {
-     "maxOutputTokens": 8192, ...
+     "responseModalities": ["IMAGE"],
+     "imageConfig": { "aspectRatio": ..., "imageSize": ... }
  }
```

---

### 3. Token Manager (`get_token`)

**文件**: [token_manager.rs](file:///H:/AI/Antigravity-Manager/src-tauri/src/proxy/token_manager.rs)

**修改** (第 251 行):
```diff
- if target_token.is_none() && !rotate && quota_group != "image_gen" {
+ // [2026-01-01] 移除 image_gen 的特殊排除
+ if target_token.is_none() && !rotate {
```

---

### 4. 编译脚本

**新建**: [build.bat](file:///H:/AI/Antigravity-Manager/build.bat)

---

## 功能对比验证

| 功能 | 文档要求 | 修复后 | v3.3.8 |
|------|----------|--------|--------|
| `responseModalities` (generations) | ✅ | ✅ | ✅ |
| `imageSize` (generations) | ✅ | ✅ | ✅ |
| `quality` 参数 (edits) | ✅ | ✅ | ✅ |
| `image2`/`image3` 多图 (edits) | ✅ | ✅ | ✅ |
| `aspectRatio`/`imageSize` (edits) | ✅ | ✅ | ✅ |
| image_gen 60s 锁定 | ✅ | ✅ | ✅ |

**结论**: 所有修改与 v3.3.8 和文档要求一致 ✅

---

## 编译方法

```powershell
cd H:\AI\Antigravity-Manager
.\build.bat
```

输出位置: `src-tauri\target\release\antigravity-tools.exe`

---

## 回滚说明

### 回滚 handle_images_generations
删除以下代码:
- `let image_size = match quality { ... }` 映射块
- `generationConfig` 中的 `responseModalities` 和 `imageSize`

### 回滚 handle_images_edits
删除以下代码:
- `image2_data`, `image3_data`, `quality` 变量
- `"image2"`, `"image3"`, `"quality"` 字段解析
- `aspect_ratio`, `image_size` 映射块
- 将 `generationConfig` 改回文本生成模式

### 回滚 token_manager
将第 251 行改回:
```rust
if target_token.is_none() && !rotate && quota_group != "image_gen" {
```

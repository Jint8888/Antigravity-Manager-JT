# Antigravity-Manager 代码修复实现计划

## 问题描述

经过对 `docs/` 目录文档的审查和与旧版本 v3345 代码的对比，发现当前 Antigravity-Manager 代码缺少以下功能：

| 功能 | 文档要求 | 当前代码 | v3345 | 状态 |
|------|---------|---------|-------|------|
| **图像生成 imageSize 映射** | ✅ | ❌ 缺失 | ✅ | 需修复 |
| **图像生成 responseModalities** | ✅ | ❌ 缺失 | ✅ | 需修复 |
| **图像编辑 多参考图(image2-7)** | ✅ | ❌ 缺失 | ✅ | 需修复 |
| **图像编辑 quality 参数** | ✅ | ❌ 缺失 | ✅ | 需修复 |
| **图像编辑 aspectRatio/imageSize** | ✅ | ❌ 缺失 | ✅ | 需修复 |
| **音频转录功能** | ✅ | ✅ 已实现 | ✅ | 无需修改 |

---

## Proposed Changes

### [Modify] `src-tauri/src/proxy/handlers/openai.rs`

#### 1. `handle_images_generations` 函数修复

**添加 quality → imageSize 映射**（约第 970-986 行）:

```rust
// 3. 映射 quality 到 imageSize (Gemini API 实际控制分辨率的参数)
// "1K" = ~1024px, "2K" = ~2048px, "4K" = ~4096px
let image_size = match quality {
    "hd" => "4K",
    "medium" => "2K",
    _ => "1K",
};

tracing::info!(
    "[Images] Mapped quality '{}' to imageSize '{}'",
    quality,
    image_size
);
```

**更新 generationConfig**（约第 1040-1046 行）:

```diff
  "generationConfig": {
      "candidateCount": 1,
+     "responseModalities": ["IMAGE"],
      "imageConfig": {
-         "aspectRatio": aspect_ratio
+         "aspectRatio": aspect_ratio,
+         "imageSize": image_size
      }
  }
```

---

#### 2. `handle_images_edits` 函数修复

**添加多参考图变量声明**（约第 1175-1188 行）:

```diff
- let mut image_data = None;
+ let mut image_data: Option<String> = None;
+ let mut image2_data: Option<String> = None;
+ let mut image3_data: Option<String> = None;
+ let mut image4_data: Option<String> = None;
+ let mut image5_data: Option<String> = None;
+ let mut image6_data: Option<String> = None;
+ let mut image7_data: Option<String> = None;
  let mut mask_data = None;
+ let mut quality = "standard".to_string();
```

**添加多参考图字段解析**（约第 1232-1272 行）:

```rust
} else if name == "image2" {
    let data = field.bytes().await.map_err(...)?;
    image2_data = Some(base64::engine::general_purpose::STANDARD.encode(data));
} else if name == "image3" { ... }
// ... image4-7 同理
} else if name == "quality" {
    if let Ok(val) = field.text().await {
        quality = val;
    }
}
```

**添加 aspectRatio 和 imageSize 映射**（约第 1321-1343 行）:

```rust
// 2. 映射 size 到 aspectRatio
let aspect_ratio = match size.as_str() {
    "1792x1024" | "1920x1080" => "16:9",
    "1024x1792" | "1080x1920" => "9:16",
    "1024x768" | "1280x960" => "4:3",
    "768x1024" | "960x1280" => "3:4",
    _ => "1:1",
};

// 3. 映射 quality 到 imageSize
let image_size = match quality.as_str() {
    "hd" => "4K",
    "medium" => "2K",
    _ => "1K",
};
```

**添加多参考图到请求体**（约第 1370-1434 行）:

```rust
// Add second reference image if provided
if let Some(data) = image2_data {
    contents_parts.push(json!({
        "inlineData": { "mimeType": "image/png", "data": data }
    }));
    tracing::info!("[Images] Added second reference image");
}
// ... image3-7 同理
```

**更新 generationConfig**（约第 1448-1455 行）:

```diff
  "generationConfig": {
      "candidateCount": 1,
-     "maxOutputTokens": 8192,
-     "stopSequences": [],
-     "temperature": 1.0,
-     "topP": 0.95,
-     "topK": 40
+     "responseModalities": ["IMAGE"],
+     "imageConfig": {
+         "aspectRatio": aspect_ratio,
+         "imageSize": image_size
+     }
  }
```

---

## Verification Plan

### 编译验证

由于这是 Rust/Tauri 项目，需要用户手动验证编译：

```powershell
cd H:\AI\Antigravity-Manager
npm install
npm run tauri build
```

成功标准：编译无错误，输出 `src-tauri\target\release\antigravity-tools.exe`

### 代码对比验证

修改后的代码将与 v3345 相同函数进行逐行比对，确保功能一致。

---

## 回滚说明

如需回滚，参照文档 `docs/walkthrough-文档对齐修复.md` 中的回滚说明删除对应代码。

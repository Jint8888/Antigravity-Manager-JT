# Antigravity-Manager 文档对齐修复 Walkthrough

**修改日期**: 2026-01-04  
**修改者**: Claude Sonnet 4  
**版本**: v3.4.0 (对齐旧版本功能)

---

## 概述

对照开发文档 (`docs/`) 对新拉取的 Antigravity-Manager 代码进行修复，使其达到与旧版本 (`Antigravity-Manager-old`) 功能一致的状态。

---

## 修改内容

### 1. 图像分辨率修复 (`handle_images_generations`)

**文件**: [openai.rs](file:///h:/AI/Antigravity-Manager/src-tauri/src/proxy/handlers/openai.rs)

**新增功能**:
- `quality` → `imageSize` 映射：
  | quality | imageSize | 分辨率 |
  |---------|-----------|--------|
  | standard | 1K | ~1024px |
  | medium | 2K | ~2048px |
  | hd | 4K | ~4096px |
- 添加 `responseModalities: ["IMAGE"]`
- 添加 `imageConfig.imageSize` 参数
- 添加映射日志输出

```diff
+ // 3. 映射 quality 到 imageSize (Gemini API 实际控制分辨率的参数)
+ let image_size = match quality {
+     "hd" => "4K",
+     "medium" => "2K",
+     _ => "1K",
+ };
+ tracing::info!("[Images] Mapped quality '{}' to imageSize '{}'", quality, image_size);

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

### 2. 多参考图功能 + 图像编辑配置 (`handle_images_edits`)

**文件**: [openai.rs](file:///h:/AI/Antigravity-Manager/src-tauri/src/proxy/handlers/openai.rs)

**新增功能**:
- `image2_data`, `image3_data` 变量支持多参考图
- `quality` 参数解析
- `size` → `aspectRatio` 映射
- `quality` → `imageSize` 映射
- 更新 `generationConfig` 为图像生成配置

```diff
- let mut image_data = None;
+ let mut image_data: Option<String> = None;
+ let mut image2_data: Option<String> = None;
+ let mut image3_data: Option<String> = None;
+ let mut quality = "standard".to_string();

+ } else if name == "image2" { ... }
+ } else if name == "image3" { ... }
+ } else if name == "quality" { ... }

+ // Add second/third reference images
+ if let Some(data) = image2_data { ... }
+ if let Some(data) = image3_data { ... }

+ // 映射 size → aspectRatio, quality → imageSize
+ let aspect_ratio = match size.as_str() { ... };
+ let image_size = match quality.as_str() { ... };

  "generationConfig": {
      "candidateCount": 1,
-     "maxOutputTokens": 8192,
-     "temperature": 1.0, ...
+     "responseModalities": ["IMAGE"],
+     "imageConfig": {
+         "aspectRatio": aspect_ratio,
+         "imageSize": image_size
+     }
  }
```

---

### 3. 音频上传功能

#### 3.1 音频处理模块

**文件**: [mod.rs](file:///h:/AI/Antigravity-Manager/src-tauri/src/proxy/audio/mod.rs) (新建)

- `AudioProcessor::detect_mime_type()` - 支持 MP3/WAV/M4A/OGG/FLAC/AIFF
- `AudioProcessor::encode_to_base64()` - Base64 编码
- `AudioProcessor::exceeds_size_limit()` - 15MB 限制检查
- 包含单元测试

#### 3.2 音频转录处理器

**文件**: [audio.rs](file:///h:/AI/Antigravity-Manager/src-tauri/src/proxy/handlers/audio.rs) (新建)

- `handle_audio_transcription()` - OpenAI Whisper API 兼容
- 支持 multipart/form-data (file, model, prompt)
- 验证文件大小 ≤15MB
- 调用 Gemini v1internal API

#### 3.3 模块导出

**文件**: [mod.rs](file:///h:/AI/Antigravity-Manager/src-tauri/src/proxy/mod.rs)
```diff
+ pub mod audio;            // 音频处理模块
```

**文件**: [mod.rs](file:///h:/AI/Antigravity-Manager/src-tauri/src/proxy/handlers/mod.rs)
```diff
+ pub mod audio;
```

#### 3.4 路由注册

**文件**: [server.rs](file:///h:/AI/Antigravity-Manager/src-tauri/src/proxy/server.rs)
```diff
+ .route(
+     "/v1/audio/transcriptions",
+     post(handlers::audio::handle_audio_transcription),
+ ) // 音频转录 API
```

#### 3.5 OpenAI 协议扩展

**文件**: [models.rs](file:///h:/AI/Antigravity-Manager/src-tauri/src/proxy/mappers/openai/models.rs)
```diff
+ #[serde(rename = "audio_url")]
+ AudioUrl { audio_url: OpenAIAudioUrl },

+ pub struct OpenAIAudioUrl {
+     pub url: String,  // data:audio/mp3;base64,...
+ }
```

**文件**: [request.rs](file:///h:/AI/Antigravity-Manager/src-tauri/src/proxy/mappers/openai/request.rs)
```diff
+ OpenAIContentBlock::AudioUrl { audio_url } => {
+     if audio_url.url.starts_with("data:audio/") {
+         // 解析并添加到 inlineData
+     }
+ }
```

---

## 与旧版本对比验证

| 功能点 | 旧版本 | 修复后 | 状态 |
|--------|--------|--------|------|
| quality→imageSize (generations) | ✅ | ✅ | 一致 |
| responseModalities (generations) | ✅ | ✅ | 一致 |
| imageConfig.imageSize (generations) | ✅ | ✅ | 一致 |
| image2/image3 (edits) | ✅ | ✅ | 一致 |
| quality 解析 (edits) | ✅ | ✅ | 一致 |
| aspectRatio/imageSize (edits) | ✅ | ✅ | 一致 |
| responseModalities (edits) | ✅ | ✅ | 一致 |
| audio/mod.rs | ✅ | ✅ | 一致 |
| handlers/audio.rs | ✅ | ✅ | 一致 |
| /v1/audio/transcriptions 路由 | ✅ | ✅ | 一致 |
| OpenAIAudioUrl 模型 | ✅ | ✅ | 一致 |
| AudioUrl 请求转换 | ✅ | ✅ | 一致 |

**结论**: 所有修改与旧版本功能一致 ✅

---

## 编译命令

```powershell
cd h:\AI\Antigravity-Manager
npm install
npm run tauri build
```

输出: `src-tauri\target\release\antigravity-tools.exe`

---

## 回滚说明

### 回滚图像分辨率修复 (handle_images_generations)

删除以下代码:
- 第 738-751 行: `let image_size = match quality { ... }` 及日志
- 第 788 行: `let image_size = image_size.to_string();`
- 第 803 行: `"responseModalities": ["IMAGE"],`
- 第 806 行: `, "imageSize": image_size`

### 回滚多参考图功能 (handle_images_edits)

删除以下代码:
- 第 936-937 行: `image2_data`, `image3_data` 声明
- 第 942 行: `quality` 变量声明
- 第 996-1011 行: image2/image3/quality 字段解析
- 第 1007 行: 日志中的 `images=...`
- 第 1062-1093 行: image2/image3 添加到 contents_parts
- 第 1095-1121 行: aspectRatio/imageSize 映射
- 将 generationConfig 改回文本生成模式

### 回滚音频功能

删除文件:
- `src-tauri/src/proxy/audio/mod.rs`
- `src-tauri/src/proxy/handlers/audio.rs`

修改文件:
- `src-tauri/src/proxy/mod.rs`: 移除 `pub mod audio;`
- `src-tauri/src/proxy/handlers/mod.rs`: 移除 `pub mod audio;`
- `src-tauri/src/proxy/server.rs`: 移除音频路由
- `src-tauri/src/proxy/mappers/openai/models.rs`: 移除 AudioUrl
- `src-tauri/src/proxy/mappers/openai/request.rs`: 移除 AudioUrl 处理

---

## 相关文档

- [图像分辨率修复开发文档](file:///h:/AI/Antigravity-Manager/docs/图像分辨率修复开发文档.md)
- [多参考图功能开发文档](file:///h:/AI/Antigravity-Manager/docs/多参考图功能开发文档.md)
- [音频上传功能实现规划](file:///h:/AI/Antigravity-Manager/docs/音频上传功能实现规划.md)
- [音频上传功能实现记录](file:///h:/AI/Antigravity-Manager/docs/walkthrough-音频上传功能实现.md)

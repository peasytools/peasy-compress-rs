# peasy-compress

[![crates.io](https://img.shields.io/crates/v/peasy-compress)](https://crates.io/crates/peasy-compress)
[![docs.rs](https://docs.rs/peasy-compress/badge.svg)](https://docs.rs/peasy-compress)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

Async Rust client for the [Peasy Compress](https://peasytools.com) API -- compress, archive, and extract files with ZIP, TAR, gzip, Brotli, and Zstandard. Built with reqwest, serde, and tokio.

Built from [Peasy Tools](https://peasytools.com), a comprehensive file compression toolkit offering free online tools for creating and extracting archives across multiple formats. The glossary covers compression algorithms from classic DEFLATE to modern Brotli and Zstandard, while guides compare archive formats and explain lossless vs lossy compression strategies.

> **Explore compression formats at [peasytools.com](https://peasytools.com)** -- [ZIP](https://peasytools.com/formats/zip/), [TAR](https://peasytools.com/formats/tar/), [Gzip](https://peasytools.com/formats/gz/), and more.

<p align="center">
  <img src="demo.gif" alt="peasy-compress demo -- ZIP, TAR, and gzip compression tools in Rust terminal" width="800">
</p>

## Table of Contents

- [Install](#install)
- [Quick Start](#quick-start)
- [What You Can Do](#what-you-can-do)
  - [Compression and Archive Tools](#compression-and-archive-tools)
  - [Browse Compression Reference Content](#browse-compression-reference-content)
  - [Search and Discovery](#search-and-discovery)
- [API Client](#api-client)
  - [Available Methods](#available-methods)
- [Learn More About Compression](#learn-more-about-compression)
- [Also Available](#also-available)
- [Peasy Developer Tools](#peasy-developer-tools)
- [License](#license)

## Install

```toml
[dependencies]
peasy-compress = "0.2.0"
tokio = { version = "1", features = ["full"] }
```

Or via cargo:

```bash
cargo add peasy-compress
```

## Quick Start

```rust
use peasy_compress::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    // List available compression tools
    let tools = client.list_tools(&Default::default()).await?;
    for tool in &tools.results {
        println!("{}: {}", tool.name, tool.description);
    }

    Ok(())
}
```

## What You Can Do

### Compression and Archive Tools

File compression reduces storage costs and speeds up network transfers. Archive formats like ZIP and TAR bundle multiple files into a single container, while compression algorithms like DEFLATE, Brotli, and Zstandard reduce the size of the archived data. Understanding which format to use -- ZIP for cross-platform compatibility, TAR+gzip for Unix workflows, or Brotli for web content delivery -- is critical for choosing the right tool for each task.

| Format | Type | Best For |
|--------|------|----------|
| ZIP | Archive + Compression | Cross-platform file sharing, per-file compression |
| TAR | Archive only | Unix/Linux bundles, combined with gzip or Brotli |
| gzip | Compression only | Single file compression, HTTP content encoding |
| Brotli | Compression only | Web assets, higher compression ratios than gzip |
| Zstandard | Compression only | Fast real-time compression, database backups |

```rust
use peasy_compress::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    // Fetch the ZIP compression tool details
    let tool = client.get_tool("zip-compress").await?;
    println!("Tool: {}", tool.name);           // ZIP compression tool
    println!("Category: {}", tool.category);   // Archive and compression category

    // List supported compression and archive formats
    let formats = client.list_formats(&Default::default()).await?;
    for fmt in &formats.results {
        println!(".{} -- {} ({})", fmt.extension, fmt.name, fmt.mime_type);
    }

    Ok(())
}
```

Learn more: [ZIP Format Reference](https://peasytools.com/formats/zip/) · [TAR Format Reference](https://peasytools.com/formats/tar/) · [Archive Formats Compared](https://peasytools.com/guides/archive-formats-compared/)

### Browse Compression Reference Content

The compression glossary explains key concepts from basic archiving terminology to advanced algorithm details. Understanding the difference between lossless and lossy compression, how DEFLATE works under the hood, and when to choose Brotli over gzip helps developers make informed decisions about their compression pipelines.

| Glossary Term | Description |
|---------------|-------------|
| Archive | A single file containing multiple files and directory structures |
| TAR | Tape Archive format that bundles files without compression |
| Brotli | Google-developed compression algorithm optimized for web content |
| Lossless Compression | Compression that preserves all original data upon decompression |
| Zstandard | Facebook-developed algorithm balancing speed and compression ratio |

```rust
// Browse compression glossary terms programmatically
let glossary = client.list_glossary(&peasy_compress::ListOptions {
    search: Some("gzip".into()),
    ..Default::default()
}).await?;
for term in &glossary.results {
    println!("{}: {}", term.term, term.definition); // Compression algorithm definition
}

// Read in-depth guides comparing archive formats
let guides = client.list_guides(&peasy_compress::ListGuidesOptions {
    category: Some("compression".into()),
    ..Default::default()
}).await?;
for guide in &guides.results {
    println!("{} ({})", guide.title, guide.audience_level); // Guide title and difficulty
}
```

Learn more: [Archive Glossary](https://peasytools.com/glossary/archive/) · [Brotli Glossary](https://peasytools.com/glossary/brotli/) · [Lossless vs Lossy Compression Guide](https://peasytools.com/guides/lossless-vs-lossy-compression-guide/)

### Search and Discovery

The unified search endpoint queries across all compression tools, glossary terms, guides, and supported file formats simultaneously. This is useful for building CLI tools, documentation search, or CI/CD integrations that need to find the right compression approach.

```rust
// Search across all compression tools, glossary, and guides
let results = client.search("zip", Some(20)).await?;
println!("Found {} tools, {} glossary terms",
    results.results.tools.len(),
    results.results.glossary.len()); // Cross-content compression search results
```

Learn more: [TAR Glossary](https://peasytools.com/glossary/tar/) · [Lossless Compression Glossary](https://peasytools.com/glossary/lossless-compression/) · [Zstandard Glossary](https://peasytools.com/glossary/zstandard/)

## API Client

The client wraps the [Peasy Tools REST API](https://peasytools.com/developers/) with strongly-typed Rust structs using serde deserialization.

```rust
use peasy_compress::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    // Or with a custom base URL:
    // let client = Client::with_base_url("https://custom.example.com");

    // List tools with filters
    let opts = peasy_compress::ListOptions {
        page: Some(1),
        limit: Some(10),
        search: Some("zip".into()),
        ..Default::default()
    };
    let tools = client.list_tools(&opts).await?;

    // Get a specific tool
    let tool = client.get_tool("zip-compress").await?;
    println!("{}: {}", tool.name, tool.description);

    // Search across all content
    let results = client.search("zip", Some(20)).await?;
    println!("Found {} tools", results.results.tools.len());

    // Browse the glossary
    let glossary = client.list_glossary(&peasy_compress::ListOptions {
        search: Some("gzip".into()),
        ..Default::default()
    }).await?;
    for term in &glossary.results {
        println!("{}: {}", term.term, term.definition);
    }

    // Discover guides
    let guides = client.list_guides(&peasy_compress::ListGuidesOptions {
        category: Some("compression".into()),
        ..Default::default()
    }).await?;
    for guide in &guides.results {
        println!("{} ({})", guide.title, guide.audience_level);
    }

    // List format conversions
    let conversions = client.list_conversions(&peasy_compress::ListConversionsOptions {
        source: Some("zip".into()),
        ..Default::default()
    }).await?;

    Ok(())
}
```

### Available Methods

| Method | Description |
|--------|-------------|
| `list_tools(&opts)` | List tools (paginated, filterable) |
| `get_tool(slug)` | Get tool by slug |
| `list_categories(&opts)` | List tool categories |
| `list_formats(&opts)` | List file formats |
| `get_format(slug)` | Get format by slug |
| `list_conversions(&opts)` | List format conversions |
| `list_glossary(&opts)` | List glossary terms |
| `get_glossary_term(slug)` | Get glossary term |
| `list_guides(&opts)` | List guides |
| `get_guide(slug)` | Get guide by slug |
| `list_use_cases(&opts)` | List use cases |
| `search(query, limit)` | Search across all content |
| `list_sites()` | List Peasy sites |
| `openapi_spec()` | Get OpenAPI specification |

Full API documentation at [peasytools.com/developers/](https://peasytools.com/developers/).
OpenAPI 3.1.0 spec: [peasytools.com/api/openapi.json](https://peasytools.com/api/openapi.json).

## Learn More About Compression

- **Formats**: [ZIP](https://peasytools.com/formats/zip/) · [TAR](https://peasytools.com/formats/tar/) · [Gzip](https://peasytools.com/formats/gz/) · [All Formats](https://peasytools.com/formats/)
- **Guides**: [Archive Formats Compared](https://peasytools.com/guides/archive-formats-compared/) · [Lossless vs Lossy Compression](https://peasytools.com/guides/lossless-vs-lossy-compression-guide/) · [All Guides](https://peasytools.com/guides/)
- **Glossary**: [Archive](https://peasytools.com/glossary/archive/) · [TAR](https://peasytools.com/glossary/tar/) · [Brotli](https://peasytools.com/glossary/brotli/) · [Lossless Compression](https://peasytools.com/glossary/lossless-compression/) · [Zstandard](https://peasytools.com/glossary/zstandard/) · [All Terms](https://peasytools.com/glossary/)
- **API**: [REST API Docs](https://peasytools.com/developers/) · [OpenAPI Spec](https://peasytools.com/api/openapi.json)

## Also Available

| Language | Package | Install |
|----------|---------|---------|
| **Python** | [peasy-compress](https://pypi.org/project/peasy-compress/) | `pip install "peasy-compress[all]"` |
| **TypeScript** | [peasy-compress](https://www.npmjs.com/package/peasy-compress) | `npm install peasy-compress` |
| **Go** | [peasy-compress-go](https://pkg.go.dev/github.com/peasytools/peasy-compress-go) | `go get github.com/peasytools/peasy-compress-go` |
| **Ruby** | [peasy-compress](https://rubygems.org/gems/peasy-compress) | `gem install peasy-compress` |

## Peasy Developer Tools

Part of the [Peasy Tools](https://peasytools.com) open-source developer ecosystem.

| Package | PyPI | npm | crates.io | Description |
|---------|------|-----|-----------|-------------|
| peasy-pdf | [PyPI](https://pypi.org/project/peasy-pdf/) | [npm](https://www.npmjs.com/package/peasy-pdf) | [crate](https://crates.io/crates/peasy-pdf) | PDF merge, split, rotate, compress -- [peasypdf.com](https://peasypdf.com) |
| peasy-image | [PyPI](https://pypi.org/project/peasy-image/) | [npm](https://www.npmjs.com/package/peasy-image) | [crate](https://crates.io/crates/peasy-image) | Image resize, crop, convert, compress -- [peasyimage.com](https://peasyimage.com) |
| peasy-audio | [PyPI](https://pypi.org/project/peasy-audio/) | [npm](https://www.npmjs.com/package/peasy-audio) | [crate](https://crates.io/crates/peasy-audio) | Audio trim, merge, convert, normalize -- [peasyaudio.com](https://peasyaudio.com) |
| peasy-video | [PyPI](https://pypi.org/project/peasy-video/) | [npm](https://www.npmjs.com/package/peasy-video) | [crate](https://crates.io/crates/peasy-video) | Video trim, resize, thumbnails, GIF -- [peasyvideo.com](https://peasyvideo.com) |
| peasy-css | [PyPI](https://pypi.org/project/peasy-css/) | [npm](https://www.npmjs.com/package/peasy-css) | [crate](https://crates.io/crates/peasy-css) | CSS minify, format, analyze -- [peasycss.com](https://peasycss.com) |
| **peasy-compress** | [PyPI](https://pypi.org/project/peasy-compress/) | [npm](https://www.npmjs.com/package/peasy-compress) | [crate](https://crates.io/crates/peasy-compress) | ZIP, TAR, gzip compression -- [peasytools.com](https://peasytools.com) |
| peasy-document | [PyPI](https://pypi.org/project/peasy-document/) | [npm](https://www.npmjs.com/package/peasy-document) | [crate](https://crates.io/crates/peasy-document) | Markdown, HTML, CSV, JSON conversion -- [peasyformats.com](https://peasyformats.com) |
| peasytext | [PyPI](https://pypi.org/project/peasytext/) | [npm](https://www.npmjs.com/package/peasytext) | [crate](https://crates.io/crates/peasytext) | Text case conversion, slugify, word count -- [peasytext.com](https://peasytext.com) |

## License

MIT

use anyhow::{bail, Context};
use std::io::Write;
use std::path::{Path, PathBuf};
use zip::write::SimpleFileOptions;
use zip::{CompressionMethod, ZipWriter};

const EXPORT_FILES: [&str; 10] = [
    "JD.txt",
    "Extracted.json",
    "FitScore.md",
    "TailorPlan.md",
    "Resume_1pg_Tailored.md",
    "Resume_2pg_Tailored.md",
    "RecruiterMessage.md",
    "HiringManagerMessage.md",
    "CoverNote_Short.md",
    "TrackerRow.csv",
];

const SECTION_ORDER: [(&str, &str); 6] = [
    ("Resume", "Resume_1pg_Tailored.md"),
    ("Tailor Plan", "TailorPlan.md"),
    ("Fit Score", "FitScore.md"),
    ("Recruiter Message", "RecruiterMessage.md"),
    ("Hiring Manager Message", "HiringManagerMessage.md"),
    ("Cover Short", "CoverNote_Short.md"),
];

const PDF_PAGE_WIDTH_PT: i32 = 612;
const PDF_PAGE_HEIGHT_PT: i32 = 792;
const PDF_MARGIN_PT: i32 = 72;
const PDF_FONT_SIZE_PT: i32 = 11;
const PDF_LEADING_PT: i32 = 14;

pub fn export_markdown_bundle(packet_dir: &Path, out_dir: &Path) -> anyhow::Result<PathBuf> {
    std::fs::create_dir_all(out_dir).with_context(|| format!("creating {}", out_dir.display()))?;
    let target = out_dir.join(
        packet_dir
            .file_name()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| "packet".to_string()),
    );
    if target.exists() {
        std::fs::remove_dir_all(&target)
            .with_context(|| format!("removing {}", target.display()))?;
    }
    std::fs::create_dir_all(&target).with_context(|| format!("creating {}", target.display()))?;

    for name in EXPORT_FILES {
        let source = packet_dir.join(name);
        if source.exists() {
            std::fs::copy(&source, target.join(name))
                .with_context(|| format!("copying {}", source.display()))?;
        }
    }
    if packet_dir.join("Diff.md").exists() {
        std::fs::copy(packet_dir.join("Diff.md"), target.join("Diff.md"))
            .with_context(|| format!("copying {}", packet_dir.join("Diff.md").display()))?;
    }

    Ok(target)
}

fn xml_escape(input: &str) -> String {
    input
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

fn collect_sections(packet_dir: &Path) -> anyhow::Result<Vec<(String, String)>> {
    let mut sections: Vec<(String, String)> = Vec::new();
    for (title, file) in SECTION_ORDER {
        let path = packet_dir.join(file);
        if path.exists() {
            let body = std::fs::read_to_string(&path)
                .with_context(|| format!("reading {}", path.display()))?;
            sections.push((title.to_string(), body));
        }
    }
    Ok(sections)
}

fn docx_document_xml(packet_dir: &Path) -> anyhow::Result<String> {
    let sections = collect_sections(packet_dir)?;

    let mut xml = String::from(
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:document xmlns:wpc="http://schemas.microsoft.com/office/word/2010/wordprocessingCanvas" xmlns:mc="http://schemas.openxmlformats.org/markup-compatibility/2006" xmlns:o="urn:schemas-microsoft-com:office:office" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" xmlns:m="http://schemas.openxmlformats.org/officeDocument/2006/math" xmlns:v="urn:schemas-microsoft-com:vml" xmlns:wp14="http://schemas.microsoft.com/office/word/2010/wordprocessingDrawing" xmlns:wp="http://schemas.openxmlformats.org/drawingml/2006/wordprocessingDrawing" xmlns:w10="urn:schemas-microsoft-com:office:word" xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" xmlns:w14="http://schemas.microsoft.com/office/word/2010/wordml" xmlns:wpg="http://schemas.microsoft.com/office/word/2010/wordprocessingGroup" xmlns:wpi="http://schemas.microsoft.com/office/word/2010/wordprocessingInk" xmlns:wne="http://schemas.microsoft.com/office/2006/wordml" xmlns:wps="http://schemas.microsoft.com/office/word/2010/wordprocessingShape" mc:Ignorable="w14 wp14">
<w:body>
"#,
    );

    for (title, body) in sections {
        xml.push_str(&format!("<w:p><w:r><w:t>{}</w:t></w:r></w:p>\n", xml_escape(&title)));
        for line in body.replace("\r\n", "\n").lines() {
            let escaped = xml_escape(line);
            xml.push_str(&format!(
                "<w:p><w:r><w:t xml:space=\"preserve\">{escaped}</w:t></w:r></w:p>\n"
            ));
        }
        xml.push_str("<w:p><w:r><w:t> </w:t></w:r></w:p>\n");
    }

    xml.push_str("<w:sectPr><w:pgSz w:w=\"12240\" w:h=\"15840\"/><w:pgMar w:top=\"1440\" w:right=\"1440\" w:bottom=\"1440\" w:left=\"1440\" w:header=\"708\" w:footer=\"708\" w:gutter=\"0\"/></w:sectPr></w:body></w:document>");
    Ok(xml)
}

fn pdf_escape_text(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    for ch in input.chars() {
        let normalized = if ch.is_ascii() {
            if ch.is_ascii_control() && ch != '\t' {
                ' '
            } else {
                ch
            }
        } else {
            '?'
        };
        match normalized {
            '\\' => out.push_str("\\\\"),
            '(' => out.push_str("\\("),
            ')' => out.push_str("\\)"),
            '\t' => out.push(' '),
            c => out.push(c),
        }
    }
    out
}

fn pdf_lines(packet_dir: &Path) -> anyhow::Result<Vec<String>> {
    let sections = collect_sections(packet_dir)?;
    let mut lines = Vec::new();

    for (idx, (title, body)) in sections.iter().enumerate() {
        lines.push(title.clone());
        lines.push(String::new());
        lines.extend(body.replace("\r\n", "\n").replace('\r', "\n").lines().map(str::to_string));
        if idx + 1 < sections.len() {
            lines.push(String::new());
        }
    }

    if lines.is_empty() {
        lines.push("ApplyKit Packet Export".to_string());
    }

    Ok(lines)
}

fn build_pdf_page_stream(lines: &[String]) -> Vec<u8> {
    let start_y = PDF_PAGE_HEIGHT_PT - PDF_MARGIN_PT;
    let mut stream = format!(
        "BT\n/F1 {} Tf\n{} TL\n{} {} Td\n",
        PDF_FONT_SIZE_PT, PDF_LEADING_PT, PDF_MARGIN_PT, start_y
    );

    for (idx, line) in lines.iter().enumerate() {
        if idx > 0 {
            stream.push_str("T*\n");
        }
        stream.push('(');
        stream.push_str(&pdf_escape_text(line));
        stream.push_str(") Tj\n");
    }
    stream.push_str("ET\n");
    stream.into_bytes()
}

fn build_pdf_bytes(lines: &[String]) -> Vec<u8> {
    let usable_height = PDF_PAGE_HEIGHT_PT - (PDF_MARGIN_PT * 2);
    let max_lines_per_page = std::cmp::max(1, usable_height / PDF_LEADING_PT) as usize;

    let page_chunks = lines.chunks(max_lines_per_page).collect::<Vec<_>>();
    let page_count = std::cmp::max(1, page_chunks.len());

    let mut objects: Vec<Vec<u8>> = Vec::new();

    objects.push(b"<< /Type /Catalog /Pages 2 0 R >>".to_vec());

    let mut kids = String::new();
    for i in 0..page_count {
        let page_obj_id = 4 + (i * 2);
        kids.push_str(&format!("{page_obj_id} 0 R "));
    }
    objects.push(
        format!("<< /Type /Pages /Kids [{}] /Count {} >>", kids.trim_end(), page_count)
            .into_bytes(),
    );
    objects.push(b"<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>".to_vec());

    for (index, chunk) in page_chunks.iter().enumerate() {
        let content_obj_id = 5 + (index * 2);
        let page_obj = format!(
            "<< /Type /Page /Parent 2 0 R /MediaBox [0 0 {} {}] /Contents {} 0 R /Resources << /Font << /F1 3 0 R >> >> >>",
            PDF_PAGE_WIDTH_PT, PDF_PAGE_HEIGHT_PT, content_obj_id
        )
        .into_bytes();
        objects.push(page_obj);

        let stream = build_pdf_page_stream(chunk);
        let mut content_obj = format!("<< /Length {} >>\nstream\n", stream.len()).into_bytes();
        content_obj.extend_from_slice(&stream);
        content_obj.extend_from_slice(b"endstream");
        objects.push(content_obj);
    }

    if page_chunks.is_empty() {
        let page_obj =
            format!(
                "<< /Type /Page /Parent 2 0 R /MediaBox [0 0 {} {}] /Contents 5 0 R /Resources << /Font << /F1 3 0 R >> >> >>",
                PDF_PAGE_WIDTH_PT, PDF_PAGE_HEIGHT_PT
            )
            .into_bytes();
        objects.push(page_obj);
        let stream = build_pdf_page_stream(lines);
        let mut content_obj = format!("<< /Length {} >>\nstream\n", stream.len()).into_bytes();
        content_obj.extend_from_slice(&stream);
        content_obj.extend_from_slice(b"endstream");
        objects.push(content_obj);
    }

    let mut out = Vec::new();
    out.extend_from_slice(b"%PDF-1.4\n");

    let mut offsets = Vec::with_capacity(objects.len() + 1);
    offsets.push(0usize);
    for (idx, object) in objects.iter().enumerate() {
        offsets.push(out.len());
        out.extend_from_slice(format!("{} 0 obj\n", idx + 1).as_bytes());
        out.extend_from_slice(object);
        out.extend_from_slice(b"\nendobj\n");
    }

    let xref_offset = out.len();
    out.extend_from_slice(format!("xref\n0 {}\n", objects.len() + 1).as_bytes());
    out.extend_from_slice(b"0000000000 65535 f \n");
    for offset in offsets.iter().skip(1) {
        out.extend_from_slice(format!("{offset:010} 00000 n \n").as_bytes());
    }
    out.extend_from_slice(
        format!(
            "trailer\n<< /Size {} /Root 1 0 R >>\nstartxref\n{}\n%%EOF\n",
            objects.len() + 1,
            xref_offset
        )
        .as_bytes(),
    );

    out
}

pub fn export_docx(packet_dir: &Path, out_path: &Path) -> anyhow::Result<()> {
    if let Some(parent) = out_path.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }

    let file = std::fs::File::create(out_path)
        .with_context(|| format!("creating {}", out_path.display()))?;
    let mut zip = ZipWriter::new(file);
    let opts = SimpleFileOptions::default().compression_method(CompressionMethod::Deflated);

    let content_types = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
  <Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/>
  <Default Extension="xml" ContentType="application/xml"/>
  <Override PartName="/word/document.xml" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml"/>
</Types>"#;

    let root_rels = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
  <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" Target="word/document.xml"/>
</Relationships>"#;

    let doc_rels = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"></Relationships>"#;

    let document_xml = docx_document_xml(packet_dir)?;

    zip.start_file("[Content_Types].xml", opts)?;
    zip.write_all(content_types.as_bytes())?;

    zip.start_file("_rels/.rels", opts)?;
    zip.write_all(root_rels.as_bytes())?;

    zip.start_file("word/_rels/document.xml.rels", opts)?;
    zip.write_all(doc_rels.as_bytes())?;

    zip.start_file("word/document.xml", opts)?;
    zip.write_all(document_xml.as_bytes())?;

    zip.finish()?;
    Ok(())
}

pub fn export_pdf(packet_dir: &Path, out_path: &Path) -> anyhow::Result<()> {
    if !packet_dir.exists() {
        bail!("packet directory does not exist: {}", packet_dir.display());
    }
    if !packet_dir.is_dir() {
        bail!("packet path must be a directory: {}", packet_dir.display());
    }
    if let Some(parent) = out_path.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }

    let lines = pdf_lines(packet_dir)?;
    let bytes = build_pdf_bytes(&lines);
    std::fs::write(out_path, bytes).with_context(|| format!("writing {}", out_path.display()))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Read;
    use zip::read::ZipArchive;

    #[test]
    fn markdown_bundle_exports_canonical_files() {
        let packet = tempfile::tempdir().expect("packet");
        std::fs::write(packet.path().join("Resume_1pg_Tailored.md"), "resume").expect("write");
        std::fs::write(packet.path().join("FitScore.md"), "fit").expect("write");
        std::fs::write(packet.path().join("Diff.md"), "diff").expect("write");

        let out = tempfile::tempdir().expect("out");
        let target = export_markdown_bundle(packet.path(), out.path()).expect("export");

        assert!(target.join("Resume_1pg_Tailored.md").exists());
        assert!(target.join("FitScore.md").exists());
        assert!(target.join("Diff.md").exists());
    }

    #[test]
    fn docx_export_contains_expected_entries() {
        let packet = tempfile::tempdir().expect("packet");
        std::fs::write(packet.path().join("Resume_1pg_Tailored.md"), "# Resume\n- Bullet")
            .expect("write");

        let out = tempfile::tempdir().expect("out");
        let out_path = out.path().join("packet.docx");
        export_docx(packet.path(), &out_path).expect("docx");

        let file = std::fs::File::open(&out_path).expect("open");
        let mut archive = ZipArchive::new(file).expect("zip");
        let mut names = Vec::new();
        for i in 0..archive.len() {
            names.push(archive.by_index(i).expect("entry").name().to_string());
        }
        names.sort();
        assert_eq!(
            names,
            vec![
                "[Content_Types].xml",
                "_rels/.rels",
                "word/_rels/document.xml.rels",
                "word/document.xml"
            ]
        );

        let mut doc = archive.by_name("word/document.xml").expect("document");
        let mut xml = String::new();
        doc.read_to_string(&mut xml).expect("read xml");
        assert!(xml.contains("Resume"));
        assert!(xml.contains("Bullet"));
    }

    #[test]
    fn pdf_export_is_deterministic() {
        let packet = tempfile::tempdir().expect("packet");
        std::fs::write(packet.path().join("Resume_1pg_Tailored.md"), "# Resume\n- Bullet")
            .expect("write");
        std::fs::write(packet.path().join("TailorPlan.md"), "Plan line").expect("write");
        let out = tempfile::tempdir().expect("out");

        let a = out.path().join("packet_a.pdf");
        let b = out.path().join("packet_b.pdf");
        export_pdf(packet.path(), &a).expect("pdf a");
        export_pdf(packet.path(), &b).expect("pdf b");

        let a_bytes = std::fs::read(&a).expect("read a");
        let b_bytes = std::fs::read(&b).expect("read b");
        assert_eq!(a_bytes, b_bytes);
        assert!(a_bytes.starts_with(b"%PDF-1.4"));
    }

    #[test]
    fn pdf_export_rejects_missing_packet_dir() {
        let out = tempfile::tempdir().expect("out");
        let missing = out.path().join("missing_packet");
        let err = export_pdf(&missing, &out.path().join("packet.pdf")).expect_err("missing dir");
        assert!(err.to_string().contains("does not exist"));
    }

    #[test]
    fn pdf_export_rejects_non_directory_output_parent() {
        let packet = tempfile::tempdir().expect("packet");
        std::fs::write(packet.path().join("Resume_1pg_Tailored.md"), "resume").expect("write");

        let out = tempfile::tempdir().expect("out");
        let parent_file = out.path().join("not_a_dir");
        std::fs::write(&parent_file, "file").expect("write parent file");
        let target = parent_file.join("packet.pdf");

        let err = export_pdf(packet.path(), &target).expect_err("invalid output parent");
        assert!(err.to_string().contains("creating"));
    }
}

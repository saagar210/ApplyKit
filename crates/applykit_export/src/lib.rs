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

fn docx_document_xml(packet_dir: &Path) -> anyhow::Result<String> {
    let mut sections: Vec<(String, String)> = Vec::new();
    let ordered = [
        ("Resume", "Resume_1pg_Tailored.md"),
        ("Tailor Plan", "TailorPlan.md"),
        ("Fit Score", "FitScore.md"),
        ("Recruiter Message", "RecruiterMessage.md"),
        ("Hiring Manager Message", "HiringManagerMessage.md"),
        ("Cover Short", "CoverNote_Short.md"),
    ];

    for (title, file) in ordered {
        let path = packet_dir.join(file);
        if path.exists() {
            let body = std::fs::read_to_string(&path)
                .with_context(|| format!("reading {}", path.display()))?;
            sections.push((title.to_string(), body));
        }
    }

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

pub fn export_pdf(_packet_dir: &Path, _out_path: &Path) -> anyhow::Result<()> {
    bail!("PDF export not implemented yet; deterministic renderer pending")
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
    fn pdf_export_returns_bounded_stub_error() {
        let packet = tempfile::tempdir().expect("packet");
        let out = tempfile::tempdir().expect("out");
        let err = export_pdf(packet.path(), &out.path().join("packet.pdf")).expect_err("stub");
        assert!(err.to_string().contains("not implemented yet"));
    }
}

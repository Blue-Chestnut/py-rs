use py_rs::PY;

#[derive(PY)]
#[py(export_to = "issue_317/")]
struct VariantId(u32);

#[derive(PY)]
#[py(export_to = "issue_317/")]
struct VariantOverview {
    id: u32,
    name: String,
}

#[derive(PY)]
#[py(export, export_to = "issue_317/")]
struct Container {
    variants: Vec<(VariantId, VariantOverview)>,
}

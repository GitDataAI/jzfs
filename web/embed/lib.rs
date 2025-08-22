use actix_web::{
    get, http::header::{CACHE_CONTROL, CONTENT_TYPE}, HttpRequest, HttpResponse,
    Result,
};
use rust_embed::Embed;
use session::Session;

#[derive(Embed)]
#[folder = "dist/"]
struct Dist;

#[get("/{path:.*}")]
pub async fn web_ui(req: HttpRequest, _session: Session) -> Result<HttpResponse> {
    let path = req.match_info().query("path").trim_start_matches('/');
    if let Some(file) = Dist::get(path) {
        let mime = mime_guess::from_path(path)
            .first_or_octet_stream()
            .to_string();
        return Ok(HttpResponse::Ok()
            .insert_header((CONTENT_TYPE, mime))
            .insert_header((CACHE_CONTROL, "public, max-age=86400"))
            .body(file.data));
    }
    let index = Dist::get("index.html")
        .ok_or_else(|| actix_web::error::ErrorInternalServerError("Missing index.html"))?;
    if let Ok(bytes) = std::str::from_utf8(&*index.data).map(|x|x.to_string()) {
        let mut header = String::new();
        if path == "" {
            header.push_str("<title>GitDataAI | Cloud - Git for Machine Learning Data Management</title>\n");
            header.push_str(
                "<meta name=\"description\" content=\"GitDataAI Cloud provides Git-based data management for machine learning projects. Version control for datasets, models, and AI experiments.\" />\n"
            );
            header.push_str(
                "<meta name=\"keywords\" content=\"Git, machine learning, data management, AI, version control, datasets, models\" />\n"
            );
        } else if path == "repositories" {
            header.push_str("<title>GitDataAI | Repositories - Manage Your ML Data Repositories</title>\n");
            header.push_str(
                "<meta name=\"description\" content=\"Browse and manage your machine learning data repositories. Git-based version control for datasets and models.\" />\n"
            );
            header.push_str(
                "<meta name=\"keywords\" content=\"repositories, ML data, datasets, models, Git repositories, data versioning\" />\n"
            );
        } else if path == "ai" {
            header.push_str("<title>GitDataAI | AI Models - Version Control for Machine Learning Models</title>\n");
            header.push_str(
                "<meta name=\"description\" content=\"Manage AI models with Git-based version control. Track model versions, experiments, and performance metrics.\" />\n"
            );
            header.push_str(
                "<meta name=\"keywords\" content=\"AI models, machine learning, model versioning, experiment tracking, ML models\" />\n"
            );
        } else if path == "dataset" {
            header.push_str("<title>GitDataAI | Datasets - Version Control for ML Datasets</title>\n");
            header.push_str(
                "<meta name=\"description\" content=\"Version control for machine learning datasets. Track dataset changes, manage versions, and collaborate on data.\" />\n"
            );
            header.push_str(
                "<meta name=\"keywords\" content=\"datasets, ML data, data versioning, data management, dataset tracking\" />\n"
            );
        } else if path == "marketplace" {
            header.push_str("<title>GitDataAI | Marketplace - Discover ML Models and Datasets</title>\n");
            header.push_str(
                "<meta name=\"description\" content=\"Discover and share machine learning models and datasets. Explore community-contributed AI resources.\" />\n"
            );
            header.push_str(
                "<meta name=\"keywords\" content=\"marketplace, ML models, datasets, AI resources, community models\" />\n"
            );
        } else if path == "about" {
            header.push_str("<title>GitDataAI | About - Git for Machine Learning</title>\n");
            header.push_str(
                "<meta name=\"description\" content=\"Learn about GitDataAI - Git-based data management platform for machine learning teams.\" />\n"
            );
            header.push_str(
                "<meta name=\"keywords\" content=\"GitDataAI, machine learning, Git, data management, version control\" />\n"
            );
        }

        // 添加通用的SEO标签
        header.push_str("<meta name=\"robots\" content=\"index, follow\" />\n");
        header.push_str("<meta property=\"og:type\" content=\"website\" />\n");
        header.push_str("<meta property=\"og:site_name\" content=\"GitDataAI\" />\n");

        let bytes = bytes.replace("<!-- header -->", &header);
        return Ok(HttpResponse::Ok()
            .insert_header((CONTENT_TYPE, "text/html"))
            .insert_header((CACHE_CONTROL, "no-cache"))
            .body(bytes));
    }
    Ok(HttpResponse::Ok()
        .insert_header((CONTENT_TYPE, "text/html"))
        .insert_header((CACHE_CONTROL, "no-cache"))
        .body(index.data))
}


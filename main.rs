use actix_web::{web, App, HttpServer, HttpResponse, middleware};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
struct Project {
    id: u32,
    title: String,
    description: String,
    technologies: Vec<String>,
    github_url: Option<String>,
    demo_url: Option<String>,
}

// Sample projects data
fn get_projects() -> Vec<Project> {
    vec![
        Project {
            id: 1,
            title: "Prime Number Generator".to_string(),
            description: "A simple prime number generator in Rust.".to_string(),
            technologies: vec!["Rust".to_string()],
            github_url: Some("https://github.com/jacob/hello_world".to_string()),
            demo_url: None,
        },
        Project {
            id: 2,
            title: "Portfolio Website".to_string(),
            description: "A modern portfolio website built with Actix-web.".to_string(),
            technologies: vec!["Rust".to_string(), "Actix-web".to_string(), "HTML".to_string()],
            github_url: Some("https://github.com/jacob/portfolio_website".to_string()),
            demo_url: Some("https://portfolio.example.com".to_string()),
        },
    ]
}

// Home page
async fn home() -> HttpResponse {
    let html = r#"
    <!DOCTYPE html>
    <html>
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>Jacob's Portfolio</title>
        <style>
            * { margin: 0; padding: 0; box-sizing: border-box; }
            body {
                font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
                line-height: 1.6;
                color: #333;
                background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
                min-height: 100vh;
                display: flex;
                flex-direction: column;
            }
            nav {
                background: rgba(0,0,0,0.1);
                padding: 1rem 2rem;
                display: flex;
                justify-content: space-between;
                align-items: center;
                backdrop-filter: blur(10px);
            }
            nav a { color: white; text-decoration: none; margin: 0 1rem; font-weight: 500; }
            .container {
                max-width: 1200px;
                margin: 0 auto;
                padding: 2rem;
                flex: 1;
            }
            .hero {
                text-align: center;
                color: white;
                margin: 3rem 0;
            }
            .hero h1 { font-size: 3rem; margin-bottom: 1rem; }
            .hero p { font-size: 1.3rem; opacity: 0.9; margin-bottom: 2rem; }
            .btn {
                display: inline-block;
                background: white;
                color: #667eea;
                padding: 0.8rem 2rem;
                border-radius: 5px;
                text-decoration: none;
                font-weight: bold;
                transition: transform 0.3s;
            }
            .btn:hover { transform: translateY(-2px); }
            .projects-grid {
                display: grid;
                grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
                gap: 2rem;
                margin-top: 3rem;
            }
            .project-card {
                background: white;
                padding: 2rem;
                border-radius: 8px;
                box-shadow: 0 4px 6px rgba(0,0,0,0.1);
                transition: transform 0.3s;
            }
            .project-card:hover { transform: translateY(-5px); }
            .project-card h3 { color: #667eea; margin-bottom: 0.5rem; }
            .technologies { margin: 1rem 0; }
            .tech-tag {
                display: inline-block;
                background: #f0f0f0;
                padding: 0.3rem 0.8rem;
                border-radius: 3px;
                font-size: 0.85rem;
                margin-right: 0.5rem;
                margin-bottom: 0.5rem;
            }
            .links { margin-top: 1rem; }
            .links a {
                color: #667eea;
                text-decoration: none;
                margin-right: 1rem;
                font-weight: 500;
            }
            footer {
                background: rgba(0,0,0,0.2);
                color: white;
                text-align: center;
                padding: 2rem;
            }
        </style>
    </head>
    <body>
        <nav>
            <h2 style="color: white;">Portfolio</h2>
            <div>
                <a href="/">Home</a>
                <a href="/projects">Projects</a>
            </div>
        </nav>
        <div class="container">
            <div class="hero">
                <h1>Hello, I'm Jacob</h1>
                <p>Hobbyist showcasing some projects</p>
                <a href="/projects" class="btn">View My Work</a>
            </div>
        </div>
        <footer>
            <p>&copy; 2025 Jacob's Portfolio. All rights reserved.</p>
        </footer>
    </body>
    </html>
    "#;
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

// Projects page
async fn projects() -> HttpResponse {
    let projects = get_projects();
    let mut html = r#"
    <!DOCTYPE html>
    <html>
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>My Projects</title>
        <style>
            * { margin: 0; padding: 0; box-sizing: border-box; }
            body {
                font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
                line-height: 1.6;
                color: #333;
                background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
                min-height: 100vh;
            }
            nav {
                background: rgba(0,0,0,0.1);
                padding: 1rem 2rem;
                display: flex;
                justify-content: space-between;
                align-items: center;
                backdrop-filter: blur(10px);
            }
            nav a { color: white; text-decoration: none; margin: 0 1rem; font-weight: 500; }
            .container {
                max-width: 1200px;
                margin: 0 auto;
                padding: 2rem;
            }
            h1 { color: white; margin-bottom: 2rem; font-size: 2.5rem; }
            .projects-grid {
                display: grid;
                grid-template-columns: repeat(auto-fit, minmax(350px, 1fr));
                gap: 2rem;
            }
            .project-card {
                background: white;
                padding: 2rem;
                border-radius: 8px;
                box-shadow: 0 8px 16px rgba(0,0,0,0.1);
                transition: all 0.3s;
            }
            .project-card:hover {
                transform: translateY(-8px);
                box-shadow: 0 12px 24px rgba(0,0,0,0.15);
            }
            .project-card h3 {
                color: #667eea;
                margin-bottom: 0.5rem;
                font-size: 1.5rem;
            }
            .project-card p { color: #666; margin-bottom: 1rem; }
            .technologies { margin: 1rem 0; }
            .tech-tag {
                display: inline-block;
                background: #667eea;
                color: white;
                padding: 0.4rem 0.8rem;
                border-radius: 3px;
                font-size: 0.85rem;
                margin-right: 0.5rem;
                margin-bottom: 0.5rem;
            }
            .links {
                margin-top: 1.5rem;
                display: flex;
                gap: 1rem;
            }
            .links a {
                display: inline-block;
                background: #667eea;
                color: white;
                padding: 0.6rem 1.2rem;
                border-radius: 4px;
                text-decoration: none;
                font-weight: 500;
                transition: background 0.3s;
            }
            .links a:hover { background: #764ba2; }
            footer {
                background: rgba(0,0,0,0.2);
                color: white;
                text-align: center;
                padding: 2rem;
                margin-top: 3rem;
            }
        </style>
    </head>
    <body>
        <nav>
            <h2 style="color: white;">Portfolio</h2>
            <div>
                <a href="/">Home</a>
                <a href="/projects">Projects</a>
            </div>
        </nav>
        <div class="container">
            <h1>My Projects</h1>
            <div class="projects-grid">
    "#.to_string();

    for project in projects {
        html.push_str(&format!(
            r#"
                <div class="project-card">
                    <h3>{}</h3>
                    <p>{}</p>
                    <div class="technologies">
            "#,
            project.title, project.description
        ));

        for tech in project.technologies {
            html.push_str(&format!(r#"<span class="tech-tag">{}</span>"#, tech));
        }

        html.push_str(r#"
                    </div>
                    <div class="links">
            "#);

        if let Some(github) = project.github_url {
            html.push_str(&format!(
                r#"<a href="{}" target="_blank">GitHub</a>"#,
                github
            ));
        }
        if let Some(demo) = project.demo_url {
            html.push_str(&format!(
                r#"<a href="{}" target="_blank">Live Demo</a>"#,
                demo
            ));
        }

        html.push_str(
            r#"
                    </div>
                </div>
            "#,
        );
    }

    html.push_str(
        r#"
            </div>
        </div>
        <footer>
            <p>&copy; 2025 Jacob's Portfolio. All rights reserved.</p>
        </footer>
    </body>
    </html>
    "#,
    );

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

// API endpoint to get projects as JSON
async fn api_projects() -> HttpResponse {
    HttpResponse::Ok().json(get_projects())
}

// API endpoint to get single project
async fn api_project(id: web::Path<u32>) -> HttpResponse {
    let projects = get_projects();
    match projects.iter().find(|p| p.id == *id) {
        Some(project) => HttpResponse::Ok().json(project),
        None => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Project not found"
        })),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 Portfolio website starting on http://localhost:8080");
    println!("📖 Visit http://localhost:8080 to view your portfolio");
    println!("📂 API endpoints: /api/projects, /api/projects/:id");

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .route("/", web::get().to(home))
            .route("/projects", web::get().to(projects))
            .route("/api/projects", web::get().to(api_projects))
            .route("/api/projects/{id}", web::get().to(api_project))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

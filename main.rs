use axum::{
    extract::{Path, Query},
    response::Html,
    routing::get,
    Router,
};
use serde::Serialize;
use std::{collections::HashMap, net::SocketAddr};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

#[derive(Clone, Serialize)]
struct Evento {
    id: u32,
    nome: &'static str,
    data: &'static str,
    local: &'static str,
    preco: f32,
}

#[derive(Clone, Serialize)]
struct Artista {
    slug: &'static str,
    nome: &'static str,
    genero: &'static str,   // "trap" | "funk" | "rap"
    origem: &'static str,   // "nacional" | "internacional"
    descricao: &'static str,
    imagem: &'static str,   // caminho em /static/img/...
}

fn eventos() -> Vec<Evento> {
    vec![
        Evento {
            id: 1,
            nome: "ONE MORE SHOW • Tech Week 2025",
            data: "21 • 22 • 23 de novembro",
            local: "Arena Digital • Online",
            preco: 49.90,
        },
    ]
}

fn artistas() -> Vec<Artista> {
    vec![
        // Trap nacional
        Artista {
            slug: "matue",
            nome: "Matuê",
            genero: "trap",
            origem: "nacional",
            descricao: "Um dos maiores nomes do trap nacional, dono de shows com visual cinematográfico.",
            imagem: "/static/img/matue.jpg",
        },
        Artista {
            slug: "teto",
            nome: "Teto",
            genero: "trap",
            origem: "nacional",
            descricao: "Flow melódico, refrões grudentos e uma estética forte de internet.",
            imagem: "/static/img/teto.jpg",
        },
        Artista {
            slug: "wiu",
            nome: "WIU",
            genero: "trap",
            origem: "nacional",
            descricao: "Produtor e artista que mistura trap com referências pop e eletrônicas.",
            imagem: "/static/img/wiu.jpg",
        },
        // Trap internacional
        Artista {
            slug: "travis-scott",
            nome: "Travis Scott",
            genero: "trap",
            origem: "internacional",
            descricao: "Referência global em trap, conhecido por cenários imersivos e experiência de show completa.",
            imagem: "/static/img/travis_scott.jpg",
        },
        Artista {
            slug: "playboi-carti",
            nome: "Playboi Carti",
            genero: "trap",
            origem: "internacional",
            descricao: "Visual experimental, performances intensas e uma base de fãs extremamente engajada.",
            imagem: "/static/img/playboi_carti.jpg",
        },
        // Funk nacional
        Artista {
            slug: "kevin-o-chris",
            nome: "Kevin O Chris",
            genero: "funk",
            origem: "nacional",
            descricao: "Referência do funk 150 BPM, dono de hits que dominam as pistas.",
            imagem: "/static/img/kevin_o_chris.jpg",
        },
        Artista {
            slug: "mc-poze",
            nome: "MC Poze do Rodo",
            genero: "funk",
            origem: "nacional",
            descricao: "Um dos maiores nomes do funk carioca, voz forte das comunidades.",
            imagem: "/static/img/mc_poze.jpg",
        },
        Artista {
            slug: "anitta",
            nome: "Anitta",
            genero: "funk",
            origem: "nacional",
            descricao: "Artista global que levou o funk brasileiro para os maiores palcos do mundo.",
            imagem: "/static/img/anitta.jpg",
        },
        // Funk / urbano internacional
        Artista {
            slug: "rosalia",
            nome: "ROSALÍA",
            genero: "funk",
            origem: "internacional",
            descricao: "Artista espanhola que mistura urbano, reggaeton e experimental, visualmente muito forte.",
            imagem: "/static/img/rosalia.jpg",
        },
        Artista {
            slug: "bad-bunny",
            nome: "Bad Bunny",
            genero: "funk",
            origem: "internacional",
            descricao: "Um dos maiores nomes da música latina e urbana, dono de grandes turnês mundiais.",
            imagem: "/static/img/bad_bunny.jpg",
        },
        // Rap nacional
        Artista {
            slug: "djonga",
            nome: "Djonga",
            genero: "rap",
            origem: "nacional",
            descricao: "Letras fortes e shows intensos, referência em rap nacional contemporâneo.",
            imagem: "/static/img/djonga.jpg",
        },
        Artista {
            slug: "racionais",
            nome: "Racionais MC's",
            genero: "rap",
            origem: "nacional",
            descricao: "Clássicos absolutos do rap brasileiro, influência para gerações inteiras.",
            imagem: "/static/img/racionais.jpg",
        },
        Artista {
            slug: "bk",
            nome: "BK'",
            genero: "rap",
            origem: "nacional",
            descricao: "Rap com estética apurada, letras introspectivas e shows muito bem produzidos.",
            imagem: "/static/img/bk.jpg",
        },
        // Rap internacional
        Artista {
            slug: "kendrick-lamar",
            nome: "Kendrick Lamar",
            genero: "rap",
            origem: "internacional",
            descricao: "Um dos maiores nomes do rap mundial, discos premiados e performances conceituais.",
            imagem: "/static/img/kendrick_lamar.jpg",
        },
        Artista {
            slug: "eminem",
            nome: "Eminem",
            genero: "rap",
            origem: "internacional",
            descricao: "Lenda do rap, dono de clássicos que atravessam gerações.",
            imagem: "/static/img/eminem.jpg",
        },
        Artista {
            slug: "drake",
            nome: "Drake",
            genero: "rap",
            origem: "internacional",
            descricao: "Rap, R&B e pop em uma mistura que domina paradas mundiais e estádios lotados.",
            imagem: "/static/img/drake.jpg",
        },
    ]
}

fn base_css() -> &'static str {
    r#"
    :root {
        color-scheme: dark;
        --purple-500: #6C3BFF;
        --purple-400: #8E4CFF;
        --purple-300: #B686FF;
        --purple-200: #D8C0FF;
        --blue-neon: #4F46E5;
        --magenta: #FF00D4;
        --bg-main: #050014;
        --bg-card: #12002E;
        --bg-soft: #141126;
        --text-main: #F9FAFB;
        --text-muted: #9CA3AF;
        --border-soft: rgba(148, 163, 184, 0.35);
    }

    * {
        box-sizing: border-box;
        margin: 0;
        padding: 0;
    }

    body {
        font-family: system-ui, -apple-system, BlinkMacSystemFont, "SF Pro Text", "Inter", sans-serif;
        background: radial-gradient(circle at top, #1E1B4B, #020016 55%, #000 100%);
        color: var(--text-main);
        min-height: 100vh;
    }

    a {
        color: inherit;
        text-decoration: none;
    }

    .shell {
        max-width: 1200px;
        margin: 0 auto;
        padding: 20px 16px 48px;
    }

    .nav {
        display: flex;
        align-items: center;
        justify-content: space-between;
        padding: 14px 18px;
        border-radius: 999px;
        border: 1px solid var(--border-soft);
        background: radial-gradient(circle at top left, rgba(255,255,255,0.08), rgba(15,23,42,0.92));
        backdrop-filter: blur(18px);
        position: sticky;
        top: 16px;
        z-index: 20;
    }

    .logo-mark {
        display: flex;
        flex-direction: column;
        gap: 2px;
    }

    .logo-small {
        font-size: 0.65rem;
        letter-spacing: 0.18em;
        text-transform: uppercase;
        color: var(--purple-200);
    }

    .logo-main {
        font-weight: 800;
        font-size: 0.9rem;
        letter-spacing: 0.22em;
    }

    .nav-links {
        display: flex;
        gap: 12px;
        align-items: center;
        font-size: 0.8rem;
    }

    .pill-nav {
        padding: 6px 12px;
        border-radius: 999px;
        border: 1px solid var(--border-soft);
        display: inline-flex;
        align-items: center;
        gap: 6px;
        color: var(--text-muted);
    }

    .pill-nav .dot {
        width: 6px;
        height: 6px;
        border-radius: 999px;
        background: radial-gradient(circle at center, var(--magenta), var(--purple-500));
        box-shadow: 0 0 12px rgba(236, 72, 153, 0.8);
    }

    .link {
        padding: 6px 12px;
        border-radius: 999px;
        border: 1px solid transparent;
        color: var(--text-muted);
        cursor: pointer;
        transition: all 0.16s ease-out;
    }

    .link:hover {
        border-color: rgba(148,163,184,0.6);
        color: var(--text-main);
        background: rgba(15,23,42,0.9);
    }

    .link.active {
        border-color: rgba(180, 83, 246, 0.9);
        color: var(--purple-200);
        background: radial-gradient(circle at top left, rgba(180,83,246,0.4), rgba(15,23,42,0.96));
        box-shadow: 0 0 18px rgba(129, 140, 248, 0.6);
    }

    .search-bar {
        width: 100%;
        padding: 10px 4px 0;
    }

    .search-bar form {
        width: 100%;
    }

    .search-bar input {
        width: 100%;
        padding: 10px 12px;
        border-radius: 12px;
        border: 1px solid rgba(148,163,184,0.7);
        background: rgba(15,23,42,0.95);
        color: var(--text-main);
        font-size: 0.85rem;
    }

    .search-bar input::placeholder {
        color: rgba(148,163,184,0.8);
    }

    .hero {
        margin-top: 26px;
        display: grid;
        grid-template-columns: minmax(0, 1.6fr) minmax(0, 1.1fr);
        gap: 20px;
        align-items: stretch;
    }

    .hero-card {
        border-radius: 26px;
        padding: 20px 22px 22px;
        background:
            radial-gradient(circle at top left, rgba(236,72,153,0.26), transparent 55%),
            radial-gradient(circle at bottom right, rgba(59,130,246,0.25), transparent 55%),
            linear-gradient(135deg, rgba(15,23,42,0.98), rgba(15,23,42,0.85));
        border: 1px solid rgba(148,163,184,0.7);
        box-shadow: 0 24px 60px rgba(0,0,0,0.9);
    }

    .hero-kicker {
        font-size: 0.7rem;
        letter-spacing: 0.18em;
        text-transform: uppercase;
        color: var(--purple-200);
    }

    .hero-title {
        margin-top: 10px;
        font-size: 1.7rem;
        line-height: 1.25;
    }

    .hero-sub {
        margin-top: 8px;
        font-size: 0.9rem;
        color: var(--text-muted);
        max-width: 460px;
    }

    .hero-meta {
        margin-top: 18px;
        display: flex;
        flex-wrap: wrap;
        gap: 10px;
        font-size: 0.8rem;
        color: var(--text-muted);
    }

    .hero-meta .badge {
        padding: 4px 9px;
        border-radius: 999px;
        border: 1px solid rgba(148,163,184,0.5);
        background: rgba(15,23,42,0.9);
    }

    .hero-cta-row {
        margin-top: 22px;
        display: flex;
        gap: 10px;
        flex-wrap: wrap;
        align-items: center;
    }

    .btn-primary {
        padding: 10px 20px;
        border-radius: 999px;
        border: 1px solid transparent;
        background: linear-gradient(135deg, var(--magenta), var(--purple-500));
        color: white;
        font-size: 0.85rem;
        font-weight: 500;
        cursor: pointer;
        transition: all 0.16s ease-out;
    }

    .btn-primary:hover {
        transform: translateY(-1px);
        box-shadow: 0 18px 40px rgba(0,0,0,0.9);
    }

    .btn-ghost {
        padding: 9px 16px;
        border-radius: 999px;
        border: 1px solid rgba(148,163,184,0.7);
        background: rgba(15,23,42,0.9);
        color: var(--text-main);
        font-size: 0.8rem;
        cursor: pointer;
    }

    .btn-small {
        padding-inline: 14px;
        padding-block: 6px;
        font-size: 0.75rem;
    }

    .hero-right {
        border-radius: 24px;
        padding: 18px 18px 20px;
        background: radial-gradient(circle at top, rgba(255,255,255,0.12), rgba(5,0,20,0.96));
        border: 1px solid var(--border-soft);
        display: flex;
        flex-direction: column;
        gap: 10px;
        font-size: 0.8rem;
    }

    .hero-right-header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        color: var(--text-muted);
    }

    .hero-right-event {
        font-size: 0.9rem;
        font-weight: 500;
    }

    .hero-right-chip {
        font-size: 0.7rem;
        padding: 3px 8px;
        border-radius: 999px;
        border: 1px solid rgba(148,163,184,0.6);
    }

    .hero-right-main {
        margin-top: 6px;
        padding: 10px;
        border-radius: 18px;
        background: rgba(5,5,12,0.9);
        display: flex;
        flex-direction: column;
        gap: 8px;
    }

    .hero-right-main strong {
        color: var(--purple-200);
    }

    .hero-right-list {
        margin-top: 8px;
        font-size: 0.78rem;
        color: var(--text-muted);
    }

    .hero-right-list ul {
        margin-left: 16px;
        margin-top: 4px;
    }

    .section {
        margin-top: 34px;
    }

    .section-header {
        display: flex;
        justify-content: space-between;
        align-items: flex-end;
        gap: 10px;
    }

    .section-title {
        font-size: 1.05rem;
        font-weight: 600;
    }

    .section-sub {
        font-size: 0.8rem;
        color: var(--text-muted);
    }

    .artist-grid {
        margin-top: 18px;
        display: grid;
        grid-template-columns: repeat(4, minmax(0, 1fr));
        gap: 14px;
    }

    .artist-card {
        border-radius: 18px;
        padding: 10px 10px 12px;
        background: linear-gradient(145deg, rgba(15,23,42,0.96), rgba(15,23,42,0.88));
        border: 1px solid rgba(148,163,184,0.4);
        cursor: pointer;
        transition: transform 0.16s ease-out, box-shadow 0.16s ease-out, border-color 0.16s;
        display: flex;
        flex-direction: column;
        gap: 6px;
        box-shadow: 0 18px 40px rgba(0,0,0,0.85);
    }

    .artist-card:hover {
        transform: translateY(-3px);
        border-color: rgba(180,83,246,0.9);
        box-shadow: 0 26px 70px rgba(0,0,0,0.95);
    }

    .artist-thumb {
        width: 100%;
        aspect-ratio: 4 / 3;
        border-radius: 14px;
        background-position: center;
        background-size: cover;
        background-repeat: no-repeat;
        margin-bottom: 6px;
    }

    .artist-name {
        font-size: 0.9rem;
        font-weight: 600;
    }

    .artist-tagline {
        font-size: 0.75rem;
        color: var(--text-muted);
    }

    .pill-origin {
        display: inline-flex;
        align-items: center;
        gap: 4px;
        font-size: 0.7rem;
        padding: 2px 7px;
        border-radius: 999px;
        border: 1px solid rgba(148,163,184,0.6);
        color: var(--text-muted);
        margin-top: 4px;
    }

    .pill-origin .dot {
        width: 5px;
        height: 5px;
        border-radius: 999px;
        background: radial-gradient(circle at center, var(--blue-neon), var(--magenta));
    }

    /* Página de gênero */

    .genre-layout {
        margin-top: 30px;
        display: grid;
        grid-template-columns: minmax(0, 2.1fr) minmax(0, 1.3fr);
        gap: 24px;
        align-items: flex-start;
    }

    .genre-sidebar {
        border-radius: 22px;
        padding: 16px 16px 18px;
        background: radial-gradient(circle at top, rgba(255,255,255,0.12), rgba(5,0,20,0.96));
        border: 1px solid var(--border-soft);
        font-size: 0.82rem;
        color: var(--text-muted);
    }

    .genre-sidebar h2 {
        margin-bottom: 6px;
        font-size: 0.9rem;
    }

    .genre-sidebar ul {
        margin: 6px 0 0;
        padding-left: 18px;
    }

    .genre-sidebar li {
        margin-bottom: 4px;
    }

    /* Página de artista */

    .artist-layout {
        margin-top: 28px;
        display: grid;
        grid-template-columns: minmax(0, 1.9fr) minmax(0, 1.2fr);
        gap: 24px;
        align-items: flex-start;
    }

    .artist-cover {
        border-radius: 22px;
        padding: 16px 16px 18px;
        border: 1px solid var(--border-soft);
        background:
            radial-gradient(circle at top left, rgba(236,72,153,0.28), transparent 55%),
            linear-gradient(145deg, rgba(15,23,42,0.97), rgba(15,23,42,0.9));
        display: flex;
        gap: 16px;
    }

    .artist-cover-image {
        width: 40%;
        border-radius: 18px;
        background-position: center;
        background-size: cover;
        background-repeat: no-repeat;
        aspect-ratio: 4/3;
    }

    .artist-cover-main {
        flex: 1;
        display: flex;
        flex-direction: column;
        gap: 8px;
        justify-content: space-between;
    }

    .artist-cover-main h1 {
        font-size: 1.4rem;
    }

    .artist-cover-main p {
        font-size: 0.85rem;
        color: var(--text-muted);
    }

    .artist-meta-row {
        display: flex;
        gap: 10px;
        flex-wrap: wrap;
        font-size: 0.78rem;
        color: var(--text-muted);
    }

    .artist-meta-tag {
        padding: 4px 9px;
        border-radius: 999px;
        border: 1px solid rgba(148,163,184,0.6);
        background: rgba(15,23,42,0.9);
    }

    .artist-aside {
        border-radius: 20px;
        padding: 14px 16px 16px;
        background: rgba(10,10,20,0.96);
        border: 1px dashed rgba(148,163,184,0.6);
        font-size: 0.8rem;
        color: var(--text-muted);
    }

    /* Telas de autenticação */

    .auth-layout {
        margin-top: 32px;
        display: grid;
        grid-template-columns: minmax(0, 1.3fr) minmax(0, 1.1fr);
        gap: 24px;
        align-items: stretch;
    }

    .auth-card {
        border-radius: 24px;
        padding: 20px 20px 22px;
        background: radial-gradient(circle at top, rgba(255,255,255,0.14), rgba(5,0,20,0.96));
        border: 1px solid var(--border-soft);
        display: flex;
        flex-direction: column;
        gap: 14px;
        font-size: 0.9rem;
    }

    .auth-card p {
        margin: 0;
        color: var(--text-muted);
        font-size: 0.8rem;
    }

    .auth-form {
        margin-top: 8px;
        display: flex;
        flex-direction: column;
        gap: 10px;
    }

    .auth-form label {
        font-size: 0.75rem;
        color: var(--text-muted);
    }

    .auth-form input {
        width: 100%;
        padding: 9px 11px;
        border-radius: 10px;
        border: 1px solid rgba(148,163,184,0.55);
        background: rgba(15,23,42,0.95);
        color: var(--text-main);
        font-size: 0.85rem;
    }

    .auth-form input::placeholder {
        color: rgba(148,163,184,0.75);
    }

    .auth-form .row-inline {
        display: flex;
        justify-content: space-between;
        align-items: center;
        font-size: 0.75rem;
        color: var(--text-muted);
        gap: 8px;
    }

    .auth-form .link-muted {
        color: var(--purple-300);
        cursor: pointer;
        text-decoration: underline;
    }

    .auth-badge {
        font-size: 0.7rem;
        padding: 4px 9px;
        border-radius: 999px;
        border: 1px solid rgba(148,163,184,0.45);
        background: rgba(15,23,42,0.9);
        display: inline-flex;
        align-items: center;
        gap: 6px;
    }

    .auth-legend {
        font-size: 0.78rem;
        color: var(--text-muted);
        line-height: 1.5;
    }

    .auth-highlight {
        font-weight: 600;
        color: var(--purple-200);
    }

    /* Página de ingressos */

    .tickets-layout {
        margin-top: 30px;
        display: grid;
        grid-template-columns: minmax(0, 1.7fr) minmax(0, 1.1fr);
        gap: 24px;
        align-items: flex-start;
    }

    .tickets-list {
        display: flex;
        flex-direction: column;
        gap: 12px;
    }

    .ticket-card {
        border-radius: 18px;
        padding: 12px 14px;
        background: linear-gradient(135deg, rgba(17,24,39,0.98), rgba(88,28,135,0.85));
        border: 1px solid rgba(148,163,184,0.55);
        display: flex;
        justify-content: space-between;
        gap: 14px;
        align-items: center;
        box-shadow: 0 18px 50px rgba(0,0,0,0.85);
    }

    .ticket-main {
        display: flex;
        flex-direction: column;
        gap: 4px;
        font-size: 0.8rem;
    }

    .ticket-artist {
        font-size: 0.95rem;
        font-weight: 600;
    }

    .ticket-meta {
        color: var(--text-muted);
        font-size: 0.75rem;
    }

    .ticket-venue {
        font-size: 0.75rem;
        color: var(--purple-200);
    }

    .ticket-side {
        text-align: right;
        display: flex;
        flex-direction: column;
        gap: 6px;
        font-size: 0.8rem;
    }

    .ticket-price {
        font-weight: 600;
    }

    .tickets-sidebar {
        border-radius: 22px;
        padding: 18px 18px 20px;
        background: radial-gradient(circle at top, rgba(255,255,255,0.18), rgba(5,0,20,0.96));
        border: 1px solid var(--border-soft);
        font-size: 0.8rem;
        color: var(--text-muted);
        display: flex;
        flex-direction: column;
        gap: 10px;
    }

    .tickets-sidebar h2 {
        margin: 0;
        font-size: 0.95rem;
    }

    .tickets-sidebar ul {
        margin: 4px 0 0;
        padding-left: 18px;
    }

    .tickets-sidebar li {
        margin-bottom: 4px;
    }

    .footer {
        margin-top: 40px;
        padding-top: 14px;
        border-top: 1px dashed rgba(148,163,184,0.5);
        font-size: 0.74rem;
        color: var(--text-muted);
        display: flex;
        flex-wrap: wrap;
        gap: 8px;
        justify-content: space-between;
    }

    @media (max-width: 960px) {
        .nav {
            flex-direction: column;
            gap: 10px;
            align-items: flex-start;
        }
        .hero {
            grid-template-columns: minmax(0, 1fr);
        }
        .artist-grid {
            grid-template-columns: repeat(2, minmax(0, 1fr));
        }
        .genre-layout,
        .artist-layout,
        .auth-layout,
        .tickets-layout {
            grid-template-columns: minmax(0, 1fr);
        }
        .artist-cover {
            flex-direction: column;
        }
        .artist-cover-image {
            width: 100%;
        }
    }
    "#
}

fn artist_card(a: &Artista) -> String {
    let genero_label = match a.genero {
        "trap" => "Trap",
        "funk" => "Funk / Urbano",
        "rap" => "Rap",
        _ => "Artista",
    };

    let origem_label = match a.origem {
        "nacional" => "Brasil",
        "internacional" => "Mundo",
        _ => "Cena global",
    };

    format!(
        r#"<a href="/artista/{slug}">
<div class="artist-card">
    <div class="artist-thumb" style="background-image:url('{img}');"></div>
    <div class="artist-name">{nome}</div>
    <div class="artist-tagline">{genero_label}</div>
    <div class="pill-origin">
        <span class="dot"></span>
        <span>{origem_label}</span>
    </div>
</div>
</a>"#,
        slug = a.slug,
        img = a.imagem,
        nome = a.nome,
        genero_label = genero_label,
        origem_label = origem_label
    )
}

fn layout(title: &str, body: String, active: &str) -> Html<String> {
    let css = base_css();

    let nav_home = if active == "home" { "link active" } else { "link" };
    let nav_trap = if active == "trap" { "link active" } else { "link" };
    let nav_funk = if active == "funk" { "link active" } else { "link" };
    let nav_rap = if active == "rap" { "link active" } else { "link" };
    let nav_login = if active == "login" { "link active" } else { "link" };
    let nav_ingressos = if active == "ingressos" { "link active" } else { "link" };

    Html(format!(
        r#"<!DOCTYPE html>
<html lang="pt-BR">
<head>
    <meta charset="utf-8" />
    <title>{title}</title>
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <style>{css}</style>
</head>
<body>
    <div class="shell">
        <nav class="nav">
            <div class="logo-mark">
                <span class="logo-small">ONE MORE</span>
                <span class="logo-main">SHOW</span>
            </div>
            <div class="nav-links">
                <div class="pill-nav">
                    <span class="dot"></span>
                    <span>Evento digital • Tech Week</span>
                </div>
                <a href="/" class="{nav_home}">Início</a>
                <a href="/genero/trap" class="{nav_trap}">Trap</a>
                <a href="/genero/funk" class="{nav_funk}">Funk</a>
                <a href="/genero/rap" class="{nav_rap}">Rap</a>
                <a href="/ingressos" class="{nav_ingressos}">Ingressos</a>
                <a href="/login" class="{nav_login}">Login</a>
            </div>
        </nav>

        <div class="search-bar">
            <form action="/buscar" method="get">
                <input type="text" name="q" placeholder="Buscar artista..." />
            </form>
        </div>

        {body}

        <footer class="footer">
            <span>ONE MORE SHOW • Projeto conceitual para a sua Tech Week.</span>
            <span>Layout em Rust + Axum • Paleta roxo neon.</span>
        </footer>
    </div>
</body>
</html>
"#,
        title = title,
        css = css,
        nav_home = nav_home,
        nav_trap = nav_trap,
        nav_funk = nav_funk,
        nav_rap = nav_rap,
        nav_login = nav_login,
        nav_ingressos = nav_ingressos,
        body = body,
    ))
}

async fn home() -> Html<String> {
    let evs = eventos();
    let arts = artistas();

    let evento = &evs[0];

    let mut destaques_trap = String::new();
    for a in arts.iter().filter(|a| a.genero == "trap").take(4) {
        destaques_trap.push_str(&artist_card(a));
    }

    let mut destaques_funk = String::new();
    for a in arts.iter().filter(|a| a.genero == "funk").take(4) {
        destaques_funk.push_str(&artist_card(a));
    }

    let mut destaques_rap = String::new();
    for a in arts.iter().filter(|a| a.genero == "rap").take(4) {
        destaques_rap.push_str(&artist_card(a));
    }

    let body = format!(
        r#"
<section class="hero">
    <div class="hero-card">
        <div class="hero-kicker">{nome}</div>
        <h1 class="hero-title">Um festival para maratonar seus artistas favoritos.</h1>
        <p class="hero-sub">
            Trap, funk e rap em um palco digital. Monte sua própria line-up e explore os artistas
            que você levaria para o ONE MORE SHOW.
        </p>
        <div class="hero-meta">
            <span class="badge">{data}</span>
            <span class="badge">{local}</span>
            <span class="badge">Acesso online • Full HD</span>
        </div>
        <div class="hero-cta-row">
            <a href="/genero/trap"><button class="btn-primary">Explorar artistas</button></a>
            <a href="/ingressos"><button class="btn-ghost">Ver ingressos fictícios</button></a>
        </div>
    </div>
    <aside class="hero-right">
        <div class="hero-right-header">
            <div>
                <div class="hero-right-event">Experiência guiada para Tech Week</div>
                <div class="section-sub">Simulação de plataforma de festival digital.</div>
            </div>
            <span class="hero-right-chip">Demo interativa</span>
        </div>
        <div class="hero-right-main">
            <span><strong>Como usar:</strong></span>
            <div class="hero-right-list">
                <ul>
                    <li>Escolha um gênero (Trap, Funk ou Rap) na navegação;</li>
                    <li>Clique em um artista para ver a página detalhada;</li>
                    <li>Acesse a aba <strong>Ingressos</strong> para simular compra;</li>
                    <li>Use a página <strong>Login</strong> para mostrar telas de cadastro.</li>
                </ul>
            </div>
        </div>
    </aside>
</section>

<section class="section">
    <div class="section-header">
        <div>
            <div class="section-title">Trap • BR & mundo</div>
            <div class="section-sub">Beats pesados, visuais futuristas e muita energia.</div>
        </div>
        <a href="/genero/trap" class="section-sub">Ver todos &rarr;</a>
    </div>
    <div class="artist-grid">
        {destaques_trap}
    </div>
</section>

<section class="section">
    <div class="section-header">
        <div>
            <div class="section-title">Funk & urbano</div>
            <div class="section-sub">Do baile ao palco global.</div>
        </div>
        <a href="/genero/funk" class="section-sub">Ver todos &rarr;</a>
    </div>
    <div class="artist-grid">
        {destaques_funk}
    </div>
</section>

<section class="section">
    <div class="section-header">
        <div>
            <div class="section-title">Rap • BR & mundo</div>
            <div class="section-sub">Clássicos e novos nomes lado a lado.</div>
        </div>
        <a href="/genero/rap" class="section-sub">Ver todos &rarr;</a>
    </div>
    <div class="artist-grid">
        {destaques_rap}
    </div>
</section>
"#,
        nome = evento.nome,
        data = evento.data,
        local = evento.local,
        destaques_trap = destaques_trap,
        destaques_funk = destaques_funk,
        destaques_rap = destaques_rap,
    );

    layout("Início • ONE MORE SHOW", body, "home")
}

async fn pagina_genero(Path(slug): Path<String>) -> Html<String> {
    let genero = slug.as_str();

    let titulo_genero = match genero {
        "trap" => "Trap • Experimentos e megashows",
        "funk" => "Funk & Urbano • Do baile ao mundo",
        "rap" => "Rap • Clássicos e nova geração",
        _ => "Line-up",
    };

    let subtitulo = match genero {
        "trap" => "Beats pesados, luzes neon e shows cheios de efeitos especiais.",
        "funk" => "Brasil, América Latina e o mundo em ritmo de baile e festa.",
        "rap" => "Letras fortes, storytelling e performances históricas.",
        _ => "Escolha artistas para montar sua noite perfeita.",
    };

    let arts = artistas();
    let mut cards = String::new();

    for a in arts.iter().filter(|a| a.genero == genero) {
        cards.push_str(&artist_card(a));
    }

    let body = format!(
        r#"
<section class="section">
    <div class="section-header">
        <div>
            <div class="section-title">{titulo}</div>
            <div class="section-sub">{sub}</div>
        </div>
    </div>
    <div class="genre-layout">
        <div>
            <div class="artist-grid">
                {cards}
            </div>
        </div>
        <aside class="genre-sidebar">
            <h2>Curadoria One More Show</h2>
            <p>Esta tela ajuda a explicar como funcionaria uma página de gênero dentro de uma plataforma de festival.</p>
            <ul>
                <li>Artistas organizados por gênero musical;</li>
                <li>Cards clicáveis que levam para páginas individuais;</li>
                <li>Visual consistente com a home e com a identidade neon.</li>
            </ul>
            <p>Você pode usar esta tela na Tech Week para mostrar como pensar experiência do usuário em eventos digitais.</p>
        </aside>
    </div>
</section>
"#,
        titulo = titulo_genero,
        sub = subtitulo,
        cards = cards
    );

    let active = match genero {
        "trap" => "trap",
        "funk" => "funk",
        "rap" => "rap",
        _ => "home",
    };

    layout("Gênero • ONE MORE SHOW", body, active)
}

async fn pagina_artista(Path(slug): Path<String>) -> Html<String> {
    let arts = artistas();
    let artista = match arts.into_iter().find(|a| a.slug == slug) {
        Some(a) => a,
        None => {
            let body = r#"
<section class="section">
    <div class="section-header">
        <div>
            <div class="section-title">Artista não encontrado</div>
            <div class="section-sub">Tente voltar para a home e escolher outro card.</div>
        </div>
    </div>
</section>
"#
            .to_string();
            return layout("Artista • ONE MORE SHOW", body, "home");
        }
    };

    let genero_label = match artista.genero {
        "trap" => "Trap",
        "funk" => "Funk / Urbano",
        "rap" => "Rap",
        _ => "Artista",
    };

    let origem_label = match artista.origem {
        "nacional" => "Brasil",
        "internacional" => "Cena internacional",
        _ => "Cena global",
    };

    let body = format!(
        r#"
<section class="section">
    <div class="artist-layout">
        <div class="artist-cover">
            <div class="artist-cover-image" style="background-image:url('{img}');"></div>
            <div class="artist-cover-main">
                <div>
                    <h1>{nome}</h1>
                    <p>{descricao}</p>
                </div>
                <div class="artist-meta-row">
                    <div class="artist-meta-tag">{genero}</div>
                    <div class="artist-meta-tag">{origem}</div>
                    <div class="artist-meta-tag">Presente no ONE MORE SHOW</div>
                </div>
            </div>
        </div>
        <aside class="artist-aside">
            <strong>Ideia para Tech Week:</strong>
            <p>
                Use esta página para explicar como ficaria a ficha detalhada de um artista dentro da
                sua plataforma: imagem em destaque, descrição, gênero, origem e destaques do show.
            </p>
            <p>
                Você pode conectar esta tela à página de ingressos, simulando seleção de datas em que
                este artista se apresenta no festival.
            </p>
        </aside>
    </div>
</section>
"#,
        img = artista.imagem,
        nome = artista.nome,
        descricao = artista.descricao,
        genero = genero_label,
        origem = origem_label
    );

    let active = match artista.genero {
        "trap" => "trap",
        "funk" => "funk",
        "rap" => "rap",
        _ => "home",
    };

    layout("Artista • ONE MORE SHOW", body, active)
}

async fn buscar(Query(params): Query<HashMap<String, String>>) -> Html<String> {
    let q = params
        .get("q")
        .cloned()
        .unwrap_or_default()
        .to_lowercase();

    let mut out = String::from(
        r#"<section class="section">
    <div class="section-header">
        <div>
            <div class="section-title">Resultados da busca</div>
            <div class="section-sub">Clique em um artista para abrir a página detalhada.</div>
        </div>
    </div>
"#,
    );

    let mut count = 0usize;
    for a in artistas() {
        if a.nome.to_lowercase().contains(&q) {
            count += 1;
            out.push_str(&format!(
                r#"<p><a href="/artista/{slug}" class="link">{nome}</a></p>"#,
                slug = a.slug,
                nome = a.nome
            ));
        }
    }

    if count == 0 {
        out.push_str("<p class=\"section-sub\">Nenhum artista encontrado para essa busca.</p>");
    }

    out.push_str("</section>");

    layout("Busca • ONE MORE SHOW", out, "home")
}

async fn pagina_login() -> Html<String> {
    let body = r#"
<section class="section">
    <div class="section-header">
        <div>
            <div class="section-title">Entrar ou criar conta</div>
            <div class="section-sub">Simulação de login para o festival ONE MORE SHOW.</div>
        </div>
        <span class="section-sub">Somente layout • Nenhum dado é realmente salvo.</span>
    </div>
    <div class="auth-layout">
        <div class="auth-card">
            <div class="section-title" style="font-size:1.1rem;">Login</div>
            <p>Acesse sua conta fictícia para visualizar pedidos, QR Codes e preferências de line-up.</p>
            <form class="auth-form">
                <div>
                    <label for="login-email">E-mail</label>
                    <input id="login-email" type="email" placeholder="seuemail@exemplo.com" />
                </div>
                <div>
                    <label for="login-senha">Senha</label>
                    <input id="login-senha" type="password" placeholder="••••••••" />
                </div>
                <div class="row-inline">
                    <label>
                        <input type="checkbox" style="width:auto;"> Lembrar de mim
                    </label>
                    <span class="link-muted">Esqueci minha senha</span>
                </div>
                <button type="button" class="btn-primary">Entrar</button>
            </form>
        </div>
        <div class="auth-card">
            <span class="auth-badge">Novo por aqui?</span>
            <div class="section-title" style="font-size:1.1rem; margin-top:4px;">Criar conta</div>
            <p>Cadastre-se para receber novidades, pré-venda exclusiva e experiências no ONE MORE SHOW.</p>
            <form class="auth-form">
                <div>
                    <label for="cadastro-nome">Nome completo</label>
                    <input id="cadastro-nome" type="text" placeholder="Seu nome" />
                </div>
                <div>
                    <label for="cadastro-email">E-mail</label>
                    <input id="cadastro-email" type="email" placeholder="voce@exemplo.com" />
                </div>
                <div>
                    <label for="cadastro-senha">Criar senha</label>
                    <input id="cadastro-senha" type="password" placeholder="Mínimo 8 caracteres" />
                </div>
                <div class="row-inline">
                    <span class="auth-legend">
                        Ao continuar, você concorda com os <span class="auth-highlight">termos de uso</span> 
                        e a <span class="auth-highlight">política de privacidade</span> fictícios da plataforma.
                    </span>
                </div>
                <button type="button" class="btn-primary">Criar conta</button>
            </form>
        </div>
    </div>
</section>
"#
    .to_string();

    layout("Login • ONE MORE SHOW", body, "login")
}

async fn pagina_ingressos() -> Html<String> {
    let arts = artistas();
    let mut ingressos_html = String::new();

    for (i, a) in arts.iter().enumerate() {
        let (cidade, casa) = match (a.genero, a.origem) {
            ("trap", "nacional") => ("São Paulo, SP", "Allianz Parque"),
            ("trap", "internacional") => ("Rio de Janeiro, RJ", "Jeunesse Arena"),
            ("funk", "nacional") => ("Rio de Janeiro, RJ", "Praça da Apoteose"),
            ("funk", "internacional") => ("São Paulo, SP", "Neo Química Arena"),
            ("rap", "nacional") => ("Belo Horizonte, MG", "Mineirão"),
            ("rap", "internacional") => ("São Paulo, SP", "Estádio do Morumbi"),
            _ => ("São Paulo, SP", "Arena One More Show"),
        };

        let data = match i % 4 {
            0 => "21/11/2025 • 21h00",
            1 => "22/11/2025 • 21h00",
            2 => "23/11/2025 • 20h00",
            _ => "24/11/2025 • 20h00",
        };

        let preco_base: f32 = match a.origem {
            "nacional" => 180.0,
            "internacional" => 420.0,
            _ => 250.0,
        };

        ingressos_html.push_str(&format!(
            r#"
<div class="ticket-card">
    <div class="ticket-main">
        <div class="ticket-artist">{nome}</div>
        <div class="ticket-meta">{data} • {cidade}</div>
        <div class="ticket-venue">{casa}</div>
    </div>
    <div class="ticket-side">
        <div class="ticket-price">a partir de R$ {preco:.2}</div>
        <button class="btn-primary btn-small" type="button">Selecionar ingresso</button>
    </div>
</div>
"#,
            nome = a.nome,
            data = data,
            cidade = cidade,
            casa = casa,
            preco = preco_base
        ));
    }

    let body = format!(
        r#"
<section class="section">
    <div class="section-header">
        <div>
            <div class="section-title">Ingressos • ONE MORE SHOW</div>
            <div class="section-sub">Escolha uma data em São Paulo, Rio, BH e outros palcos icônicos.</div>
        </div>
        <span class="section-sub">Simulação de compra • Valores e datas fictícios.</span>
    </div>
    <div class="tickets-layout">
        <div class="tickets-list">
            {ingressos}
        </div>
        <aside class="tickets-sidebar">
            <h2>Como funciona esta página?</h2>
            <p>
                Esta é uma vitrine fictícia que simula a jornada de compra de um festival real.
                Você pode usar este layout na sua Tech Week para explicar:
            </p>
            <ul>
                <li>Como organizar line-up por cidade e data;</li>
                <li>Como destacar preços e setores de forma clara;</li>
                <li>Como criar uma sidebar educativa com avisos importantes.</li>
            </ul>
            <p>
                Em uma versão com backend, cada botão <strong>Selecionar ingresso</strong> abriria
                um fluxo de assentos, meios de pagamento e confirmação.</p>
            <p>
                Aqui, o foco é o visual: paleta roxo neon, gradientes e cards que seguem o mesmo
                conceito da home do ONE MORE SHOW.</p>
        </aside>
    </div>
</section>
"#,
        ingressos = ingressos_html
    );

    layout("Ingressos • ONE MORE SHOW", body, "ingressos")
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(home))
        .route("/login", get(pagina_login))
        .route("/ingressos", get(pagina_ingressos))
        .route("/genero/:slug", get(pagina_genero))
        .route("/buscar", get(buscar))
        .route("/artista/:slug", get(pagina_artista))
        .nest_service("/static", ServeDir::new("static"));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Servidor rodando em http://{addr}");

    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

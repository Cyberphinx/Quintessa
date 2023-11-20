<script lang="ts">
    import type { SignedMedia } from "$lib/model";
    import { cvMedia } from "$lib/stores/commonStore";
    import { supabaseCredentials } from "$lib/stores/supabaseStore";

    $: avatar =
        $cvMedia.length > 0
            ? `${$supabaseCredentials.url}/storage/v1${
                  $cvMedia.find((x) => x.path === "quintessa/profile.jpg")
                      ?.signedURL
              }`
            : "/assets/Placeholder.svg";
    let snippets: any = {};
    $: $cvMedia.map((x: SignedMedia) => {
        snippets[
            x.path
        ] = `${$supabaseCredentials.url}/storage/v1${x.signedURL}`;
    });

    const captions = [
        "Fig.1 Office building core floorplan (Autodesk Revit)",
        "Fig.2 Office facade construction details (Autodesk Revit)",
        "Fig.3 Office RCP with MEP services and structures (Autodesk Revit)",
        "Fig.4 Color coded building master-plan (Autodesk Revit)",
        "Fig.5 Academic project - Digital Rainforest Ark (Adobe AfterEffect)",
    ];
</script>

<div class="container">
    <div class="sidebar">
        <div class="sidebar-section">
            <img class="avatar" src={avatar} alt="profile" />
        </div>
        <div class="sidebar-section">
            <p class="sidebar-title">CONTACT</p>
            <p class="contact">
                Email:
                <a href="mailto:archon@tuta.io">archon@tuta.io</a>
            </p>
            <p class="contact">
                Mobile: <a href="tel:00447907606864">+44 (0) 7907 606864</a>
            </p>
            <p class="contact">
                Website:
                <a href="https://www.lingleng.co.uk" target="_blank"
                    >www.lingleng.co.uk</a
                >
            </p>
        </div>
        <div class="sidebar-section">
            <p class="sidebar-title">DoB</p>
            <p class="sidebar-content">March 1988</p>
        </div>
        <div class="sidebar-section">
            <p class="sidebar-title">NATIONALITY</p>
            <p class="sidebar-content">British</p>
        </div>
        <div class="sidebar-section">
            <p class="sidebar-title">TOP SKILLS</p>
            <p class="sidebar-content">Architecture</p>
            <p class="sidebar-content">Revit / BIM</p>
            <p class="sidebar-content">Typescript</p>
            <p class="sidebar-content">Rust-lang</p>
            <p class="sidebar-content">HTML</p>
            <p class="sidebar-content">CSS</p>
        </div>
        <div class="sidebar-section">
            <p class="sidebar-title">LANGUAGES</p>
            <p class="sidebar-content">English</p>
            <p class="sidebar-content">Chinese (Mandarin)</p>
        </div>
        <div class="sidebar-section">
            <p class="sidebar-title">CERTIFICATIONS</p>
            <p class="sidebar-content">ARB Qualified Architect</p>
            <p class="sidebar-content">Autodesk Revit Certified Professional</p>
            <p class="sidebar-content">CDM Training Certificate</p>
            <p class="sidebar-content">
                Asbestos Awareness for Architects and Designers
            </p>
        </div>
        <div class="sidebar-section">
            {#each Array(5) as _, i}
                <img
                    class="snippet"
                    src={snippets[`quintessa/snippet_0${i + 1}.png`]}
                    alt="profile"
                />
                <p class="snippet-caption">{captions[i]}</p>
            {/each}
        </div>
    </div>
    <div class="contents">
        <div class="section">
            <h1 class="name">Ling Leng</h1>
            <p class="title">
                ARB Qualified Architect, Revit/BIM Specialist, Full-stack
                developer
            </p>
            <p class="address">Welshpool, UK</p>
            <div class="contacts">
                <hr />
                <p class="section-title">CONTACT</p>
                <p class="contact">
                    Email:
                    <a href="mailto:archon@tuta.io">archon@tuta.io</a>
                </p>
                <p class="contact">
                    Mobile: <a href="tel:00447907606864">+44 (0) 7907 606864</a>
                </p>
            </div>
        </div>
        <div class="section">
            <p class="section-title">SUMMARY</p>
            <p class="subtitle-text">
                Multi-talented professional architect with 7 years of experience
                in architectural design and engineering industry, and 3 years of
                experience in full-stack web development.
            </p>
            <p class="subtitle">Architectural design skills:</p>
            <p class="subtitle-text">
                Strong design ability, with 3D modeling, architectural
                detailing, technical drafting, design coordination and
                presentation skills, along with specialism in BIM (Revit), NBS
                Create specification, Microsoft packages and Indesign for
                documentation and presentation. Proven track record of working
                on large and complex residential, commercial and industrial
                projects in London, ranging from Concept, Technical Design to
                Production Information and Construction, for packages including
                interiors, facade, metalwork, staircase, roof, partitions,
                landscaping, and robotic warehouse, etc.
            </p>
            <p class="subtitle">Full-stack web development skills:</p>
            <p class="subtitle-text">
                Experience in creating full-stack web application from scratch
                for real estate web portals, with skill-sets ranging from
                responsive UI design, front-end and back-end development to
                database integration and GitHub, Jira and Kanban collaborative
                workflow, mainly focusing on Typescript as front-end, and Rust
                as back-end languages, with Docker deployment knowledge on cloud
                platforms, along with the ability to write simple unit-tests.
                Technical specialism in web scraping and data mining using
                browser-automation, such as puppeteer and headless chromium.
                Practical experience setting up authentication middle-ware using
                JWT and auth0. Understanding the importance of clarity,
                readability and maintainability in code, following best practice
                principles, such as the Clean Architecture and Facade pattern.
            </p>
            <p class="subtitle-text">
                Tech-stack: <img
                    class="image"
                    src="/assets/Typescript.png"
                    alt="ts"
                />
                Typescript Sveltekit and React front-end,
                <img class="image" src="/assets/Rust.png" alt="rs" />
                Rust axum and
                <img class="image" src="/assets/Csharp.png" alt="cs" /> C# ASP.NET
                7 back-end, PostgreSQL database, plain CSS, HTML, and Docker
            </p>
            <p class="subtitle">Transferable soft skills:</p>
            <p class="subtitle-text">
                Adapative quick learner, always open and passionate to learn new
                technologies. Effiicent time management, effective team-working,
                and project management with respect to deadlines, as well as
                effective communication, collaboration and coordination skills
                between internal and external team members. Proven ability of
                working under stress, working well both within a team and
                individually. Project presentation skills to clients and
                contractors. Professional translation skills between English and
                Mandarin Chinese.
            </p>

            <br />
        </div>
        <div class="section">
            <p class="section-title">WORK</p>
            <div class="company">
                <p class="company-name">
                    <a href="https://sanctum.co.uk" target="_blank"
                        >Sanctum.co.uk</a
                    >
                </p>
                <p class="position">
                    Full-stack developer<span class="date"
                        >- November 2021 - Present (3 years)</span
                    >
                </p>
                <p class="location">Fully remote working, United Kingdom</p>
                <p class="project">
                    UI Design, frontend and backend development, database and
                    cloud deployment
                </p>
            </div>
            <div class="company">
                <p class="company-name">Ocado Engineering</p>
                <p class="position">
                    Senior Revit Technician<span class="date"
                        >- December 2020 - November 2021 (1 year)</span
                    >
                </p>
                <p class="location">Fully remote working, United Kingdom</p>
                <p class="project">
                    2021: Aeon CFC (Customer Fulfillment Centre), Japan
                </p>
                <p class="project">
                    2020 - 2021: Kroger CFC (Customer Fulfillment Centre), USA
                </p>
            </div>
            <div class="company">
                <p class="company-name">Allford Hall Monaghan Morris</p>
                <p class="position">
                    Architect<span class="date"
                        >- December 2017 - December 2020 (3 years)</span
                    >
                </p>
                <p class="location">London, England, United Kingdom</p>
                <p class="project">
                    2020: Arthur Stanley House, London, United Kingdom
                </p>
                <p class="project">
                    2019: Wood Wharf D3D4, London, United Kingdom
                </p>
                <p class="project">
                    2017 - 2018: Google Deepmind Headquarter, London, United
                    Kingdom
                </p>
                <br />
                <p class="position">
                    Architectural Assistant <span class="date"
                        >- September 2014 - December 2017 (3 years)</span
                    >
                </p>
                <p class="location">London, England, United Kingdom</p>
                <p class="project">
                    2015 - 2017: The Post Building, London, United Kingdom
                </p>
                <p class="project">
                    2014 - 2015: Camden Lock Village, London, United Kingdom
                </p>
            </div>
            <div class="company">
                <p class="company-name">
                    COOP HIMMELB(L)AU Wolf D. Prix & Partner
                </p>
                <p class="position">
                    Architectural Intern<span class="date">
                        - June 2014 - September 2014 (4 months)</span
                    >
                </p>
                <p class="location">Vienna, Austria</p>
                <p class="project">
                    New Parliamentary Complex for the Republic of Albania,
                    Tirana
                </p>
                <p class="project">
                    COOP HIMMELB(L)AU's Architectural Design Shape Archive
                </p>
            </div>
            <div class="company">
                <p class="company-name">March and White Design</p>
                <p class="position">
                    Architectural Assistant<span class="date"
                        >- May 2011 - September 2011 (5 months)</span
                    >
                </p>
                <p class="location">London, England, United Kingdom</p>
            </div>
            <div class="company">
                <p class="company-name">RLP Rudiger Lainer + Partner</p>
                <p class="position">
                    Architectural Intern<span class="date"
                        >- June 2010 - January 2011 (8 months)</span
                    >
                </p>
                <p class="location">Vienna, Austria</p>
            </div>
            <div class="company">
                <p class="company-name">Beijing Matsubara and Architects</p>
                <p class="position">
                    Architectural Assistant<span class="date"
                        >- June 2009 - September 2009 (4 months)</span
                    >
                </p>
                <p class="location">Beijing, China</p>
            </div>
        </div>
        <div class="section">
            <p class="section-title">EDUCATION</p>
            <div class="company">
                <p class="company-name">
                    Architectural Association School of Architecture
                </p>
                <span class="position">
                    ARB Part 3 Qualification, Architectural Professional
                    Practice (2016 - 2017)
                </span>
            </div>
            <div class="company">
                <p class="company-name">
                    Architectural Association School of Architecture
                </p>
                <p class="position">
                    ARB Part 2 Qualification, Diploma of Architecture (2011 -
                    2014)
                </p>
            </div>
            <div class="company">
                <p class="company-name">
                    The Glasgow School of Art (Mackintosh School of
                    Architecture)
                </p>
                <p class="position">
                    ARB Part 1 Qualification, Bachelor of Architecture (2007 -
                    2010)
                </p>
            </div>
        </div>
        <div class="section">
            <p class="section-title">WORKSHOP</p>
            <div class="company">
                <p class="workshop">
                    Architectural Association - MakeLab 2016 - Digital /
                    Physical Growth
                </p>
                <p class="workshop">
                    Robotic Architecture Workshop - Robofold 2011
                </p>
                <p class="workshop">Urban Computing - Ubiquitous Oulu 2010</p>

                <p class="workshop">
                    Engineer Without Borders - Solar Water Heater 2009
                </p>
                <p class="workshop">
                    Architects Sans Frontiers - Eden Project - Edible
                    Architecture 2009
                </p>
                <p class="workshop">
                    Architects Sans Frontiers - Building Communities, Salvador
                    da Bahia, Brazil 2009
                </p>
            </div>
        </div>
    </div>
</div>

<style>
    .container {
        padding: 2rem;
        display: grid;
        grid-template-columns: 1fr 3.14fr;
        gap: 1rem;
    }
    .avatar {
        margin-top: -1rem;
        width: 200px;
        object-fit: cover;
        border-radius: 50%;
        box-shadow: 3px 3px 6px rgba(0, 0, 0, 0.3);
    }
    .snippet {
        width: 200px;
        height: 200px;
        object-fit: cover;
        border-radius: 3rem;
    }
    .snippet-caption {
        font-size: 0.95rem;
        color: var(--foam);
        padding-bottom: 1rem;
    }

    .sidebar {
        /* border: 1px solid var(--muted); */
        border-radius: 0.5rem;
        padding: 0.5rem 1rem;
    }
    .sidebar-section {
        margin: 2rem 0 0 1rem;
    }
    .sidebar-content {
        font-size: 0.95rem;
        margin: 0.5rem 1rem;
        color: var(--foam);
    }
    .sidebar-title {
        font-weight: bold;
    }

    .contents {
        /* border: 1px solid var(--muted); */
        border-radius: 0.5rem;
        padding: 0.5rem 1rem;
        display: grid;
    }
    hr {
        margin: 2rem 5rem;
        border: 0px solid var(--muted);
        border-top: 1px solid var(--muted);
    }
    .image {
        width: 1.15rem;
        color: var(--text);
        object-fit: cover;
    }

    .name {
        font-size: 2rem;
    }
    .title {
        font-size: 1rem;
    }
    .address {
        padding-bottom: 1rem;
        font-size: 1rem;
        color: var(--subtle);
    }
    .contact {
        font-size: 1rem;
        color: var(--foam);
        margin-left: 1rem;
    }
    .contact > a {
        color: var(--foam);
    }

    .section-title {
        font-size: 1.25rem;
        font-weight: bold;
    }

    .subtitle {
        font-size: 1rem;
    }
    .subtitle-text {
        padding-left: 1.5rem;
        font-size: 0.95rem;
    }

    .company {
        padding: 0.5rem 1.5rem;
        margin-bottom: 1rem;
        background-color: var(--overlay);
        border-radius: 1rem;
    }
    .company-name {
        margin: 0.5rem;
        text-decoration: underline;
    }
    .company-name > a {
        color: var(--iris);
        font-weight: bold;
    }
    .position {
        margin: 0.5rem;
        font-size: 0.95rem;
        font-weight: bold;
    }
    .date {
        margin: 0.5rem;
        font-size: 0.95rem;
        color: var(--pine);
        font-weight: normal;
    }
    .location {
        margin: 0.5rem;
        font-size: 0.95rem;
        color: var(--subtle);
    }
    .project {
        margin: 0.5rem;
        font-size: 0.95rem;
        color: var(--iris);
    }
    .workshop {
        font-size: 0.95rem;
    }

    @media screen and (max-width: 1000px) {
        .container {
            grid-template-columns: 1fr;
            padding: 0;
        }
        hr {
            margin: 0;
        }
        .contents {
            border: 0px solid var(--muted);
        }
        .sidebar {
            display: none;
        }
    }
    @media screen and (min-width: 1000px) {
        .contacts {
            display: none;
        }
    }
</style>

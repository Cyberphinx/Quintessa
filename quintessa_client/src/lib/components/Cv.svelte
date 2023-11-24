<script lang="ts">
    import type { SignedMedia } from "$lib/model";
    import { cvMedia, resume } from "$lib/stores/commonStore";
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
        snippets[x.path] =
            `${$supabaseCredentials.url}/storage/v1${x.signedURL}`;
    });

    const captions = $resume && $resume.snippets.split(",");
</script>

{#if $resume}
    <div class="container">
        <div class="sidebar">
            <div class="sidebar-section">
                <img class="avatar" src={avatar} alt="profile" />
            </div>
            <div class="sidebar-section">
                <p class="sidebar-title">CONTACT</p>
                <p class="contact">
                    Email:
                    <a href="mailto:{$resume.email}">{$resume.email}</a>
                </p>
                <p class="contact">
                    Mobile: <a href="tel:00{$resume.mobile.replace(/\D/g, '')}"
                        >{$resume.mobile}</a
                    >
                </p>
                <p class="contact">
                    Website:
                    <a href="https://{$resume.website}" target="_blank"
                        >{$resume.website}</a
                    >
                </p>
            </div>
            <div class="sidebar-section">
                <p class="sidebar-title">DoB</p>
                <p class="sidebar-content">{$resume.birthdate}</p>
            </div>
            <div class="sidebar-section">
                <p class="sidebar-title">NATIONALITY</p>
                <p class="sidebar-content">{$resume.nationality}</p>
            </div>
            <div class="sidebar-section">
                <p class="sidebar-title">TOP SKILLS</p>
                {#each $resume.top_skills.split(",") as skill}
                    <p class="sidebar-content">{skill}</p>
                {/each}
            </div>
            <div class="sidebar-section">
                <p class="sidebar-title">LANGUAGES</p>
                {#each $resume.languages.split(",") as language}
                    <p class="sidebar-content">{language}</p>
                {/each}
            </div>
            <div class="sidebar-section">
                <p class="sidebar-title">CERTIFICATIONS</p>
                {#each $resume.certifications.split(",") as certification}
                    <p class="sidebar-content">{certification}</p>
                {/each}
            </div>
            {#if captions}
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
            {/if}
        </div>
        <div class="contents">
            <div class="section">
                <h1 class="name">{$resume.name}</h1>
                <p class="title">{$resume.job_title}</p>
                <p class="address">{$resume.address}</p>
                <div class="contacts">
                    <hr />
                    <p class="section-title">CONTACT</p>
                    <p class="contact">
                        Email:
                        <a href="mailto:{$resume.email}">{$resume.email}</a>
                    </p>
                    <p class="contact">
                        Mobile: <a
                            href="tel:00{$resume.mobile.replace(/\D/g, '')}"
                            >{$resume.mobile}</a
                        >
                    </p>
                </div>
            </div>
            <div class="section">
                <p class="section-title">SUMMARY</p>
                <p class="subtitle-text">
                    Multi-talented professional architect with 7 years of
                    experience in architectural design and engineering industry,
                    and 3 years of experience in full-stack web development.
                </p>
                <p class="subtitle">Architectural design skills:</p>
                <p class="subtitle-text">
                    Strong design ability, with 3D modeling, architectural
                    detailing, technical drafting, design coordination and
                    presentation skills, along with specialism in BIM (Revit),
                    NBS Create specification, Microsoft packages and Indesign
                    for documentation and presentation. Proven track record of
                    working on large and complex residential, commercial and
                    industrial projects in London, ranging from Concept,
                    Technical Design to Production Information and Construction,
                    for packages including interiors, facade, metalwork,
                    staircase, roof, partitions, landscaping, and robotic
                    warehouse, etc.
                </p>
                <p class="subtitle">Full-stack web development skills:</p>
                <p class="subtitle-text">
                    Experience in creating full-stack web application from
                    scratch for real estate web portals, with skill-sets ranging
                    from responsive UI design, front-end and back-end
                    development to database integration and GitHub, Jira and
                    Kanban collaborative workflow, mainly focusing on Typescript
                    as front-end, and Rust as back-end languages, with Docker
                    deployment knowledge on cloud platforms, along with the
                    ability to write simple unit-tests. Technical specialism in
                    web scraping and data mining using browser-automation, such
                    as puppeteer and headless chromium. Practical experience
                    setting up authentication middle-ware using JWT and auth0.
                    Understanding the importance of clarity, readability and
                    maintainability in code, following best practice principles,
                    such as the Clean Architecture and Facade pattern.
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
                    Adapative quick learner, always open and passionate to learn
                    new technologies. Effiicent time management, effective
                    team-working, and project management with respect to
                    deadlines, as well as effective communication, collaboration
                    and coordination skills between internal and external team
                    members. Proven ability of working under stress, working
                    well both within a team and individually. Project
                    presentation skills to clients and contractors. Professional
                    translation skills between English and Mandarin Chinese.
                </p>
                <br />
            </div>
            <div class="section">
                <p class="section-title">WORK</p>
                {#each $resume.works as work}
                    <div class="company">
                        <p class="company-name">
                            {work.company_name}
                        </p>
                        <p class="position">
                            {work.position}
                            <span class="date">- {work.duration}</span>
                        </p>
                        <p class="location">
                            {work.location}
                        </p>
                        {#each work.projects.split(";") as project}
                            <p class="project">
                                {project}
                            </p>{/each}
                    </div>
                {/each}
            </div>
            <div class="section">
                <p class="section-title">EDUCATION</p>
                {#each $resume.educations as education}
                    <div class="company">
                        <p class="company-name">
                            {education.school}
                        </p>
                        <span class="position">{education.degree}</span>
                    </div>
                {/each}
            </div>
            <div class="section">
                <p class="section-title">WORKSHOP</p>
                <div class="company">
                    {#each $resume.workshops.split(",") as workshop}
                        <p class="workshop">{workshop}</p>
                    {/each}
                </div>
            </div>
        </div>
    </div>
{/if}

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

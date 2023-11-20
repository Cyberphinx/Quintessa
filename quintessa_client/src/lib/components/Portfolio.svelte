<script lang="ts">
    import { goto } from "$app/navigation";
    import { page } from "$app/stores";
    import { loading, portfolio, projects } from "$lib/stores/commonStore";
    import { device } from "$lib/utilities/device";

    let images: any = {};
    let projectIndex = 0;
    let mediaIndex = 0;

    let categories = [
        ["Architecture", "Architecture"],
        ["Academic", "Academic"],
        ["Web", "Web Development"],
        ["Drawing", "Art"],
    ];

    async function handlePrevProject() {
        if (projectIndex > 0) projectIndex -= 1;
        if (mediaIndex !== 0) mediaIndex = 0;
    }
    async function handleNextProject() {
        if (projectIndex < $projects.length - 1) projectIndex += 1;
        if (mediaIndex !== 0) mediaIndex = 0;
    }

    async function handleCategory(value: string) {
        loading.set(true);
        $page.url.searchParams.delete("category");
        projects.set([]);
        const newUrl = new URL($page.url);
        if (value) newUrl.searchParams.set("category", value);
        else newUrl.searchParams.delete("category");
        goto(newUrl).then(async () => {
            loading.set(false);
            $portfolio && $portfolio.scrollIntoView();
        });
    }

    async function resetCategory() {
        loading.set(true);
        goto("/").then(async () => {
            loading.set(false);
        });
    }
</script>

<div class="container">
    <div class="categories">
        <button
            style={!$page.url.searchParams.get("category")
                ? "background: var(--foam); color: var(--base); border: 1px solid var(--base);"
                : ""}
            on:click={() => resetCategory()}>All</button
        >
        {#each categories as category}
            <button
                style={$page.url.searchParams.get("category") === category[0]
                    ? "background: var(--foam); color: var(--base); border: 1px solid var(--base);"
                    : ""}
                on:click={() => handleCategory(category[0])}
                >{category[1]}</button
            >{/each}
    </div>

    <div class="projects-nav">
        <button
            on:click={() => handlePrevProject()}
            disabled={projectIndex === 0}
            >Previous <span class="extra-text">project</span></button
        >
        <p class="projects-index">
            {projectIndex + 1} of {$projects.length}
            <span class="extra-text">projects</span>
        </p>
        <button
            on:click={() => handleNextProject()}
            disabled={projectIndex + 1 === $projects.length}
            >Next <span class="extra-text">project</span></button
        >
    </div>

    <div class="projects">
        <div class="contents">
            {#if $projects[projectIndex].media.length > 0 && images}
                {#if $device !== "mobile"}<div>
                        <img
                            class="image"
                            src={$projects[projectIndex].media[mediaIndex].url}
                            alt="project"
                        />
                        <p class="caption">
                            {mediaIndex + 1} of {$projects[projectIndex].media
                                .length}
                        </p>
                        <p class="caption">
                            {$projects[projectIndex].media[mediaIndex].caption}
                        </p>
                    </div>
                {/if}
                <div class="thumbnails">
                    {#each $projects[projectIndex].media.sort((a, b) => a.id - b.id) as projectMedia, i}
                        <button
                            class="thumbnail-button"
                            on:click={() => (mediaIndex = i)}
                        >
                            <img
                                class="thumbnail"
                                src={projectMedia.url
                                    ? projectMedia.url
                                    : "/assets/Placeholder.svg"}
                                alt={projectMedia.caption
                                    ? projectMedia.caption
                                    : "Project Media"}
                            />
                            <span class="index">
                                {i + 1} of {$projects[projectIndex].media
                                    .length}
                            </span>
                        </button>
                    {/each}
                </div>
            {/if}
        </div>
        <div class="sidebar">
            <h2>{$projects[projectIndex].name}</h2>
            <p class="description">{$projects[projectIndex].description}</p>
            {#if $projects[projectIndex].category === "Architecture"}
                <p class="description">
                    My work scope include but not limited to the following
                    tasks:
                </p>
                <ol>
                    {#each $projects[projectIndex].tasks.split(";") as task}<li
                            class="task"
                        >
                            {task}
                        </li>{/each}
                </ol>
                <br />
                <table>
                    <tr><th colspan="2">Summary</th></tr>
                    <tr
                        ><td>Sector</td><td>{$projects[projectIndex].sector}</td
                        ></tr
                    >
                    <tr
                        ><td>Location</td><td
                            >{$projects[projectIndex].location}</td
                        ></tr
                    >
                    <tr
                        ><td>Address</td><td
                            >{$projects[projectIndex].address}</td
                        ></tr
                    >
                    <tr
                        ><td>Client</td><td>{$projects[projectIndex].client}</td
                        ></tr
                    >
                    <tr
                        ><td>Start</td><td
                            >{$projects[projectIndex].start_date}</td
                        ></tr
                    >
                    <tr
                        ><td>Completion</td><td
                            >{$projects[projectIndex].completion_date}</td
                        ></tr
                    >
                </table>
                <br />
                <table>
                    <tr><th colspan="2">Project Team</th></tr>
                    <tr
                        ><td>Architect</td><td
                            >{$projects[projectIndex].architect}</td
                        ></tr
                    >
                    <tr
                        ><td>Main contractor</td><td
                            >{$projects[projectIndex].main_contractor}</td
                        ></tr
                    >
                    <tr
                        ><td>Project manager</td><td
                            >{$projects[projectIndex].project_manager}</td
                        ></tr
                    >
                    <tr
                        ><td>Structural engineer</td><td
                            >{$projects[projectIndex].structural_engineer}</td
                        ></tr
                    >
                    <tr
                        ><td>Services engineer</td><td
                            >{$projects[projectIndex].services_engineer}</td
                        ></tr
                    >
                </table>{/if}
        </div>
    </div>
</div>

<style>
    .container {
        padding: 1rem;
    }
    button {
        color: var(--text);
        border: 1px solid var(--muted);
        border-radius: 0.5rem;
        padding: 0.5rem 1rem;
    }
    .categories {
        display: flex;
        gap: 1rem;
        flex-wrap: wrap;
        justify-content: center;
    }
    .projects {
        padding: 1rem 0;
        display: grid;
        grid-template-columns: 1.25fr 3fr;
        gap: 1rem;
    }
    .sidebar {
        border-radius: 0.5rem;
        padding: 0.5rem 1rem;
    }

    .contents {
        border-radius: 0.5rem;
        padding: 0.5rem 1rem;
        text-align: center;
    }
    h2 {
        font-size: 1.5rem;
    }

    .projects-nav {
        padding: 2rem 1rem 0 1rem;
        margin: 0;
        display: flex;
        justify-content: space-between;
        align-items: center;
    }
    .projects-nav > button:disabled {
        cursor: default;
        color: var(--muted);
        border: 1px solid var(--highlight-med);
    }
    .projects-index {
        font-size: 1rem;
        margin: 0;
        text-align: center;
        color: var(--foam);
        border: 1px solid var(--muted);
        border-radius: 0.5rem;
        padding: 0.5rem 1rem;
    }

    .description {
        font-size: 0.95rem;
        line-height: 1.5rem;
    }
    .task {
        font-size: 0.95rem;
    }

    table {
        border-collapse: collapse;
        width: 350px;
    }
    th {
        border: 1px solid var(--muted);
        font-size: 0.85rem;
        background-color: var(--overlay);
        text-align: left;
        padding: 0.5rem 1rem;
    }
    td {
        margin: 0;
        border: 1px solid var(--muted);
        padding: 0.5rem 1rem;
        font-size: 0.85rem;
    }

    .image {
        margin-top: 1rem;
        width: 800px;
        height: 600px;
        border-radius: 1rem;
        object-fit: contain;
    }
    .caption {
        color: var(--foam);
    }

    .thumbnails {
        padding-top: 1rem;
        display: flex;
        flex-wrap: wrap;
        justify-content: center;
        gap: 0.5rem;
    }
    .thumbnail {
        width: 150px;
        height: 150px;
        object-fit: cover;
        border-radius: 1rem;
        box-shadow: 2px 2px 3px rgba(0, 0, 0, 0.3);
    }
    .thumbnail-button {
        margin: 0;
        padding: 0.5rem;
        display: grid;
        gap: 0.5rem;
        border: none;
        border-radius: 1rem;
    }
    .thumbnail-button:hover {
        background-color: var(--highlight-med);
    }
    .index {
        font-size: 0.85rem;
        margin: 0;
        color: var(--foam);
    }

    @media screen and (max-width: 1800px) {
        .projects {
            grid-template-columns: 1fr;
        }
        .image {
            height: 600px;
            width: 800px;
        }
    }
    @media screen and (max-width: 1280px) {
        .projects {
            grid-template-columns: 1fr;
        }
        .image {
            height: 600px;
            width: 800px;
        }
    }
    @media screen and (max-width: 1000px) {
        .projects {
            grid-template-columns: 1fr;
        }
        .image {
            height: 400px;
            width: 600px;
        }
    }
    @media screen and (max-width: 700px) {
        .projects {
            grid-template-columns: 1fr;
        }
        .image {
            margin: 0;
            padding: 0;
            height: 450px;
            width: 450px;
            border-radius: 1rem;
        }
        .extra-text {
            display: none;
        }
        .projects-index {
            border: none;
        }
    }
    @media screen and (max-width: 600px) {
        .projects {
            grid-template-columns: 1fr;
        }
        .image {
            margin: 0;
            padding: 0;
            height: 400px;
            width: 400px;
            border-radius: 1rem;
        }
        .extra-text {
            display: none;
        }
        .projects-index {
            border: none;
        }
    }
    @media screen and (max-width: 500px) {
        .container {
            padding: 1rem 0;
        }
        .thumbnail {
            width: 220px;
            height: 200px;
        }
        .image {
            display: none;
        }
        .caption {
            display: none;
        }
        table {
            width: 75vw;
        }
    }
</style>

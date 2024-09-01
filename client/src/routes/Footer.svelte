<script>
    import { page } from "$app/stores";
</script>

<footer>
    <nav>
        <ul>
            <li aria-current={$page.url.pathname === "/" ? "page" : undefined}>
                <a href="/">Home</a>
            </li>
            <li aria-current={$page.url.pathname === "/about" ? "page" : undefined}>
                <a href="/about">About</a>
            </li>
            <li aria-current={$page.url.pathname.startsWith("/system") ? "page" : undefined}>
                <a href="/system">System</a>
            </li>
            <li aria-current={$page.url.pathname.startsWith("/tnf") ? "page" : undefined}>
                <a href="/tnf">Track Things</a>
            </li>
        </ul>
    </nav>
</footer>

<style>
    :global(body, html) {
        margin: 0;
        padding: 0;
        height: 100%;
        display: flex;
        flex-direction: column;
    }

    footer {
        margin-top: auto; /* Push footer to bottom */
        background: #f9f9f9;
        padding: 1rem;
        border-top: 1px solid #dddddd;
    }

    nav {
        display: flex;
        justify-content: center;
        align-items: center;
        gap: 1rem;
    }

    svg {
        width: 24px;
        height: auto;
        fill: #009879; /* Change as needed */
    }

    ul {
        list-style: none;
        padding: 0;
        display: flex;
        gap: 1rem;
    }

    li {
        position: relative;
    }

    a {
        text-decoration: none;
        color: #333;
        font-weight: 500;
    }

    li[aria-current="page"] a {
        color: #009879; /* Highlight current page */
    }
</style>
